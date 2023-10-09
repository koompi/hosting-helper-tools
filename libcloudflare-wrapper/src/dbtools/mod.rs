use super::{
    cloudflare_datatype::{ObjResponse, ObjResult},
    filtered_datatype::zones::{CloudflareData, CloudflarePending},
    DateTime, Utc,
};
use libdatabase::{open_database, params, DBClient};

pub mod read_ops;
pub mod write_ops;

pub(crate) async fn db_migration(force: bool) -> Result<(), (u16, String)> {
    if let Some(_) = libdatabase::db_migration(
        DBClient::LibCloudflare,
        match force {
            true => Some(DBClient::LibCloudflare),
            false => None,
        },
    ) {
        let response = ObjResponse::get_zone(None, false).await;
        response.unwrap()?;

        if !response.is_empty() {
            if let ObjResult::ZonesData(zones) = response.result.unwrap() {
                zones
                    .iter()
                    .for_each(|each_zone| match each_zone.status == "pending" {
                        true => write_ops::insert_tbl_cloudflare_pending(
                            &each_zone.name,
                            serde_json::json!(each_zone.name_servers)
                                .to_string()
                                .as_str(),
                            each_zone
                                .original_name_servers
                                .as_ref()
                                .and_then(|nameservers| {
                                    Some(serde_json::json!(nameservers).to_string())
                                }),
                            each_zone
                                .original_registrar
                                .as_deref()
                                .and_then(|data| data.split(",").next()),
                            Utc::now().to_rfc3339().as_str(),
                        ),
                        false => {
                            write_ops::insert_tbl_cloudflare_data(&each_zone.name, &each_zone.id)
                        }
                    })
            }
        }
    }

    Ok(())
}
