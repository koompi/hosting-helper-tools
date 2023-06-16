pub mod dbtools;
pub mod fstools;
pub mod http_server;
pub mod templates;

use std::process::Command;

const STREAM_SITES_PATH: &str = "/etc/nginx/stream-sites";
const REDIRECT_SITES_PATH: &str = "/etc/nginx/redirect-sites";
const PROXY_SITES_PATH: &str = "/etc/nginx/proxy-sites";
const NGINX_DEFAULT_CERT_PATH: &str = "/etc/nginx/ssl";
const DATABASE_PATH: &str = "./libnginx-wrapper.db";

fn restart_reload_service() {
    Command::new("systemctl")
        .arg("reload-or-restart")
        .arg("nginx")
        .output()
        .unwrap();
}

pub fn init_migration(force: bool) {
    [
        STREAM_SITES_PATH,
        REDIRECT_SITES_PATH,
        PROXY_SITES_PATH,
        NGINX_DEFAULT_CERT_PATH,
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
        .args(["-keyout", &format!("{NGINX_DEFAULT_CERT_PATH}/nginx.key")])
        .args(["-out", &format!("{NGINX_DEFAULT_CERT_PATH}/nginx.crt")])
        .output()
        .unwrap();

    restart_reload_service();
}
