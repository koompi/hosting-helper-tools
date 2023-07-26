use super::{Arg, ArgAction};

pub(crate) fn redir_arg() -> Arg {
    Arg::new("redirect_feature")
        .short('r')
        .long("redir")
        .help("Redirect Feature: Add Redirect NGINX configuration file")
        .required(true)
        .conflicts_with_all(["proxy_feature", "filehost_feature", "spa_feature"])
        .action(ArgAction::SetTrue)
}
