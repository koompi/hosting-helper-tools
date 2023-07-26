use super::{Arg, ArgAction};

pub(crate) fn proxy_arg() -> Arg {
    Arg::new("proxy_feature")
        .short('p')
        .long("proxy")
        .help("Proxy Feature: Add Reverse Proxy NGINX configuration file")
        .required(true)
        .conflicts_with_all(["redirect_feature", "filehost_feature", "spa_feature"])
        .action(ArgAction::SetTrue)
}
