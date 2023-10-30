use url::Url;

use super::{
    dbtools::crud::{
        insert_tbl_nginxconf, query_existence_from_tbl_nginxconf, select_one_from_tbl_nginxconf,
        update_target_tbl_nginxconf,
    },
    fstools::write_ops::write_file,
    nginx_features::NginxFeatures,
    restart_reload_service,
    target_site::TargetSite,
    templates::http_server::{
        gen_filehost_templ, gen_proxy_templ, gen_redirect_templ, gen_spa_templ,
    },
    Command, Deserialize, Serialize,
};

#[derive(Deserialize, Serialize, Default, Debug)]
pub struct NginxObj {
    server_name: String,
    target_site: TargetSite,
    feature: NginxFeatures,
}

impl NginxObj {
    pub fn get_server_name(&self) -> &str {
        &self.server_name.as_str()
    }
    pub fn get_target_site(&self) -> &TargetSite {
        &self.target_site
    }
    pub fn get_feature(&self) -> &NginxFeatures {
        &self.feature
    }
    pub fn get_target_site_protocol(&self) -> String {
        url::Url::parse(match &self.target_site {
            TargetSite::Single(singlesite) => &singlesite,
            TargetSite::Multiple(multisite) => multisite.iter().next().unwrap(),
            _ => unreachable!(),
        })
        .unwrap()
        .scheme()
        .to_string()
    }

    pub fn new(
        server_name: String,
        target_site: TargetSite,
        feature: NginxFeatures,
    ) -> Result<Self, (u16, String)> {
        let data = NginxObj {
            server_name,
            target_site,
            feature,
        };
        data.verify()?;
        Ok(data)
    }

    pub(crate) fn new_unchecked(
        server_name: String,
        target_site: TargetSite,
        feature: NginxFeatures,
    ) -> Self {
        NginxObj {
            server_name,
            target_site,
            feature,
        }
    }

    pub async fn update_target(
        server_name: &str,
        target_site: TargetSite,
    ) -> Result<(), (u16, String)> {
        let old_obj = match select_one_from_tbl_nginxconf(server_name) {
            Ok(obj) => Ok(obj),
            Err(_) => Err((400, String::from("Server Name doesn't exist"))),
        }?;
        let target_site_str = target_site.to_string();

        Self::new(old_obj.server_name, target_site, old_obj.feature)?
            .finish(false)
            .await?;

        update_target_tbl_nginxconf(server_name, &target_site_str);

        Ok(())
    }

    pub async fn setup_cloudflare(
        &self,
        switch: bool,
        ip_check: bool,
    ) -> Result<(), (u16, String)> {
        let client = libcloudflare_wrapper::get_client();

        if switch {
            libcloudflare_wrapper::setup_domain(&self.get_server_name()).await
        } else if ip_check {
            let our_ip = libcloudflare_wrapper::get_public_ip(&client, None).await;
            let domain_ip =
                libcloudflare_wrapper::get_public_ip(&client, Some(&self.get_server_name())).await;
            match our_ip != domain_ip {
                true => Err((
                    500,
                    format!(
                        "The Public IP {} of the Domain {} does not match Server Public IP {}",
                        domain_ip,
                        &self.get_server_name(),
                        our_ip
                    ),
                )),
                false => Ok(()),
            }
        } else {
            Ok(())
        }
    }

    pub async fn finish(&self, ssl: bool) -> Result<(), (u16, String)> {
        let destination_file = self.write_to_disk()?;

        if ssl {
            match self.make_ssl() {
                Ok(()) => Ok(()),
                Err(err) => Err({
                    std::fs::remove_file(&destination_file).unwrap();
                    err
                }),
            }?;
        }

        Ok({
            restart_reload_service();
            insert_tbl_nginxconf(
                self.server_name.as_ref(),
                self.target_site.to_string().as_ref(),
                self.feature.to_string().as_ref(),
            );
        })
    }

    pub fn verify(&self) -> Result<(), (u16, String)> {
        fn parse_target_site(singletarget: &str) -> Result<(), (u16, String)> {
            match Url::parse(singletarget) {
                Ok(_) => Ok(()),
                Err(err) => Err((400, format!("Target Site Arg Error: {}", err.to_string()))),
            }
        }

        match query_existence_from_tbl_nginxconf(&self.server_name) {
            true => Err((400, String::from("Server Name Arg Error: Already Existed"))),
            false => Ok(()),
        }?;

        match &self.feature {
            NginxFeatures::Redirect => match self.get_target_site() {
                TargetSite::Single(singletarget) => parse_target_site(singletarget),
                TargetSite::Multiple(_) => {
                    Err((400, format!("Target Site Arg Error: Too many Args")))
                }
                _ => unreachable!(),
            }?,
            NginxFeatures::Proxy => match self.get_target_site() {
                TargetSite::Single(singletarget) => parse_target_site(&singletarget),
                TargetSite::Multiple(multisite) => {
                    let sample_protocol = Url::parse(multisite.iter().next().unwrap()).unwrap();
                    let sample_protocol = sample_protocol.scheme();
                    match multisite.iter().any(|each| {
                        let cmp_protocol = Url::parse(each).unwrap();
                        let cmp_protocol = cmp_protocol.scheme();
                        cmp_protocol != sample_protocol
                    }) {
                        true => Err((
                            400,
                            String::from("Server Name Arg Error: Mismatch Protocol in Target Site"),
                        )),
                        false => Ok(()),
                    }?;
                    multisite
                        .iter()
                        .try_for_each(|each| parse_target_site(&each))
                }
                _ => unreachable!(),
            }?,
            NginxFeatures::SPA | NginxFeatures::FileHost => {
                match std::path::Path::new(&match &self.target_site {
                    TargetSite::Single(singletarget) => Ok(singletarget),
                    TargetSite::Multiple(_) => {
                        Err((400, format!("Target Site Arg Error: Too many Args")))
                    }
                    _ => unreachable!(),
                }?)
                .is_absolute()
                {
                    true => Ok(()),
                    false => Err((400, format!("Target Site Arg Error: Path not Absolute"))),
                }?
            }
            _ => unreachable!(),
        }

        Ok(())
    }

    fn write_to_disk(&self) -> Result<String, (u16, String)> {
        let redirect_sites_path = dotenv::var("REDIRECT_SITES_PATH").unwrap();
        let proxy_sites_path = dotenv::var("PROXY_SITES_PATH").unwrap();
        let spa_sites_path = dotenv::var("SPA_SITES_PATH").unwrap();
        let file_sites_path = dotenv::var("FILE_SITES_PATH").unwrap();

        let (config, destination_file) = match self.feature {
            NginxFeatures::Proxy => (
                gen_proxy_templ(
                    match &self.target_site {
                        TargetSite::Single(singlesite) => vec![singlesite.to_string()],
                        TargetSite::Multiple(multisite) => multisite.to_vec(),
                        _ => unreachable!(),
                    },
                    self.server_name.as_ref(),
                    &self.get_target_site_protocol(),
                ),
                format!("{}/{}.conf", proxy_sites_path, &self.server_name),
            ),
            NginxFeatures::Redirect => (
                gen_redirect_templ(
                    self.target_site.get_single_site(),
                    self.server_name.as_ref(),
                ),
                format!("{}/{}.conf", redirect_sites_path, &self.server_name),
            ),
            NginxFeatures::SPA => (
                gen_spa_templ(
                    self.target_site.get_single_site(),
                    self.server_name.as_ref(),
                ),
                format!("{}/{}.conf", spa_sites_path, &self.server_name),
            ),
            NginxFeatures::FileHost => (
                gen_filehost_templ(
                    self.target_site.get_single_site(),
                    self.server_name.as_ref(),
                ),
                format!("{}/{}.conf", file_sites_path, &self.server_name),
            ),
            _ => unreachable!(),
        };
        write_file(&destination_file, &config, false)?;
        Ok(destination_file)
    }

    fn make_ssl(&self) -> Result<(), (u16, String)> {
        loop {
            let certbot_res = Command::new("certbot")
                .arg("--nginx")
                .arg("--agree-tos")
                .args(["-m", "pi@koompi.com"])
                .arg("--reinstall")
                .arg("--expand")
                .args(["-d", &self.server_name])
                .output()
                .unwrap();

            let res = match certbot_res.status.code() {
                Some(code) => match code {
                    0 => Ok(()),
                    _ => match String::from_utf8_lossy(&certbot_res.stderr)
                        .starts_with("Another instance of Certbot is already running.")
                    {
                        true => {
                            std::thread::sleep(std::time::Duration::from_millis(10));
                            continue;
                        }
                        false => Err((
                            500,
                            String::from_utf8_lossy(&certbot_res.stderr).to_string(),
                        )),
                    },
                },
                None => Err((500, String::from("Terminated by a Signal"))),
            };

            break res;
        }
    }
}
