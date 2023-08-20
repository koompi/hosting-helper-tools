use super::{Arg, ArgAction};

pub(crate) fn spa_arg() -> Arg {
    Arg::new("spa_feature")
        .short('s')
        .long("spa")
        .help("List NGINX Object of SPA Feature")
        .required(true)
        .conflicts_with_all(["all_feature", "redirect_feature", "filehost_feature", "proxy_feature", "one_feature"])
        .action(ArgAction::SetTrue)
}
