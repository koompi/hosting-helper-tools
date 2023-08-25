use super::{Arg, ArgAction};

pub(crate) fn one_arg() -> [Arg; 2] {
    [Arg::new("one_feature")
        .short('o')
        .long("one")
        .help("List one NGINX Object")
        .required(true)
        .action(ArgAction::SetTrue)
        .conflicts_with_all(["redirect_feature", "proxy_feature", "spa_feature", "filehost_feature", "all_feature"]),
        Arg::new("domain_name")
        .short('d')
        .long("dname")
        .required_unless_present_any(["redirect_feature", "proxy_feature", "spa_feature", "filehost_feature", "all_feature"])
        .action(ArgAction::Set)
        .num_args(1)
        .help("Domain Name to search ReverseProxy/Redirect/SPA/FileHost for; eg: koompi.com")]
}
