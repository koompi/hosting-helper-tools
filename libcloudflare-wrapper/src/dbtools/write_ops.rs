use super::{open_database, params};

pub(crate) fn insert_tbl_cloudflare_pending(
    server_name: &str,
    newdns: &str,
    olddns: Option<String>,
    registra: Option<&str>,
    last_check: &str,
) {
    open_database()
        .execute(
            &format!(
                "INSERT INTO tblCloudflarePending(ServerName, NewDNS, OldDNS, Registra, LastCheck) 
                VALUES(?1, json(?2), json(?3), ?4, ?5);"
            ),
            params![server_name, newdns, olddns, registra, last_check],
        )
        .unwrap();
}

pub(crate) fn insert_tbl_cloudflare_data(server_name: &str, zone_id: &str) {
    open_database()
        .execute(
            &format!("INSERT INTO tblCloudflareData(ServerName, ZoneID) VALUES(?1, ?2);"),
            params![server_name, zone_id],
        )
        .unwrap();
}

pub(crate) fn update_pending_tbl(
    server_name: &str,
    newdns: &str,
    olddns: Option<String>,
    registra: Option<&str>,
    new_time: &str,
) {
    open_database()
        .execute(
            &format!(
                "UPDATE tblCloudflarePending 
            SET NewDNS = json(?2), 
            OldDNS = json(?3), 
            Registra = ?4, 
            LastCheck = ?5 
            WHERE ServerName = ?1;"
            ),
            params![server_name, newdns, olddns, registra, new_time],
        )
        .unwrap();
}

pub(crate) fn _delete_from_tbl(server_name: &str, pendingtbl: bool, ) {
    open_database()
        .execute(
            &format!("DELETE FROM ?1 WHERE ServerName = ?2;"),
            params![
                match pendingtbl {
                    true => "tblCloudflarePending",
                    false => "tblCloudflareData",
                },
                server_name
            ],
        )
        .unwrap();
}

pub(crate) fn delete_from_tblcloudflarepending(server_name: &str) {
    open_database()
        .execute(
            &format!("DELETE FROM tblCloudflarePending WHERE ServerName = ?1;"),
            params![
                server_name
            ],
        )
        .unwrap();
}
