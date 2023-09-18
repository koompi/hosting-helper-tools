pub mod crud;
pub(crate) mod migration;

use super::{
    fstools::read_ops,
    http_server::{nginx_features::NginxFeatures, nginx_obj::NginxObj, target_site::TargetSite},
};
use rusqlite::{params, Connection};

fn open_database() -> Connection {
    let database = dotenv::var("DATABASE_PATH").unwrap();
    Connection::open(match std::path::Path::new(&database).is_absolute() {
        true => database.to_owned(),
        false => format!(
            "{}/{database}",
            std::env::current_exe()
                .unwrap()
                .parent()
                .unwrap()
                .to_str()
                .unwrap()
        ),
    })
    .unwrap()
}
