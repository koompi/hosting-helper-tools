use super::{
    crud::{create_tables, insert_tbl_nginxconf},
    read_ops, DATABASE_PATH,
};

pub(crate) fn db_migration(force: bool) {
    if force {
        std::fs::remove_file(DATABASE_PATH).unwrap();
    }

    if !std::path::Path::new(DATABASE_PATH).exists() {
        create_tables();

        read_ops::read_nginx_dir().into_iter().for_each(|each| {
            insert_tbl_nginxconf(
                each.get_server_name(),
                each.get_target_site(),
                each.get_feature().to_string().as_ref(),
            )
        });
    }
}
