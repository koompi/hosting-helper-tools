use std::str::FromStr;

use super::{open_database, params, NginxFeatures, NginxObj, TargetSite};

pub(crate) fn create_tables() {
    open_database()
        .execute_batch(
            "BEGIN;
CREATE TABLE tblNginxConf(
    ServerName VARCHAR(100) NOT NULL UNIQUE PRIMARY KEY,
    Target JSON,
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
        .prepare("SELECT ServerName,Target,Feature FROM tblNginxConf WHERE ServerName = ?1")
        .unwrap()
        .query_row([server_name], |each_row| {
            let target_res: Vec<String> =
                serde_json::from_str(&each_row.get::<usize, String>(1).unwrap()).unwrap();
            Ok({
                let server_name = each_row.get::<usize, String>(0).unwrap();
                let target_site = match target_res.len() {
                    1 => TargetSite::Single(target_res.into_iter().next().unwrap()),
                    _ => TargetSite::Multiple(target_res),
                };
                let feature =
                    NginxFeatures::from_str(each_row.get::<usize, String>(2).unwrap().as_str())
                        .unwrap();
                NginxObj::new_unchecked(server_name, target_site, feature)
            })
        })
        .unwrap()
}

pub fn select_all_from_tbl_nginxconf() -> Vec<NginxObj> {
    open_database()
        .prepare("SELECT ServerName,Target,Feature FROM tblNginxConf")
        .unwrap()
        .query_map([], |each_row| {
            let target_res: Vec<String> =
                serde_json::from_str(&each_row.get::<usize, String>(1).unwrap()).unwrap();
            Ok({
                let server_name = each_row.get::<usize, String>(0).unwrap();
                let target_site = match target_res.len() {
                    1 => TargetSite::Single(target_res.into_iter().next().unwrap()),
                    _ => TargetSite::Multiple(target_res),
                };
                let feature =
                    NginxFeatures::from_str(each_row.get::<usize, String>(2).unwrap().as_str())
                        .unwrap();
                NginxObj::new_unchecked(server_name, target_site, feature)
            })
        })
        .unwrap()
        .map(|each| each.unwrap())
        .collect::<Vec<NginxObj>>()
}

pub fn select_all_by_feature_from_tbl_nginxconf(feature: &str) -> Vec<NginxObj> {
    open_database()
        .prepare("SELECT ServerName,Target,Feature FROM tblNginxConf WHERE Feature = ?1")
        .unwrap()
        .query_map(params![feature], |each_row| {
            let target_res: Vec<String> =
                serde_json::from_str(&each_row.get::<usize, String>(1).unwrap()).unwrap();
            Ok({
                let server_name = each_row.get::<usize, String>(0).unwrap();
                let target_site = match target_res.len() {
                    1 => TargetSite::Single(target_res.into_iter().next().unwrap()),
                    _ => TargetSite::Multiple(target_res),
                };
                let feature =
                    NginxFeatures::from_str(each_row.get::<usize, String>(2).unwrap().as_str())
                        .unwrap();
                NginxObj::new_unchecked(server_name, target_site, feature)
            })
        })
        .unwrap()
        .map(|each| each.unwrap())
        .collect::<Vec<NginxObj>>()
}

pub(crate) fn insert_tbl_nginxconf(server_name: &str, proxy_pass: &str, feature: &str) {
    open_database()
        .execute(
            &format!(
                "INSERT INTO tblNginxConf(ServerName, Target, Feature) VALUES(?1, json(?2), ?3);"
            ),
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
