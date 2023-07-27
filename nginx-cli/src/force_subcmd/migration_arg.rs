use super::{Arg, ArgAction};

pub(crate) fn migration_arg() -> Arg {
    Arg::new("db_migration")
        .short('m')
        .long("migration")
        .help("Database Migration: Force Repopulate DB with configuration file ")
        .required(true)
        .conflicts_with("renew_certificate")
        .action(ArgAction::SetTrue)
}
