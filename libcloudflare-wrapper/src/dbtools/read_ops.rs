use super::{open_database, params, CloudflareData, CloudflarePending, DateTime};
pub fn query_from_tbl_cloudflare_pending(server_name: &str) -> Option<CloudflarePending> {
    open_database()
        .query_row(
            "SELECT ServerName,NewDNS,OldDNS,Registra,LastCheck 
            FROM tblCloudflarePending WHERE ServerName = ?1",
            params![server_name],
            |data| {
                Ok(CloudflarePending::new(
                    data.get::<usize, String>(0).unwrap(),
                    serde_json::from_str::<Vec<String>>(
                        data.get::<usize, String>(1).unwrap().as_str(),
                    )
                    .unwrap(),
                    serde_json::from_str::<Option<Vec<String>>>(
                        data.get::<usize, String>(2).unwrap().as_str(),
                    )
                    .unwrap(),
                    data.get::<usize, Option<String>>(3).unwrap(),
                    DateTime::parse_from_rfc3339(data.get::<usize, String>(4).unwrap().as_str())
                        .unwrap()
                        .time(),
                ))
            },
        )
        .ok()
}

pub fn query_from_tbl_cloudflare_data(server_name: &str) -> Option<CloudflareData> {
    open_database()
        .query_row(
            "SELECT ServerName,ZoneID
            FROM tblCloudflareData WHERE ServerName = ?1",
            params![server_name],
            |data| Ok(CloudflareData::new(data.get_unwrap(0), data.get_unwrap(1))),
        )
        .ok()
    // None
}
