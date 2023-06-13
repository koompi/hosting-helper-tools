use super::{
    dbtools::crud::insert_tbl_nginxconf, fstools::write_ops::write_file,
    templates::http_server::gen_templ, Command,
};

#[derive(Default)]
pub struct NginxObj {
    server_name: String,
    proxy_pass: String,
}

impl NginxObj {
    pub fn new(server_name: String, proxy_pass: String) -> Result<Self, String> {
        Ok(Self {
            server_name,
            proxy_pass,
        })
    }
    pub fn finish(&self) -> Result<(), String> {
        self.write_to_disk();
        match self.make_ssl() {
            Ok(()) => Ok(insert_tbl_nginxconf(
                self.server_name.as_ref(),
                self.proxy_pass.as_ref(),
            )),
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
            Ok(o) => match o.status.code() {
                Some(code) => match code {
                    0 => Ok(()),
                    _ => Err(String::from_utf8_lossy(&o.stderr).to_string()),
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
