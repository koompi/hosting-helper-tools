use super::{nginx_obj::NginxObj, ArgMatches, target_site::TargetSite};

pub(crate) fn match_update(matches: &ArgMatches) {
    let domain_name = matches
        .get_one::<String>("domain_name")
        .expect("contains_id")
        .to_owned();

    let target = matches
        .get_many::<String>("target")
        .expect("contains_id")
        .map(|each| each.to_string())
        .collect::<Vec<String>>();

    match NginxObj::update_target(
        &domain_name,
        match target.len() {
            1 => TargetSite::Single(target.iter().next().unwrap().to_string()),
            _ => TargetSite::Multiple(target),
        }
    ) {
        Ok(()) => println!("Successfully Updated {}", domain_name),
        Err((code, message)) => eprintln!("Error {code}: {message}"),
    }
}
