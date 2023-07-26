use super::{
    nginx_ops::{NginxFeatures, NginxObj},
    ArgMatches,
};

pub(crate) fn match_add(matches: &ArgMatches) {
    let domain_name = matches
        .get_one::<String>("domain_name")
        .expect("contains_id")
        .to_owned();
    let target = matches
        .get_one::<String>("target")
        .expect("contains_id")
        .to_owned();

    let feature = if matches.get_flag("proxy_feature") {
        NginxFeatures::Proxy
    } else if matches.get_flag("redirect_feature") {
        NginxFeatures::Redirect
    } else if matches.get_flag("spa_feature") {
        NginxFeatures::SPA
    } else if matches.get_flag("filehost_feature") {
        NginxFeatures::FileHost
    } else {
        NginxFeatures::Proxy
    };

    let data = NginxObj::new(domain_name, target, feature);

    if let Err((code, message)) = data.verify() {
        eprintln!("Error Code: {code}\nError Message: {message}");
    } else {
        if let Err((code, message)) = data.finish() {
            eprintln!("Error Code: {code}\nError Message: {message}");
        } else {
            println!("Successfully added {}", data.get_server_name())
        };
    }
}
