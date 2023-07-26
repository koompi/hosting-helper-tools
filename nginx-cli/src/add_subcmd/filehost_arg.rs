use super::{Arg, ArgAction};

pub(crate) fn filehost_arg() -> Arg {
    Arg::new("filehost_feature")
        .short('f')
        .long("filehost")
        .help("FileHost Feature: Add File Hosting NGINX configuration file")
        .required(true)
        .action(ArgAction::SetTrue)
        .conflicts_with_all(["redirect_feature", "proxy_feature", "spa_feature"])
}
