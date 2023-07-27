use super::{Arg, ArgAction};

pub(crate) fn cert_arg() -> Arg {
    Arg::new("renew_certificate")
        .short('c')
        .long("cert")
        .help("Renew Certificate: Force Certbot renew certificate for domain name NGINX configuration file")
        .required(true)
        .conflicts_with("db_migration")
        .action(ArgAction::SetTrue)
}

pub(crate) fn cert_arg_dname() -> Arg {
    Arg::new("domain_name")
        .short('d')
        .long("dname")
        .required_unless_present("db_migration")
        .action(ArgAction::Set)
        .num_args(1)
        .help("Domain Name to force redo certificate")
}
