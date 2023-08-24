use super::{nginx_features::NginxFeatures, ArgMatches};
use libnginx_wrapper::dbtools::crud::{
    select_all_by_feature_from_tbl_nginxconf, select_all_from_tbl_nginxconf,
    select_one_from_tbl_nginxconf,
};
use prettytable::{Cell, Row, Table};

pub(crate) fn match_list(matches: &ArgMatches) {
    let mut data = if matches.get_flag("proxy_feature") {
        select_all_by_feature_from_tbl_nginxconf(&NginxFeatures::Proxy.to_string())
    } else if matches.get_flag("redirect_feature") {
        select_all_by_feature_from_tbl_nginxconf(&NginxFeatures::Redirect.to_string())
    } else if matches.get_flag("spa_feature") {
        select_all_by_feature_from_tbl_nginxconf(&NginxFeatures::SPA.to_string())
    } else if matches.get_flag("filehost_feature") {
        select_all_by_feature_from_tbl_nginxconf(&NginxFeatures::FileHost.to_string())
    } else if matches.get_flag("one_feature") {
        let domain_name = matches
            .get_one::<String>("domain_name")
            .expect("contains_id")
            .to_owned();
        vec![select_one_from_tbl_nginxconf(&domain_name).unwrap_or_default()]
    } else {
        select_all_from_tbl_nginxconf()
    };

    data.sort_by_key(|each|each.get_server_name().to_owned());

    let mut data_table = data
        .iter()
        .map(|each| {
            Row::new(vec![
                Cell::new(match each.get_server_name().is_empty(){
                    true => "Not Found",
                    false => each.get_server_name()
                }),
                Cell::new(each.get_target_site().to_string().as_ref()),
                Cell::new(each.get_feature().to_string().as_str()),
            ])
        })
        .collect::<Table>();

    data_table.set_titles(Row::new(vec![
        Cell::new("Domain Name"),
        Cell::new("Target"),
        Cell::new("Feature"),
    ]));

    data_table.printstd()
}
