mod proxy_arg;
mod redir_arg;
mod filehost_arg;
mod spa_arg;
mod all_arg;
mod one_arg;

use super::{Arg, ArgAction, Command};

pub(crate) fn list_subcmd() -> Command {
    Command::new("list")
        .short_flag('l')
        .long_flag("list")
        .about("list NGINX configuration file")
        .arg(proxy_arg::proxy_arg())
        .arg(redir_arg::redir_arg())
        .arg(filehost_arg::filehost_arg())
        .arg(spa_arg::spa_arg())
        .arg(all_arg::all_arg())
        .args(one_arg::one_arg())
}