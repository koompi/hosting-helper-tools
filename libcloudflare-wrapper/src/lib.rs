mod cloudflare_datatype;
mod dbtools;
mod filtered_datatype;

use chrono::{DateTime, NaiveTime, Utc};
use cloudflare_datatype::{ObjResponse, ObjResult};
use filtered_datatype::zones::CloudflarePending;
use tldextract::TldOption;
use reqwest::{Client, header::HeaderMap, Method};

pub async fn db_migration(force: bool) -> Result<(), (u16, String)> {
    dbtools::db_migration(force).await
}

pub fn get_client() -> Client {
    ObjResponse::get_client()
}

pub fn _get_headers() -> HeaderMap {
    ObjResponse::get_headers()
}

pub async fn get_public_ip(client: &Client, domain: Option<&str>) -> String {
    match domain {
        Some(domain) => match serde_json::from_str::<serde_json::Value>(
            &client
                .request(
                    Method::GET,
                    format!("https://1.1.1.1/dns-query?name={}", domain),
                )
                .header("accept", "application/dns-json")
                .send()
                .await
                .unwrap()
                .text()
                .await
                .unwrap(),
        )
        .unwrap()
        .get("Answer")
        {
            Some(data) => data
                .as_array()
                .unwrap()
                .iter()
                .next()
                .unwrap()
                .get("data")
                .unwrap()
                .as_str()
                .unwrap()
                .to_string(),
            None => String::new(),
        },
        None => client
            .request(Method::GET, "https://ip.me")
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap()
            .trim()
            .to_string(),
    }
}

pub async fn setup_domain(full_domain_name: &str) -> Result<(), (u16, String)> {
    let client = ObjResponse::get_client();
    let headers = ObjResponse::get_headers();
    let tldextracted_domain = TldOption::default()
        .build()
        .extract(full_domain_name)
        .unwrap();
    let public_ip = get_public_ip(&client, None).await;
    let subdomain = match tldextracted_domain.subdomain {
        Some(subdomain) => subdomain,
        None => String::from("@"),
    };
    let domain_tld =
        tldextracted_domain.domain.unwrap() + "." + tldextracted_domain.suffix.as_ref().unwrap();


    match dbtools::read_ops::query_from_tbl_cloudflare_pending(&domain_tld) {
        Some(domain) => match domain.is_expired() {
            true => match domain.recheck_pending_status(&client, &headers).await {
                Ok(check) => match check {
                    None => Ok(()),
                    Some(data) => Err((
                        503,
                        format!(
                            "\n{}\nfor your domain has not yet been directed to our server",
                            CloudflarePending::format_dns_vec(data.get_new_dns())
                        ),
                    )),
                },
                Err(err) => Err(err),
            },
            false => Err((
                503,
                format!(
                    "\n{}\nfor your domain has not yet been configured yet",
                    CloudflarePending::format_dns_vec(domain.get_new_dns())
                ),
            )),
        },
        None => Ok(()),
    }?;

    let result_response: ObjResponse =
        match dbtools::read_ops::query_from_tbl_cloudflare_data(&domain_tld) {
            Some(data) => {
                let response = ObjResponse::get_records(
                    &client,
                    &headers,
                    data.get_zone_id(),
                    Some(full_domain_name),
                )
                .await;
                response.unwrap()?;
                match response.is_empty() {
                    true => {
                        ObjResponse::post_record(
                            &client,
                            &headers,
                            &subdomain,
                            &public_ip,
                            data.get_zone_id(),
                        )
                        .await
                    }
                    false => match response.result.unwrap() {
                        ObjResult::DNSRecords(t) => {
                            let record = t
                                .iter()
                                .filter(|item| (item.r#type == "CNAME" || item.r#type == "A"))
                                .next()
                                .unwrap();
                            let actual_ip = match record.r#type.as_str() == "CNAME" {
                                true => {
                                    let a_response = ObjResponse::get_records(
                                        &client,
                                        &headers,
                                        data.get_zone_id(),
                                        Some(&record.content),
                                    )
                                    .await;
                                    a_response.unwrap()?;
                                    match a_response.result.unwrap() {
                                        ObjResult::DNSRecords(result) => {
                                            result.into_iter().next().unwrap().content
                                        }
                                        _ => unreachable!(),
                                    }
                                }
                                false => record.content.to_owned(),
                            };

                            if &public_ip != &actual_ip {
                                ObjResponse::put_record(
                                    &client,
                                    &headers,
                                    &subdomain,
                                    &public_ip,
                                    data.get_zone_id(),
                                    &record.id,
                                )
                                .await
                            } else {
                                ObjResponse::default()
                            }
                        }
                        _ => unreachable!(),
                    },
                }
            }
            None => ObjResponse::post_zone(&client, &headers, &domain_tld).await,
        };

    match result_response.is_empty() {
        true => Ok(()),
        false => {
            match result_response.result.unwrap() {
                ObjResult::ZoneData(zone) => Err({
                    let server_name = zone.name;
                    let newdns = serde_json::json!(zone.name_servers).to_string();
                    let olddns = zone
                        .original_name_servers
                        .as_ref()
                        .and_then(|nameservers| Some(serde_json::json!(nameservers).to_string()));

                    let error_message = match zone.original_name_servers.as_ref() {
                        Some(dns) => format!(
                            "Please remove these \n{} at your Registra {} \nand replace with these \n{}",
                            dns.join("\n"),
                            zone.original_registrar.as_ref().unwrap_or(&String::new()),
                            &zone.name_servers.join("\n")
                        ),
                        None => format!(
                            "Please replace your Nameserver at your Registra {} with \n{}", 
                            &zone.original_registrar.as_ref().unwrap_or(&String::new()), &zone.name_servers.join("\n")
                        )
                    };
                    let registra = zone
                        .original_registrar
                        .as_deref()
                        .and_then(|data| data.split(",").next());

                    let last_check = chrono::Utc::now().to_rfc3339();
                    dbtools::write_ops::insert_tbl_cloudflare_pending(
                        &server_name,
                        &newdns,
                        olddns,
                        registra,
                        &last_check,
                    );

                    (503, error_message)
                }),
                ObjResult::DNSRecord(_) => {
                    Ok(std::thread::sleep(std::time::Duration::from_secs(5)))
                }
                _ => unreachable!(),
            }
        }
    }?;

    Ok(())
}
