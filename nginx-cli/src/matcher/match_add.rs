use super::{
    nginx_features::NginxFeatures, nginx_obj::NginxObj, target_site::TargetSite, ArgMatches,
};

pub(crate) fn match_add(matches: &ArgMatches) {
    let domain_name = matches
        .get_one::<String>("domain_name")
        .expect("contains_id")
        .to_owned();
    let target = matches
        .get_many::<String>("target")
        .expect("contains_id")
        .map(|each| each.to_string())
        .collect::<Vec<String>>();

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

    match NginxObj::new(
        domain_name,
        match target.len() {
            1 => TargetSite::Single(target.iter().next().unwrap().to_string()),
            _ => TargetSite::Multiple(target),
        },
        feature,
    ) {
        Ok(data_obj) => match data_obj.finish() {
            Ok(()) => println!("Successfully added {}", data_obj.get_server_name()),
            Err((code, message)) => eprintln!("Error {code}: {message}"),
        },
        Err((code, message)) => eprintln!("Error {code}: {message}"),
    }
}
