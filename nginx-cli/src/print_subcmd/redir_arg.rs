use super::{Arg, ArgAction};

pub(crate) fn redir_arg() -> Arg {
    Arg::new("redirect_feature")
        .short('r')
        .long("redir")
        .help("List Redirect Feature")
        .required(true)
        .conflicts_with_all(["all", "proxy_feature", "filehost_feature", "spa_feature"])
        .action(ArgAction::SetTrue)
}
