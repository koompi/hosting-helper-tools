pub use rusqlite::{params, Connection, Error, Row};

pub enum DBClient {
    LibCloudflare,
    LibNginx,
}

pub fn open_database() -> Connection {
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

pub fn read_dotenv() {
    dotenv::from_path(format!(
        "{}/.env",
        std::env::current_exe()
            .unwrap()
            .parent()
            .unwrap()
            .to_str()
            .unwrap()
    )).unwrap()
}

pub(crate) fn create_tables(db: Option<DBClient>) {
    let batch_str = vec![
        "BEGIN;",
        match db {
            Some(db) => match db {
                DBClient::LibCloudflare => {
                    "CREATE TABLE tblCloudflarePending (
                ServerName VARCHAR(100) NOT NULL UNIQUE PRIMARY KEY,
                NewDNS JSON NOT NULL,
                OldDNS JSON,
                Registra VARCHAR(100),
                LastCheck VARCHAR(100) NOT NULL
            );
            CREATE TABLE tblCloudflareData (
                ServerName VARCHAR(100) NOT NULL UNIQUE PRIMARY KEY,
                ZoneID VARCHAR(100) NOT NULL UNIQUE
            );"
                }
                DBClient::LibNginx => {
                    "CREATE TABLE tblNginxConf(
                ServerName VARCHAR(100) NOT NULL UNIQUE PRIMARY KEY,
                Target JSON,
                Feature VARCHAR(100) NOT NULL
            );"
                }
            },
            None => {
                "CREATE TABLE tblNginxConf(
            ServerName VARCHAR(100) NOT NULL UNIQUE PRIMARY KEY,
            Target JSON,
            Feature VARCHAR(100) NOT NULL
        );
        CREATE TABLE tblCloudflarePending (
            ServerName VARCHAR(100) NOT NULL UNIQUE PRIMARY KEY,
            NewDNS JSON NOT NULL,
            OldDNS JSON,
            Registra VARCHAR(100)
        );
        CREATE TABLE tblCloudflareData (
            ServerName VARCHAR(100) NOT NULL UNIQUE PRIMARY KEY,
            ZoneID VARCHAR(100) NOT NULL UNIQUE
        );"
            }
        },
        "COMMIT;",
    ]
    .join(" ");

    open_database().execute_batch(&batch_str).unwrap();
}

pub fn db_migration(db: DBClient, force: Option<DBClient>) -> Option<u8> {
    if let Some(db) = force {
        open_database()
            .execute_batch(match db {
                DBClient::LibCloudflare => {
                    "BEGIN; DROP TABLE tblCloudflarePending; DROP TABLE tblCloudflareData; COMMIT;"
                }
                DBClient::LibNginx => "BEGIN; DROP TABLE tblNginxConf; COMMIT;",
            })
            .unwrap();
    }

    match db {
        DBClient::LibCloudflare => (open_database()
            .query_row(
                "SELECT COUNT(name) FROM sqlite_master WHERE type='table' AND name=?1 OR name=?2;",
                params!["tblCloudflarePending", "tblCloudflareData"],
                |data| data.get::<usize, usize>(0),
            )
            .unwrap()
            != 2)
            .then(|| {
                create_tables(Some(DBClient::LibCloudflare));
                Some(0)
            }),
        DBClient::LibNginx => (open_database()
            .query_row(
                "SELECT COUNT(name) FROM sqlite_master WHERE type='table' AND name=?1",
                params!["tblNginxConf"],
                |data| data.get::<usize, usize>(0),
            )
            .unwrap()
            != 1)
            .then(|| {
                create_tables(Some(DBClient::LibNginx));
                Some(0)
            }),
    }?

    // None // Return None when it actually did Nothing
}
