use serde::{Deserialize, Serialize};
use std::{fmt, str::FromStr};
use url::Url;

use super::{
    super::{restart_reload_service, PROXY_SITES_PATH, REDIRECT_SITES_PATH},
    dbtools::crud::{
        delete_from_tbl_nginxconf, insert_tbl_nginxconf, query_existence_from_tbl_nginxconf,
        select_one_from_tbl_nginxconf,
    },
    fstools::write_ops::write_file,
    templates::http_server::{gen_proxy_templ, gen_redirect_templ},
    Command,
};

#[derive(Deserialize, Serialize)]
pub enum NginxFeatures {
    Redirect,
    Proxy,
}

impl fmt::Display for NginxFeatures {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            NginxFeatures::Proxy => write!(f, "Proxy"),
            NginxFeatures::Redirect => write!(f, "Redirect"),
        }
    }
}

impl FromStr for NginxFeatures {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Proxy" | "proxy" => Ok(Self::Proxy),
            "Redirect" | "redirect" => Ok(Self::Redirect),
            _ => Err(()),
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct NginxObj {
    server_name: String,
    target_site: String,
    feature: NginxFeatures,
}

impl NginxObj {
    pub fn new(server_name: String, target_site: String, feature: NginxFeatures) -> Self {
        Self {
            server_name,
            target_site,
            feature,
        }
    }

    pub fn verify(&self) -> Result<(), (u16, String)> {
        match Url::parse(&self.get_target_site()) {
            Ok(_) => Ok(()),
            Err(err) => Err((400, format!("Proxy Pass Arg Error: {}", err.to_string()))),
        }?;

        match query_existence_from_tbl_nginxconf(&self.server_name) {
            true => Err((400, String::from("Server Name Arg Error: Already Existed"))),
            false => Ok(()),
        }?;

        Ok(())
    }

    pub fn finish(&self) -> Result<(), (u16, String)> {
        self.write_to_disk();
        match self.make_ssl() {
            Ok(()) => Ok({
                restart_reload_service();
                insert_tbl_nginxconf(
                    self.server_name.as_ref(),
                    self.target_site.as_ref(),
                    self.feature.to_string().as_ref(),
                );
            }),
            Err(err) => Err(err),
        }?;
        Ok(())
    }

    fn write_to_disk(&self) {
        let (config, destination_file) = match self.feature {
            NginxFeatures::Proxy => (
                gen_proxy_templ(self.target_site.as_ref(), self.server_name.as_ref()),
                format!("{}/{}.conf", PROXY_SITES_PATH, &self.server_name),
            ),
            NginxFeatures::Redirect => (
                gen_redirect_templ(self.target_site.as_ref(), self.server_name.as_ref()),
                format!("{}/{}.conf", REDIRECT_SITES_PATH, &self.server_name),
            ),
        };
        write_file(&destination_file, &config, false);
    }
    fn make_ssl(&self) -> Result<(), (u16, String)> {
        match Command::new("certbot")
            .arg("--nginx")
            .arg("--agree-tos")
            .arg("--reinstall")
            .arg("--expand")
            .arg("--quiet")
            .args(["-d", &self.server_name])
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

    pub fn get_server_name(&self) -> &str {
        &self.server_name.as_str()
    }
    pub fn get_target_site(&self) -> &str {
        &self.target_site.as_str()
    }
    pub fn get_feature(&self) -> &NginxFeatures {
        &self.feature
    }
}

pub fn remove_nginx_conf(server_name: &str) -> Result<(), (u16, String)> {
    match query_existence_from_tbl_nginxconf(server_name) {
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
    match select_one_from_tbl_nginxconf(server_name).feature {
        NginxFeatures::Proxy => {
            std::fs::remove_file(format!("{}/{}.conf", PROXY_SITES_PATH, server_name))
                .or_else(|err| Err((500, err.to_string())))
        }
        NginxFeatures::Redirect => {
            std::fs::remove_file(format!("{}/{}.conf", REDIRECT_SITES_PATH, server_name))
                .or_else(|err| Err((500, err.to_string())))
        }
    }?;

    rem_ssl(server_name)?;
    restart_reload_service();
    delete_from_tbl_nginxconf(server_name);
    Ok(())
}

pub fn remake_ssl(server_name: &str) -> Result<(), (u16, String)> {
    match query_existence_from_tbl_nginxconf(server_name) {
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
