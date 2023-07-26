use clap::{Arg, ArgAction, Command};

mod add_subcmd;
mod del_subcmd;
mod list_subcmd;
mod matcher;

use matcher::{match_add,match_del,match_list};

fn main() {
    let matches = Command::new(env!("CARGO_PKG_NAME"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .version(env!("CARGO_PKG_VERSION"))
        .subcommand_required(true)
        .arg_required_else_help(true)
        .author(env!("CARGO_PKG_AUTHORS"))
        .subcommand(add_subcmd::add_subcmd())
        .subcommand(del_subcmd::del_subcmd())
        .subcommand(list_subcmd::print_subcmd())
        .get_matches();

    match matches.subcommand() {
        Some(("add", add_matches)) => match_add::match_add(add_matches),
        Some(("delete", delete_matches)) => match_del::match_del(delete_matches),
        Some(("list", list_matches)) => match_list::match_list(list_matches),
        _ => {}
    }
}
