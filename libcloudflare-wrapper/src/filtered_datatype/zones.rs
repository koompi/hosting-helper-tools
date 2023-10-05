#![allow(dead_code, unused_variables)]

use super::{dbtools, NaiveTime, ObjResponse, ObjResult, Utc};

#[derive(Debug)]
pub struct CloudflareData {
    server_name: String,
    zone_id: String,
}

impl CloudflareData {
    pub fn new(server_name: String, zone_id: String) -> Self {
        Self {
            server_name,
            zone_id,
        }
    }

    pub fn get_server_name(&self) -> &str {
        &self.server_name
    }
    pub fn get_zone_id(&self) -> &str {
        &self.zone_id
    }
}

#[derive(Debug)]
pub struct CloudflarePending {
    server_name: String,
    new_dns: Vec<String>,
    old_dns: Option<Vec<String>>,
    registra: Option<String>,
    last_check: NaiveTime,
}

impl CloudflarePending {
    pub fn new(
        server_name: String,
        new_dns: Vec<String>,
        old_dns: Option<Vec<String>>,
        registra: Option<String>,
        last_check: NaiveTime,
    ) -> Self {
        Self {
            server_name,
            new_dns,
            old_dns,
            registra,
            last_check,
        }
    }

    pub fn get_server_name(&self) -> &str {
        &self.server_name
    }
    pub fn get_new_dns(&self) -> &Vec<String> {
        &self.new_dns
    }
    pub fn get_old_dns(&self) -> &Option<Vec<String>> {
        &self.old_dns
    }
    pub fn get_registra(&self) -> &Option<String> {
        &self.registra
    }
    pub fn get_last_check(&self) -> NaiveTime {
        self.last_check
    }

    pub fn is_expired(&self) -> bool {
        let last_check = self.get_last_check();
        let now = Utc::now().time();

        let duration = now - last_check;
        duration.num_minutes() >= dotenv::var("PENDING_CHECK_LIMIT").unwrap().parse().unwrap()
    }

    pub fn recheck_pending_status(&self) -> Result<Option<Self>, (u16, String)> {
        let res = ObjResponse::get_zone(Some(&self.get_server_name()), true);
        res.unwrap()?;
        match res.is_empty() {
            true => {
                let data = match res.result.unwrap() {
                    ObjResult::ZonesData(data) => data.into_iter().next().unwrap(),
                    _ => unreachable!()
                    // ObjResult::DNSRecords(_) => unreachable!(),
                    // ObjResult::ZoneData(_) => unreachable!(),
                    // ObjResult::DNSRecord(_) => unreachable!(),
                };
                let newdns = data.name_servers.join("\n");
                let olddns = match data.original_name_servers.as_ref() {
                    Some(nameserver) => Some(nameserver.join("\n")),
                    None => None,
                };
                let registra = data.original_registrar;
                let new_time = Utc::now().time().to_string();
                dbtools::write_ops::update_pending_tbl(
                    &self.get_server_name(),
                    &newdns,
                    olddns,
                    registra.as_deref(),
                    &new_time,
                );
                Ok(Some(dbtools::read_ops::query_from_tbl_cloudflare_pending(&self.get_server_name()).unwrap()))
            }
            false => {
                dbtools::write_ops::delete_from_tbl(&self.get_server_name(), true);
                Ok(None)
            }
        }
    }
}
