use super::{nginx_obj::NginxObj, target_site::TargetSite, ArgMatches};

pub(crate) async fn match_update(matches: &ArgMatches) {
    let domain_name = matches
        .get_one::<String>("domain_name")
        .expect("contains_id")
        .to_owned();

    let target = matches
        .get_many::<String>("target")
        .expect("contains_id")
        .map(|each| each.to_string())
        .collect::<Vec<String>>();

    let negatefeature = match matches.get_many::<String>("negate_feature") {
        Some(data) => data
            .filter_map(|each| if each == "ssl" { Some("ssl") } else { None })
            .collect(),
        None => Vec::new(),
    };

    let ssl = negatefeature.iter().any(|each| each == &"ssl");

    match NginxObj::update_target(
        &domain_name,
        match target.len() {
            1 => TargetSite::Single(target.iter().next().unwrap().to_string()),
            _ => TargetSite::Multiple(target),
        },
        !ssl,
    )
    .await
    {
        Ok(()) => println!("Successfully Updated {}", domain_name),
        Err((code, message)) => eprintln!("Error {code}: {message}"),
    }
}
