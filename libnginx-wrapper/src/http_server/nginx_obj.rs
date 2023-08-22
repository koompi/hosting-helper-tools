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
    Command, Deserialize, Serialize, FILE_SITES_PATH, PROXY_SITES_PATH, REDIRECT_SITES_PATH,
    SPA_SITES_PATH,
};

#[derive(Deserialize, Serialize, Default)]
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

    pub fn update_target(server_name: &str, target_site: TargetSite) -> Result<(), (u16, String)> {
        let old_obj = match select_one_from_tbl_nginxconf(server_name) {
            Ok(obj) => Ok(obj),
            Err(_) => Err((400, String::from("Server Name doesn't exist"))),
        }?;
        let target_site_str = target_site.to_string();

        Self::new(old_obj.server_name, target_site, old_obj.feature)?.finish()?;

        update_target_tbl_nginxconf(server_name, &target_site_str);

        Ok(())
    }

    pub fn finish(&self) -> Result<(), (u16, String)> {
        let destination_file = self.write_to_disk();
        match self.make_ssl() {
            Ok(()) => Ok({   
                println!("OK");
                restart_reload_service();
                insert_tbl_nginxconf(
                    self.server_name.as_ref(),
                    self.target_site.to_string().as_ref(),
                    self.feature.to_string().as_ref(),
                );
            }),
            Err(err) => Err({
                println!("err");
                std::fs::remove_file(destination_file).unwrap();
                err
            }),
        }?;
        Ok(())
    }

    fn verify(&self) -> Result<(), (u16, String)> {
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
                let cpath = match &self.target_site {
                    TargetSite::Single(singletarget) => Ok(singletarget),
                    TargetSite::Multiple(_) => {
                        Err((400, format!("Target Site Arg Error: Too many Args")))
                    }
                    _ => unreachable!(),
                }?;

                match std::path::Path::new(cpath).is_absolute() {
                    true => Ok(()),
                    false => Err((400, format!("Target Site Arg Error: Path not Absolute"))),
                }?
            }
            _ => unreachable!(),
        }

        Ok(())
    }

    fn write_to_disk(&self) -> String {
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
                format!("{}/{}.conf", PROXY_SITES_PATH, &self.server_name),
            ),
            NginxFeatures::Redirect => (
                gen_redirect_templ(
                    self.target_site.get_single_site(),
                    self.server_name.as_ref(),
                ),
                format!("{}/{}.conf", REDIRECT_SITES_PATH, &self.server_name),
            ),
            NginxFeatures::SPA => (
                gen_spa_templ(
                    self.target_site.get_single_site(),
                    self.server_name.as_ref(),
                ),
                format!("{}/{}.conf", SPA_SITES_PATH, &self.server_name),
            ),
            NginxFeatures::FileHost => (
                gen_filehost_templ(
                    self.target_site.get_single_site(),
                    self.server_name.as_ref(),
                ),
                format!("{}/{}.conf", FILE_SITES_PATH, &self.server_name),
            ),
            _ => unreachable!(),
        };
        // println!("{config}");
        write_file(&destination_file, &config, false);
        destination_file
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
}
