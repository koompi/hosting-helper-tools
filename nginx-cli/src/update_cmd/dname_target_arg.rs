use super::{Arg, ArgAction};

pub(crate) fn dname_target_arg() -> [Arg; 2] {
    [
        Arg::new("domain_name")
            .short('d')
            .long("dname")
            .required(true)
            .action(ArgAction::Set)
            .num_args(1)
            .help("Domain Name to receive ReverseProxy/Redirect/SPA/FileHost request from; eg: koompi.com"),
        Arg::new("target")
            .short('t')
            .long("target")
            .required(true)
            .action(ArgAction::Set)
            .num_args(1..)
            .value_delimiter(',')
            .help("Domain name to ReverseProxy/Redirect/SPA/FileHost to; eg: http://localhost:8080 or https://koompi.app or /kmp/filehost-spa"),
    ]
}
