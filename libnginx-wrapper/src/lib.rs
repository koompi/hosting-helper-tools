pub mod dbtools;
pub mod fstools;
pub mod http_server;
pub mod templates;

use std::process::Command;

const PROGRAM_BASE_NAME: &str = "nginx";
const PROGRAM_BASE_PATH: &str = "/etc/nginx";
const STREAM_SITES_PATH: &str = "/etc/nginx/stream-sites";
const REDIRECT_SITES_PATH: &str = "/etc/nginx/redirect-sites";
const PROXY_SITES_PATH: &str = "/etc/nginx/proxy-sites";
const SPA_SITES_PATH: &str = "/etc/nginx/spa-sites";
const FILE_SITES_PATH: &str = "/etc/nginx/file-sites";
const NGINX_DEFAULT_CERT_PATH: &str = "/etc/nginx/ssl";
const DATABASE_PATH: &str = "./libnginx-wrapper.db";

fn restart_reload_service() {
    Command::new("systemctl")
        .arg("reload-or-restart")
        .arg(PROGRAM_BASE_NAME)
        .output()
        .unwrap();
}

pub fn init_migration(force: bool) {
    fn nginx_migration(force: bool) {
        // Create All Necessary Directory
        [
            PROGRAM_BASE_PATH,
            STREAM_SITES_PATH,
            REDIRECT_SITES_PATH,
            PROXY_SITES_PATH,
            NGINX_DEFAULT_CERT_PATH,
        ]
        .into_iter()
        .for_each(|each| std::fs::create_dir_all(each).unwrap_or_default());

        // Make {PROGRAM_BASE_NAME}.conf
        self::fstools::write_ops::write_file(
            format!("{PROGRAM_BASE_PATH}/{PROGRAM_BASE_NAME}.conf").as_str(),
            &self::templates::nginx_conf::gen_templ(),
            false,
        );

        // Read all configuration into DB
        self::dbtools::migration::db_migration(force);

        // Make Certificate for Any websites query that doesn't exist
        Command::new("openssl")
            .arg("req")
            .arg("-x509")
            .arg("-nodes")
            .args(["-days", "365"])
            .args(["-newkey", "rsa:2048"])
            .args([
                "-subj",
                "/C=KH/ST=Cambodia/L=Phnom Penh/O=KOOMPI Co., Ltd./CN=",
            ])
            .args([
                "-keyout",
                &format!("{NGINX_DEFAULT_CERT_PATH}/{PROGRAM_BASE_NAME}.key"),
            ])
            .args([
                "-out",
                &format!("{NGINX_DEFAULT_CERT_PATH}/{PROGRAM_BASE_NAME}.crt"),
            ])
            .output()
            .unwrap();

        // Reload service
        restart_reload_service();
    }

    fn systemd_migration() {
        let service_name = "renew_certbot";
        let service_path = format!("/etc/systemd/system/{service_name}.service");
        let timer_path = format!("/etc/systemd/system/daily@{service_name}.timer");

        // Write Service File
        self::fstools::write_ops::write_file(
            &service_path,
            &self::templates::systemd_file::gen_service_template(),
            false,
        );

        // Write Timer file
        self::fstools::write_ops::write_file(
            &timer_path,
            &self::templates::systemd_file::gen_timer_template(),
            false,
        );

        // Reload systemd
        Command::new("systemctl")
            .arg("daemon-reload")
            .output()
            .unwrap();

        // Enable and Run the timer
        Command::new("systemctl")
            .arg("enable")
            .arg("--now")
            .arg(timer_path)
            .output()
            .unwrap();
    }

    nginx_migration(force);
    systemd_migration();
}
