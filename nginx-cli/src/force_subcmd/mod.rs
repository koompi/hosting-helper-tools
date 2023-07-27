use super::{Arg, ArgAction, Command};

mod cert_arg;
mod migration_arg;

pub(crate) fn force_subcmd() -> Command {
    Command::new("force")
        .short_flag('F')
        .long_flag("force")
        .about("Force NGINX or Certbot Program")
        .args([cert_arg::cert_arg(), cert_arg::cert_arg_dname()])
        .arg(migration_arg::migration_arg())
}