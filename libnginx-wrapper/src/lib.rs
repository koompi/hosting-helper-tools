use std::process::Command;

pub mod dbtools;
pub mod fstools;
pub mod http_server;
pub mod templates;

fn restart_reload_service() {
    Command::new("systemctl")
        .arg("reload-or-restart")
        .arg(dotenv::var("PROGRAM_BASE_NAME").unwrap())
        .output()
        .unwrap();
}

pub fn init_migration(force: bool) {
    fn nginx_migration(force: bool) {
        let program_base_path = dotenv::var("PROGRAM_BASE_PATH").unwrap();
        let program_base_name = dotenv::var("PROGRAM_BASE_NAME").unwrap();
        let nginx_default_cert_path = dotenv::var("NGINX_DEFAULT_CERT_PATH").unwrap();
        // Create All Necessary Directory
        [
            &program_base_path,
            &dotenv::var("STREAM_SITES_PATH").unwrap(),
            &dotenv::var("REDIRECT_SITES_PATH").unwrap(),
            &dotenv::var("PROXY_SITES_PATH").unwrap(),
            &dotenv::var("SPA_SITES_PATH").unwrap(),
            &dotenv::var("FILE_SITES_PATH").unwrap(),
            &nginx_default_cert_path,
        ]
        .into_iter()
        .for_each(|each| std::fs::create_dir_all(each).unwrap_or_default());

        // Make {PROGRAM_BASE_NAME}.conf
        self::fstools::write_ops::write_file(
            format!("{program_base_path}/{program_base_name}.conf").as_str(),
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
                &format!("{nginx_default_cert_path}/{program_base_name}.key"),
            ])
            .args([
                "-out",
                &format!("{nginx_default_cert_path}/{program_base_name}.crt"),
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

    dotenv::dotenv().ok();

    nginx_migration(force);
    systemd_migration();
}
