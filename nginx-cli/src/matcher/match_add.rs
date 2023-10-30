use super::{
    nginx_features::NginxFeatures, nginx_obj::NginxObj, target_site::TargetSite, ArgMatches,
};

use std::str::FromStr;

#[derive(PartialEq)]
enum NegatableFeature {
    Enom,
    Cloudflare,
    SSL,
    IpCheck
}
impl FromStr for NegatableFeature {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Enom" | "enom" | "ENOM" => Ok(NegatableFeature::Enom),
            "Cloudflare" | "cloudflare" | "CLOUDFLARE" => Ok(NegatableFeature::Cloudflare),
            "SSL" | "ssl" | "Ssl" => Ok(NegatableFeature::SSL),
            "IpCheck" | "ipcheck" | "IPCHECK" | "IPCheck" | "IPcheck" => Ok(NegatableFeature::IpCheck),
            _ => Err(format!("{} is not available", s)),
        }
    }
}

pub(crate) async fn match_add(matches: &ArgMatches) {
    let domain_name = matches
        .get_one::<String>("domain_name")
        .expect("contains_id")
        .to_owned();
    let target = matches
        .get_many::<String>("target")
        .unwrap()
        .map(|each| each.to_string())
        .collect::<Vec<String>>();
    let negatefeature = match matches.get_many::<String>("negate_feature") {
        Some(data) => data
            .filter_map(|each| match NegatableFeature::from_str(each) {
                Ok(data) => Some(data),
                Err(err) => {
                    eprintln!("{}", err);
                    None
                }
            })
            .collect(),
        None => Vec::new(),
    };

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

    let ssl = !negatefeature
        .iter()
        .any(|each| each == &NegatableFeature::SSL);
    let cloudflare = !negatefeature
        .iter()
        .any(|each| each == &NegatableFeature::Cloudflare);
    let _enom = !negatefeature
        .iter()
        .any(|each| each == &NegatableFeature::Enom);
    let ip_check = !negatefeature
        .iter()
        .any(|each| each == &NegatableFeature::IpCheck);

    match NginxObj::new(
        domain_name,
        match target.len() {
            1 => TargetSite::Single(target.iter().next().unwrap().to_string()),
            _ => TargetSite::Multiple(target),
        },
        feature,
    ) {
        Ok(data_obj) => {
            match data_obj.setup_cloudflare(cloudflare, ip_check).await {
                Ok(()) => match data_obj.finish(ssl).await {
                    Ok(()) => println!("Successfully added {}", data_obj.get_server_name()),
                    Err((code, message)) => eprintln!("Error {code}: {message}"),
                },
                Err((code, message)) => eprintln!("Error {code}: {message}"),
            };
        }
        Err((code, message)) => eprintln!("Error {code}: {message}"),
    }
}
