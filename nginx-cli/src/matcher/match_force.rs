use super::{ArgMatches, remake_ssl};

pub(crate) async fn match_force(matches: &ArgMatches) {
    if matches.get_flag("renew_certificate") {
        let domain_name = matches
            .get_one::<String>("domain_name")
            .expect("contains_id")
            .to_owned();
        match remake_ssl(&domain_name) {
            Ok(()) => println!("Successfully Regenerated SSL"),
            Err((code, message)) => eprintln!("Error {code}: {message}"),
        }
    } else if matches.get_flag("db_migration") {
        match libnginx_wrapper::init_migration(true) {
            Ok(()) => match libcloudflare_wrapper::db_migration(true).await {
                Ok(()) => println!("Finished!"),
                Err((code, message)) => eprintln!("Error {code}: {message}")
            },
            Err((code, message)) => eprintln!("Error {code}: {message}"),
        };
    }
}
