use super::{Arg, ArgAction};

pub(crate) fn all_arg() -> Arg {
    Arg::new("all_feature")
        .short('a')
        .long("all")
        .help("FileHost Feature: Add File Hosting NGINX configuration file")
        .required(true)
        .action(ArgAction::SetTrue)
        .conflicts_with_all(["redirect_feature", "proxy_feature", "spa_feature", "filehost_feature"])
}
