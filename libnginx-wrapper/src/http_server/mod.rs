use serde::{Deserialize, Serialize};
use url::Url;

use super::{
    dbtools::crud::{
        delete_from_tbl_nginxconf, insert_tbl_nginxconf, query_existence_from_tbl_nginxconf,
    },
    fstools::write_ops::write_file,
    restart_reload_service,
    templates::http_server::gen_templ,
    Command,
};

#[derive(Default, Deserialize, Serialize)]
pub struct NginxObj {
    server_name: String,
    proxy_pass: String,
}

impl NginxObj {
    pub fn new(server_name: String, proxy_pass: String) -> Self {
        Self {
            server_name,
            proxy_pass,
        }
    }

    pub fn verify(&self) -> Result<(), String> {
        match Url::parse(&self.get_proxy_pass()) {
            Ok(_) => Ok(()),
            Err(err) => Err(format!("Proxy Pass Arg Error: {}", err.to_string())),
        }?;

        match query_existence_from_tbl_nginxconf(&self.server_name) {
            true => Err(String::from("Server Name Arg Error: Already Existed")),
            false => Ok(())
        }?;

        Ok(())
    }

    pub fn finish(&self) -> Result<(), String> {
        self.write_to_disk();
        match self.make_ssl() {
            Ok(()) => Ok({
                restart_reload_service();
                insert_tbl_nginxconf(self.server_name.as_ref(), self.proxy_pass.as_ref());
            }),
            Err(err) => Err(err),
        }?;
        Ok(())
    }

    fn write_to_disk(&self) {
        let config = gen_templ(self.proxy_pass.as_ref(), self.server_name.as_ref());
        let destination_file = format!("/etc/nginx/sites-enabled/{}.conf", &self.server_name);
        write_file(&destination_file, &config, false);
    }
    fn make_ssl(&self) -> Result<(), String> {
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
                    _ => Err(String::from_utf8_lossy(&out.stderr).to_string()),
                },
                None => Err(String::from("Terminated by a Signal")),
            },
            Err(err) => Err(err.to_string()),
        }
    }

    pub fn get_server_name(&self) -> &str {
        &self.server_name.as_str()
    }
    pub fn get_proxy_pass(&self) -> &str {
        &self.proxy_pass.as_str()
    }
}

pub fn remove_nginx_conf(server_name: &str) -> Result<(), String> {
    fn rem_ssl(server_name: &str) -> Result<(), String> {
        match Command::new("certbot")
            .arg("delete")
            .arg("-n")
            .args(["--cert-name", server_name])
            .output()
        {
            Ok(out) => match &out.status.code() {
                Some(code) => match code {
                    0 => Ok(()),
                    _ => Err(String::from_utf8_lossy(&out.stderr).to_string()),
                },
                None => Err(String::from("Terminated by a Signal")),
            },
            Err(err) => Err(err.to_string()),
        }
    }

    std::fs::remove_file(format!("/etc/nginx/sites-enabled/{}.conf", server_name))
        .or_else(|err| Err(err.to_string()))?;
    rem_ssl(server_name)?;
    restart_reload_service();
    delete_from_tbl_nginxconf(server_name);
    Ok(())
}

pub fn remake_ssl(server_name: &str) -> Result<(), String> {
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
                _ => Err(String::from_utf8_lossy(&out.stderr).to_string()),
            },
            None => Err(String::from("Terminated by a Signal")),
        },
        Err(err) => Err(err.to_string()),
    }
}
