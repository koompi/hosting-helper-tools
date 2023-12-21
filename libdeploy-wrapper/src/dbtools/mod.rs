use libdatabase::{open_database, params, Statement, ToSql};

pub fn query_existence_from_tbl_deploydata(server_name: &str) -> bool {
    let connection = open_database();

    let mut stmt = connection
        .prepare("SELECT EXISTS(SELECT ServerName FROM tblDeployData WHERE ServerName=? LIMIT 1);")
        .unwrap();
    let mut rows = stmt.query(&[server_name]).unwrap();

    rows.next().unwrap().unwrap().get::<usize, u64>(0).unwrap() != 0
}

pub fn select_all_ports_from_tbl_deploydata() -> Vec<u16> {
    let database = open_database();
    let mut stmt = database
        .prepare("SELECT PortNumber FROM tblDeployData")
        .unwrap();
    extract_usize(&mut stmt, params![])
        .iter()
        .map(|each| *each as u16)
        .collect()
}

pub fn select_themedata_from_tbl_deploydata(server_name: &str) -> String {
    let database = open_database();
    let mut stmt = database
        .prepare("SELECT ThemePath FROM tblDeployData WHERE ServerName=?1")
        .unwrap();

    stmt.query_row(params![server_name], |each| each.get::<usize, String>(0))
        .unwrap()
}

pub fn delete_from_tbl_deploydata(server_name: &str) -> Result<(), (u16, String)> {
    open_database()
        .execute(
            "DELETE FROM tblDeploy WHERE ServerName=?1",
            params![server_name],
        )
        .unwrap();

    Ok(())
}

pub fn insert_tbl_deploydata(portnumber: u16, theme_path: &str, server_name: &str) {
    open_database()
        .execute(
            &format!(
                "INSERT INTO tblDeployData(PortNumber, ThemePath, ServerName) VALUES(?1, ?2, ?3);"
            ),
            params![portnumber, theme_path, server_name],
        )
        .unwrap();
}

fn extract_usize(stmt: &mut Statement, parameter: &[&dyn ToSql]) -> Vec<usize> {
    stmt.query_map(parameter, |each| each.get::<usize, usize>(0))
        .unwrap()
        .filter_map(|each| Some(each.unwrap()))
        .collect::<Vec<usize>>()
}
