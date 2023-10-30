use super::{Arg, ArgAction};

pub(crate) fn negate_arg() -> Arg {
    Arg::new("negate_feature")
        .short('n')
        .long("no")
        .help("Negate Feature: Nullify an Integrated features: SSL or Cloudflare or Enom")
        .action(ArgAction::Set)
        .num_args(1..)
        .value_delimiter(',')
}
