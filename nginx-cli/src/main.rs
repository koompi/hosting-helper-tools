use clap::{Arg, ArgAction, Command};

mod add_subcmd;
mod del_subcmd;
mod force_subcmd;
mod list_subcmd;
mod matcher;
mod update_cmd;

use matcher::{match_add, match_del, match_force, match_list, match_update};

#[tokio::main]
async fn main() {
    libdatabase::read_dotenv();

    let migrated_sign = format!(
        "{}/.migrated",
        std::env::current_exe()
            .unwrap()
            .parent()
            .unwrap()
            .to_str()
            .unwrap()
    );
    if !std::path::Path::new(&migrated_sign).exists() {
        libnginx_wrapper::init_migration(false)
            .unwrap_or_else(|err| eprintln!("Error Nginx Migration {}: {}", err.0, err.1));
        libcloudflare_wrapper::db_migration(false)
            .await
            .unwrap_or_else(|err| eprintln!("Error Cloudflare Migration {}: {}", err.0, err.1));
        std::fs::File::create(migrated_sign).unwrap();
    }

    let matches = Command::new(env!("CARGO_PKG_NAME"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .version(env!("CARGO_PKG_VERSION"))
        .subcommand_required(true)
        .arg_required_else_help(true)
        .author(env!("CARGO_PKG_AUTHORS"))
        .subcommand(add_subcmd::add_subcmd())
        .subcommand(del_subcmd::del_subcmd())
        .subcommand(list_subcmd::list_subcmd())
        .subcommand(force_subcmd::force_subcmd())
        .subcommand(update_cmd::update_subcmd())
        .get_matches();

    match matches.subcommand() {
        Some(("add", add_matches)) => match_add::match_add(add_matches).await,
        Some(("delete", delete_matches)) => match_del::match_del(delete_matches).await,
        Some(("list", list_matches)) => match_list::match_list(list_matches),
        Some(("force", force_matches)) => match_force::match_force(force_matches).await,
        Some(("update", update_matches)) => match_update::match_update(update_matches).await,
        _ => unreachable!(),
    }
}
