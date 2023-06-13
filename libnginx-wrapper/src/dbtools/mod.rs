pub mod migration;
pub mod crud;

use super::fstools::read_ops;
use rusqlite::{params, Connection};

const DATABASE: &str = "./libnginx-wrapper.db";

fn open_database() -> Connection {
    // self::migration::init_migration(false);
    Connection::open(DATABASE).unwrap()
}