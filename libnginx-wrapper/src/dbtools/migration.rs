use super::{
    crud::{create_tables, insert_tbl_nginxconf},
    read_ops, DATABASE_PATH,
};

pub(crate) fn db_migration(force: bool) {
    let dbpath = match std::path::Path::new(DATABASE_PATH).is_absolute() {
        true => DATABASE_PATH.to_owned(),
        false => format!(
            "{}/{DATABASE_PATH}",
            std::env::current_exe()
                .unwrap()
                .parent()
                .unwrap()
                .to_str()
                .unwrap()
        ),
    };

    if force {
        std::fs::remove_file(&dbpath).unwrap();
    }

    if !std::path::Path::new(&dbpath).exists() {
        create_tables();

        read_ops::read_nginx_dir().into_iter().for_each(|each| {
            insert_tbl_nginxconf(
                each.get_server_name(),
                each.get_target_site().to_string().as_str(),
                each.get_feature().to_string().as_ref(),
            )
        });
    }
}
