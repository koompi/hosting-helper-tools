mod add_subcmd;
mod del_subcmd;
mod print_subcmd;

use clap::{Arg, ArgAction, Command};

fn main() {
    let matches = Command::new("nginx-cli")
        .about("Nginx Helper Tools")
        .version("1.0.0")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .author("KOOMPI Development Team")
        .subcommand(add_subcmd::add_subcmd())
        .subcommand(del_subcmd::del_subcmd())
        .subcommand(print_subcmd::print_subcmd())

        .get_matches();

    match matches.subcommand() {
        Some(_t) => {},
        None => {}
    }
}