// use crate::cloudflare_datatype::records;

use super::{Client, HeaderMap, ObjResponse, ObjResult};

impl ObjResponse {
    pub async fn del_record(
        client: &Client,
        headers: &HeaderMap,
        full_domain_name: &str,
        // zone_id: &str,
    ) -> Result<(), (u16, String)> {
        // let client = Self::get_client();
        // let header = Self::get_headers();
        let tldextracted_domain = tldextract::TldOption::default()
            .build()
            .extract(full_domain_name)
            .unwrap();
        let domain_tld = tldextracted_domain.domain.unwrap()
            + "."
            + tldextracted_domain.suffix.as_ref().unwrap();
        let data = match crate::dbtools::read_ops::query_from_tbl_cloudflare_data(&domain_tld) {
            Some(data) => Ok(data),
            None => Err((500, String::from("Doesn't exist!"))),
        }?;
        let zone_id = data.get_zone_id();
        // let id = data.get_zone_id();
        let response = Self::get_records(&client, &headers, zone_id, Some(full_domain_name)).await;
        match response.unwrap() {
            Ok(()) => Ok(()),
            Err((code, message)) => Err((code, message)),
        }?;

        let records = match response.result {
            Some(data) => match data {
                ObjResult::DNSRecords(records) => match !records.is_empty() {
                    true => Ok(records),
                    false => Err((500, String::from("Vec Empty")))
                },
                _ => Err((500, String::from("Unexpected Response"))),
            },
            None => Err((500, String::from("Item doesn't exist!"))),
        }?;
        let record_id = records.into_iter().next().unwrap().id;
        let request = client
            .delete(format!(
                "https://api.cloudflare.com/client/v4/zones/{zone_id}/dns_records/{record_id}"
            ))
            .headers(headers.clone());

        let response = request.send().await.unwrap();
        match response.status() {
            reqwest::StatusCode::OK => Ok(()),
            _ => Err((500, String::from("Delete has failed. please try again.."))),
        }
    }
}
