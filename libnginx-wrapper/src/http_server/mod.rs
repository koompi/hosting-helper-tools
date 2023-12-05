use super::{dbtools, fstools, restart_reload_service, templates, Command};
use serde::{Deserialize, Serialize};
use std::{fmt, str::FromStr};

pub mod nginx_features;
pub mod nginx_obj;
pub mod target_site;

pub async fn remove_nginx_conf(
    client: Option<libcloudflare_wrapper::Client>,
    headers: Option<libcloudflare_wrapper::HeaderMap>,
    server_name: &str,
) -> Result<(), (u16, String)> {
    let redirect_sites_path = dotenv::var("REDIRECT_SITES_PATH").unwrap();
    let proxy_sites_path = dotenv::var("PROXY_SITES_PATH").unwrap();
    let spa_sites_path = dotenv::var("SPA_SITES_PATH").unwrap();
    let file_sites_path = dotenv::var("FILE_SITES_PATH").unwrap();
    let current_file_path = format!(
        "{}/{}.conf",
        match dbtools::crud::select_one_from_tbl_nginxconf(server_name, None)
            .unwrap()
            .get_feature()
        {
            nginx_features::NginxFeatures::Redirect => redirect_sites_path,
            nginx_features::NginxFeatures::Proxy => proxy_sites_path,
            nginx_features::NginxFeatures::SPA => spa_sites_path,
            nginx_features::NginxFeatures::FileHost => file_sites_path,
            nginx_features::NginxFeatures::None => unreachable!(),
        },
        server_name
    );

    match dbtools::crud::query_existence_from_tbl_nginxconf(server_name) {
        true => Ok(()),
        false => Err((400, String::from("Item doesn't exist"))),
    }?;

    loop {
        if !fstools::read_ops::read_file(&current_file_path).contains("ssl_certificate") {
            break Ok(())?;
        }

        let certbot_res = Command::new("certbot")
            .arg("delete")
            .arg("-n")
            .args(["--cert-name", server_name])
            .output()
            .unwrap();

        break match certbot_res.status.code() {
            Some(code) => match code {
                0 => Ok(()),
                _ => {
                    let error = String::from_utf8_lossy(&certbot_res.stderr).to_string();
                    if error.starts_with("Another instance of Certbot is already running.") {
                        std::thread::sleep(std::time::Duration::from_millis(100));
                        continue;
                    }
                    Err((
                        500,
                        String::from_utf8_lossy(&certbot_res.stderr).to_string(),
                    ))
                }
            },
            None => Err((500, String::from("Terminated by a Signal"))),
        }?;
    }

    let fserror = std::fs::remove_file(current_file_path);

    dbtools::crud::delete_from_tbl_nginxconf(server_name);

    match fserror {
        Ok(()) => Ok(()),
        Err(err) => match err
            .to_string()
            .contains("No such file or directory (os error 2)")
        {
            true => {
                dbtools::db_migration(true);
                Ok(())
            }
            false => Err((500, err.to_string())),
        },
    }?;

    libcloudflare_wrapper::delete_records(client, headers, server_name).await.unwrap_or(());

    restart_reload_service();
    Ok(())
}

pub fn remake_ssl(server_name: &str) -> Result<(), (u16, String)> {
    match dbtools::crud::query_existence_from_tbl_nginxconf(server_name) {
        true => Ok(()),
        false => Err((400, String::from("Item doesn't exist"))),
    }?;

    match Command::new("certbot")
        .arg("--nginx")
        .arg("--agree-tos")
        .arg("--reinstall")
        .arg("--expand")
        .arg("--quiet")
        .args(["-d", server_name])
        .output()
    {
        Ok(out) => match &out.status.code() {
            Some(code) => match code {
                0 => Ok(()),
                _ => Err((500, String::from_utf8_lossy(&out.stderr).to_string())),
            },
            None => Err((500, String::from("Terminated by a Signal"))),
        },
        Err(err) => Err((500, err.to_string())),
    }
}
