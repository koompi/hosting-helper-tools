mod cloudflare_datatype;
mod dbtools;
mod filtered_datatype;

use chrono::{DateTime, NaiveTime, Utc};
use cloudflare_datatype::{ObjResponse, ObjResult};
use filtered_datatype::zones::CloudflarePending;
use tldextract::TldOption;

pub fn db_migration(force: bool) -> Result<(), (u16, String)> {
    dbtools::db_migration(force)
}

pub fn setup_domain(full_domain_name: &str) -> Result<(), (u16, String)> {
    let tldextracted_domain = TldOption::default()
        .build()
        .extract(full_domain_name)
        .unwrap();
    let public_ip = reqwest::blocking::Client::builder()
        .local_address("0.0.0.0".parse::<std::net::IpAddr>().unwrap())
        .build()
        .unwrap()
        .request(reqwest::Method::GET, "https://ip.me")
        .send()
        .unwrap()
        .text()
        .unwrap();
    let public_ip = public_ip.trim_end();
    let subdomain = match tldextracted_domain.subdomain {
        Some(subdomain) => subdomain,
        None => String::from("@"),
    };
    let domain_tld =
        tldextracted_domain.domain.unwrap() + "." + tldextracted_domain.suffix.as_ref().unwrap();

    match dbtools::read_ops::query_from_tbl_cloudflare_pending(&domain_tld) {
        Some(domain) => match domain.is_expired() {
            true => match domain.recheck_pending_status() {
                Ok(check) => match check {
                    None => Ok(()),
                    Some(data) => Err((
                        503,
                        format!(
                            "\n{}\nfor your domain has not yet been directed to our server", 
                            CloudflarePending::format_dns_vec(data.get_new_dns())
                            // data.get_new_dns().iter().map(|each| format!("\t- {}", each)).collect::<Vec<String>>().join("\n")
                        )
                    )),
                },
                Err(err) => Err(err),
            },
            false => Err((
                503,
                format!(
                    "\n{}\nfor your domain has not yet been configured yet", 
                    // domain.get_new_dns().iter().map(|each| format!("\t- {}", each)).collect::<Vec<String>>().join("\n")
                    CloudflarePending::format_dns_vec(domain.get_new_dns())
                )
            )),
        },
        None => Ok(()),
    }?;

    let result_response: ObjResponse =
        match dbtools::read_ops::query_from_tbl_cloudflare_data(&domain_tld) {
            Some(data) => {
                let response = ObjResponse::get_records(data.get_zone_id(), Some(full_domain_name));
                response.unwrap()?;
                match response.is_empty() {
                    true => ObjResponse::post_record(&subdomain, public_ip, data.get_zone_id()),
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
                                        data.get_zone_id(),
                                        Some(&record.content),
                                    );
                                    a_response.unwrap()?;
                                    match a_response.result.unwrap() {
                                        ObjResult::DNSRecords(result) => {
                                            result.into_iter().next().unwrap().content
                                        }
                                        _ => unreachable!(),
                                        // ObjResult::ZonesData(_) => unreachable!(),
                                        // ObjResult::ZoneData(_) => unreachable!(),
                                        // ObjResult::DNSRecord(_) => unreachable!(),
                                    }
                                }
                                false => record.content.to_owned(),
                            };

                            if public_ip != &actual_ip {
                                ObjResponse::put_record(
                                    &subdomain,
                                    public_ip,
                                    data.get_zone_id(),
                                    &record.id,
                                )
                            } else {
                                ObjResponse::default()
                            }
                        }
                        _ => unreachable!(),
                        // ObjResult::ZoneData(_) => unreachable!(),
                        // ObjResult::ZonesData(_) => unreachable!(),
                        // ObjResult::DNSRecord(_) => unreachable!(),
                    },
                }
            }
            None => ObjResponse::post_zone(&domain_tld),
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

                    (
                        503,
                        error_message, 
                    )
                }),
                ObjResult::DNSRecord(_) => {
                    Ok(std::thread::sleep(std::time::Duration::from_secs(5)))
                }
                _ => unreachable!(),
                // ObjResult::DNSRecords(_) => todo!(),
                // ObjResult::None => todo!(),
                // ObjResult::ZonesData(_) => todo!(),
            }
        }
    }?;

    Ok(())
}
