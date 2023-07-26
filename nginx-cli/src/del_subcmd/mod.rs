use super::{Arg, ArgAction, Command};

pub(crate) fn del_subcmd() -> Command {
    Command::new("delete")
        .short_flag('a')
        .long_flag("add")
        .about("Delete NGINX configuration file")
        .arg(
            Arg::new("domain_name")
            .short('d')
            .long("dname")
            .required(true)
            .action(ArgAction::Set)
            .num_args(1)
            .help("Delete by Domain Name"),
        )
}