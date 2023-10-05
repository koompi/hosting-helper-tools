pub mod crud;

use super::{
    fstools::read_ops,
    http_server::{nginx_features::NginxFeatures, nginx_obj::NginxObj, target_site::TargetSite},
};
use libdatabase::{open_database, DBClient};

pub(crate) fn db_migration(force: bool) {
    libdatabase::db_migration(DBClient::LibNginx, match force {
        true => Some(DBClient::LibNginx),
        false => None,
    })
    .and_then(|_| {
        Some(read_ops::read_nginx_dir().into_iter().for_each(|each| {
            crud::insert_tbl_nginxconf(
                each.get_server_name(),
                each.get_target_site().to_string().as_str(),
                each.get_feature().to_string().as_ref(),
            )
        }))
    });
}
