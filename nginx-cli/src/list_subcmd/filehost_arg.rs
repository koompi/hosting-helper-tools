use super::{Arg, ArgAction};

pub(crate) fn filehost_arg() -> Arg {
    Arg::new("filehost_feature")
        .short('f')
        .long("filehost")
        .help("List NGINX Object of FileHost Feature")
        .required(true)
        .action(ArgAction::SetTrue)
        .conflicts_with_all(["all_feature", "redirect_feature", "proxy_feature", "spa_feature", "one_feature"])
}
