use super::{Arg, ArgAction};

pub(crate) fn spa_arg() -> Arg {
    Arg::new("spa_feature")
        .short('s')
        .long("spa")
        .help("SPA Feature: Add Single Page App NGINX configuration file")
        .required(true)
        .conflicts_with_all(["redirect_feature", "filehost_feature", "proxy_feature"])
        .action(ArgAction::SetTrue)
}
