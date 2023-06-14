pub mod migration;
pub mod crud;

use super::{fstools::read_ops, http_server::NginxObj};
use rusqlite::{params, Connection};

const DATABASE: &str = "./libnginx-wrapper.db";

fn open_database() -> Connection {
    Connection::open(DATABASE).unwrap()
}