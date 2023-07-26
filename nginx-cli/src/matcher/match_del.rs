use super::{nginx_ops::remove_nginx_conf, ArgMatches};

pub(crate) fn match_del(matches: &ArgMatches) {
    let domain_name = matches
        .get_one::<String>("domain_name")
        .expect("contains_id")
        .to_owned();

    if let Err((code, message)) = remove_nginx_conf(&domain_name) {
        eprintln!("Error Code: {code}\nError Message: {message}");
    } else {
        println!("Successfully deleted {domain_name}")
    }
}
