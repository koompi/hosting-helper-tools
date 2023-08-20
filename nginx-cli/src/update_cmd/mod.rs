use super::{Arg, ArgAction, Command};
mod dname_target_arg;

pub(crate) fn update_subcmd() -> Command {
    Command::new("update")
        .short_flag('u')
        .long_flag("update")
        .about("Update existing NGINX configuration file")
        .args(dname_target_arg::dname_target_arg())
}