pub mod dbtools;
pub mod fstools;
pub mod http_server;
pub mod templates;

use std::process::Command;

fn restart_reload_service() {
    Command::new("systemctl")
        .arg("reload-or-restart")
        .arg("nginx")
        .output()
        .unwrap();
}

pub fn init_migration(force: bool) {
    [
        "/etc/nginx/sites-enabled",
        "/etc/nginx/sites-stream",
        "/etc/nginx/ssl",
    ]
    .into_iter()
    .for_each(|each| std::fs::create_dir_all(each).unwrap_or_default());

    self::fstools::write_ops::write_file(
        "/etc/nginx/nginx.conf",
        self::templates::nginx_conf::gen_templ().as_str(),
        false,
    );

    self::dbtools::migration::db_migration(force);

    Command::new("openssl")
        .arg("req")
        .arg("-x509")
        .arg("-nodes")
        .args(["-days", "365"])
        .args(["-newkey", "rsa:2048"])
        .args(["-subj", "/C=KH/ST=Cambodia/L=Phnom Penh/O=KOOMPI Co., Ltd./CN="])
        .args(["-keyout", "/etc/nginx/ssl/nginx.key"])
        .args(["-out", "/etc/nginx/ssl/nginx.crt"])
        .output()
        .unwrap();

    restart_reload_service();
}
