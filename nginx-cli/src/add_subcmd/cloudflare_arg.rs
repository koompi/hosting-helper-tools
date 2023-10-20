use super::{Arg, ArgAction};

pub(crate) fn cloudflare_arg() -> Arg {
    Arg::new("no_cloudflare_feature")
        .short('c')
        .long("nocloudflare")
        .help("No Cloudflare Feature: Add Option to not switch Domain IP DNS Records in Cloudflare; For example: setup internal domain that does not exists in the KOOMPI Account, but we are sure that it is already pointed to our server")
        .action(ArgAction::SetTrue)
        .default_value("false")
}
