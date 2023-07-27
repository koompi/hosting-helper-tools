pub mod crud;
pub(crate) mod migration;

use super::{
    fstools::read_ops,
    http_server::nginx_ops::{NginxFeatures, NginxObj},
    DATABASE_PATH,
};
use rusqlite::{params, Connection};

fn open_database() -> Connection {
    Connection::open(match std::path::Path::new(DATABASE_PATH).is_absolute() {
        true => DATABASE_PATH.to_owned(),
        false => format!(
            "{}/{DATABASE_PATH}",
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
