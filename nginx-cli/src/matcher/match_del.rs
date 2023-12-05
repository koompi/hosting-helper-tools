use super::{remove_nginx_conf, ArgMatches};

pub(crate) async fn match_del(matches: &ArgMatches) {
    let domain_name = matches
        .get_one::<String>("domain_name")
        .expect("contains_id")
        .to_owned();

    if let Err((code, message)) = remove_nginx_conf(None, None, &domain_name).await {
        eprintln!("Error {code}: {message}");
    } else {
        println!("Successfully deleted {domain_name}")
    }
}
