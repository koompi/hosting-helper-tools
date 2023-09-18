use super::{
    dbtools, fstools, restart_reload_service, templates, Command};
use serde::{Deserialize, Serialize};
use std::{fmt, str::FromStr};

pub mod nginx_features;
pub mod nginx_obj;
pub mod target_site;

pub fn remove_nginx_conf(server_name: &str) -> Result<(), (u16, String)> {
    let redirect_sites_path = dotenv::var("REDIRECT_SITES_PATH").unwrap();
    let proxy_sites_path = dotenv::var("PROXY_SITES_PATH").unwrap();
    let spa_sites_path = dotenv::var("SPA_SITES_PATH").unwrap();
    let file_sites_path = dotenv::var("FILE_SITES_PATH").unwrap();

    match dbtools::crud::query_existence_from_tbl_nginxconf(server_name) {
        true => Ok(()),
        false => Err((400, String::from("Item doesn't exist"))),
    }?;

    fn rem_ssl(server_name: &str) -> Result<(), (u16, String)> {
        match Command::new("certbot")
            .arg("delete")
            .arg("-n")
            .args(["--cert-name", server_name])
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
    match dbtools::crud::select_one_from_tbl_nginxconf(server_name).unwrap().get_feature() {
        nginx_features::NginxFeatures::Proxy => {
            std::fs::remove_file(format!("{}/{}.conf", proxy_sites_path, server_name))
                .or_else(|err| Err((500, err.to_string())))
        }
        nginx_features::NginxFeatures::Redirect => {
            std::fs::remove_file(format!("{}/{}.conf", redirect_sites_path, server_name))
                .or_else(|err| Err((500, err.to_string())))
        }
        nginx_features::NginxFeatures::SPA => {
            std::fs::remove_file(format!("{}/{}.conf", spa_sites_path, server_name))
                .or_else(|err| Err((500, err.to_string())))
        }
        nginx_features::NginxFeatures::FileHost => {
            std::fs::remove_file(format!("{}/{}.conf", file_sites_path, server_name))
                .or_else(|err| Err((500, err.to_string())))
        }
        _ => unreachable!()
    }?;

    rem_ssl(server_name)?;
    restart_reload_service();
    dbtools::crud::delete_from_tbl_nginxconf(server_name);
    Ok(())
}

pub fn remake_ssl(server_name: &str) -> Result<(), (u16, String)> {
    match  dbtools::crud::query_existence_from_tbl_nginxconf(server_name) {
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
