use std::str::FromStr;

use super::{open_database, params, NginxFeatures, NginxObj};

pub(crate) fn create_tables() {
    open_database()
        .execute_batch(
            "BEGIN;
CREATE TABLE tblNginxConf(
    ServerName VARCHAR(100) NOT NULL UNIQUE PRIMARY KEY,
    ProxyPass VARCHAR(100) NOT NULL NOT NULL,
    Feature VARCHAR(100) NOT NULL
);
COMMIT;",
        )
        .unwrap();
}

pub(crate) fn query_existence_from_tbl_nginxconf(server_name: &str) -> bool {
    let connection = super::open_database();

    let mut stmt = connection
        .prepare("SELECT EXISTS(SELECT ServerName FROM tblNginxConf WHERE ServerName=? LIMIT 1);")
        .unwrap();
    let mut rows = stmt.query(&[server_name]).unwrap();

    rows.next().unwrap().unwrap().get::<usize, u64>(0).unwrap() != 0
}

pub(crate) fn select_one_from_tbl_nginxconf(server_name: &str) -> NginxObj {
    open_database()
        .prepare("SELECT ServerName,ProxyPass,Feature FROM tblNginxConf WHERE ServerName = ?1")
        .unwrap()
        .query_row([server_name], |each_row| {
            Ok(NginxObj::new(
                each_row.get::<usize, String>(0).unwrap(),
                each_row.get::<usize, String>(1).unwrap(),
                NginxFeatures::from_str(each_row.get::<usize, String>(2).unwrap().as_str())
                    .unwrap(),
            ))
        })
        .unwrap()
}

pub fn select_all_from_tbl_nginxconf() -> Vec<NginxObj> {
    open_database()
        .prepare("SELECT ServerName,ProxyPass,Feature FROM tblNginxConf")
        .unwrap()
        .query_map([], |each_row| {
            Ok(NginxObj::new(
                each_row.get::<usize, String>(0).unwrap(),
                each_row.get::<usize, String>(1).unwrap(),
                NginxFeatures::from_str(each_row.get::<usize, String>(2).unwrap().as_str())
                    .unwrap(),
            ))
        })
        .unwrap()
        .map(|each| each.unwrap())
        .collect::<Vec<NginxObj>>()
}

pub(crate) fn insert_tbl_nginxconf(server_name: &str, proxy_pass: &str, feature: &str) {
    open_database()
        .execute(
            "INSERT INTO tblNginxConf(ServerName, ProxyPass, Feature) VALUES(?1, ?2, ?3);",
            params![server_name, proxy_pass, feature],
        )
        .unwrap();
}

pub(crate) fn delete_from_tbl_nginxconf(server_name: &str) {
    open_database()
        .execute(
            "DELETE FROM tblNginxConf WHERE ServerName = ?1;",
            params![server_name],
        )
        .unwrap();
}
