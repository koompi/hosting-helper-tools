use super::{open_database, NginxFeatures, NginxObj, TargetSite};
use libdatabase::params;
use std::str::FromStr;

pub(crate) fn query_existence_from_tbl_nginxconf(server_name: &str) -> bool {
    let connection = super::open_database();

    let mut stmt = connection
        .prepare("SELECT EXISTS(SELECT ServerName FROM tblNginxConf WHERE ServerName=? LIMIT 1);")
        .unwrap();
    let mut rows = stmt.query(&[server_name]).unwrap();

    rows.next().unwrap().unwrap().get::<usize, u64>(0).unwrap() != 0
}

pub fn select_one_from_tbl_nginxconf(
    server_name: &str,
    feature: Option<&NginxFeatures>,
) -> Result<NginxObj, libdatabase::Error> {
    match feature {
        Some(feature) => open_database()
        .prepare("SELECT ServerName,Target,Feature FROM tblNginxConf WHERE ServerName = ?1 AND Feature = ?2")
        .unwrap()
        .query_row(params![server_name, feature.to_string().as_str()], extract_single_nginxobj),
        None => open_database()
        .prepare("SELECT ServerName,Target,Feature FROM tblNginxConf WHERE ServerName = ?1")
        .unwrap()
        .query_row(params![server_name], extract_single_nginxobj),
    }
}

pub fn select_all_from_tbl_nginxconf() -> Vec<NginxObj> {
    open_database()
        .prepare("SELECT ServerName,Target,Feature FROM tblNginxConf")
        .unwrap()
        .query_map([], extract_single_nginxobj)
        .unwrap()
        .map(|each| each.unwrap())
        .collect::<Vec<NginxObj>>()
}

pub fn select_all_by_feature_from_tbl_nginxconf(feature: &str) -> Vec<NginxObj> {
    open_database()
        .prepare("SELECT ServerName,Target,Feature FROM tblNginxConf WHERE Feature = ?1")
        .unwrap()
        .query_map(params![feature], extract_single_nginxobj)
        .unwrap()
        .map(|each| each.unwrap())
        .collect::<Vec<NginxObj>>()
}

pub(crate) fn insert_tbl_nginxconf(server_name: &str, target: &str, feature: &str) {
    open_database()
        .execute(
            &format!(
                "INSERT INTO tblNginxConf(ServerName, Target, Feature) VALUES(?1, json(?2), ?3);"
            ),
            params![server_name, target, feature],
        )
        .unwrap();
}

pub(crate) fn update_target_tbl_nginxconf(server_name: &str, target: &str) {
    open_database()
        .execute(
            &format!("UPDATE tblNginxConf SET Target = json(?2) WHERE ServerName = ?1;"),
            params![server_name, target],
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

fn extract_single_nginxobj(
    each_row: &libdatabase::Row<'_>,
) -> Result<NginxObj, libdatabase::Error> {
    let target_res: Vec<String> =
        serde_json::from_str(&each_row.get::<usize, String>(1).unwrap()).unwrap();
    Ok({
        let server_name = each_row.get::<usize, String>(0).unwrap();
        let target_site = match target_res.len() {
            1 => TargetSite::Single(target_res.into_iter().next().unwrap()),
            _ => TargetSite::Multiple(target_res),
        };
        let feature =
            NginxFeatures::from_str(each_row.get::<usize, String>(2).unwrap().as_str()).unwrap();
        NginxObj::new_unchecked(server_name, target_site, feature)
    })
}
