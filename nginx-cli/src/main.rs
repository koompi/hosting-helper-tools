use clap::{Arg, ArgAction, Command};

mod add_subcmd;
mod del_subcmd;
mod list_subcmd;
mod force_subcmd;
mod matcher;

use matcher::{match_add,match_del,match_list, match_force};

fn main() {

    libnginx_wrapper::init_migration(false);

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
        .get_matches();

    match matches.subcommand() {
        Some(("add", add_matches)) => match_add::match_add(add_matches),
        Some(("delete", delete_matches)) => match_del::match_del(delete_matches),
        Some(("list", list_matches)) => match_list::match_list(list_matches),
        Some(("force", force_matches)) => match_force::match_force(force_matches),
        _ => unreachable!()
    }
}
