use super::{Arg, ArgAction};

pub(crate) fn redir_arg() -> Arg {
    Arg::new("redirect_feature")
        .short('r')
        .long("redir")
        .help("List NGINX Object of Redirect Feature")
        .required(true)
        .conflicts_with_all(["all_feature", "proxy_feature", "filehost_feature", "spa_feature", "one_feature"])
        .action(ArgAction::SetTrue)
}
