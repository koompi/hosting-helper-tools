use super::{nginx_features::NginxFeatures, ArgMatches};
use libnginx_wrapper::dbtools::crud::{
    select_all_by_feature_from_tbl_nginxconf, select_all_from_tbl_nginxconf,
};
use prettytable::{Cell, Row, Table};

pub(crate) fn match_list(matches: &ArgMatches) {
    let data = if matches.get_flag("proxy_feature") {
        select_all_by_feature_from_tbl_nginxconf(&NginxFeatures::Proxy.to_string())
    } else if matches.get_flag("redirect_feature") {
        select_all_by_feature_from_tbl_nginxconf(&NginxFeatures::Redirect.to_string())
    } else if matches.get_flag("spa_feature") {
        select_all_by_feature_from_tbl_nginxconf(&NginxFeatures::SPA.to_string())
    } else if matches.get_flag("filehost_feature") {
        select_all_by_feature_from_tbl_nginxconf(&NginxFeatures::FileHost.to_string())
    } else {
        select_all_from_tbl_nginxconf()
    };

    let mut data_table = data.iter().map(|each| {
        Row::new(vec![
            Cell::new(each.get_server_name()),
            Cell::new(each.get_target_site().to_string().as_ref()),
            Cell::new(each.get_feature().to_string().as_str()),
        ])
    }).collect::<Table>();

    data_table.set_titles(Row::new(vec![
        Cell::new("Domain Name"),
        Cell::new("Target"),
        Cell::new("Feature"),
    ]));

    data_table.printstd()
}
