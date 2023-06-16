pub(crate) mod migration;
pub mod crud;

use super::{fstools::read_ops, http_server::nginx_ops::{NginxObj, NginxFeatures}, DATABASE_PATH};
use rusqlite::{params, Connection};

fn open_database() -> Connection {
    Connection::open(DATABASE_PATH).unwrap()
}