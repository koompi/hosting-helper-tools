mod proxy_arg;
mod redir_arg;
mod filehost_arg;
mod spa_arg;
mod dname_target_arg;
mod negate_arg;

use super::{Arg, ArgAction, Command};

pub(crate) fn add_subcmd() -> Command {
    Command::new("add")
        .short_flag('a')
        .long_flag("add")
        .about("Add new NGINX configuration file")
        .arg(proxy_arg::proxy_arg())
        .arg(redir_arg::redir_arg())
        .arg(filehost_arg::filehost_arg())
        .arg(spa_arg::spa_arg())
        .args(dname_target_arg::dname_target_arg())
        .arg(negate_arg::negate_arg())
}