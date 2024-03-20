use libnginx_wrapper::http_server::nginx_features::NginxFeatures;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct UpdateNginxQueryString {
    #[serde(default = "def_bool::<true>")]
    ssl: bool,
}
impl UpdateNginxQueryString {
    pub fn get_ssl(&self) -> &bool {
        &self.ssl
    }
}

#[derive(Debug, Deserialize)]
pub struct AddNginxQueryString {
    #[serde(default = "def_bool::<true>")]
    cloudflare: bool,
    #[serde(default = "def_bool::<true>")]
    ssl: bool,
    #[serde(default = "def_bool::<true>")]
    enom: bool,
    #[serde(default = "def_bool::<true>")]
    ipcheck: bool,
}
impl AddNginxQueryString {
    pub fn get_cloudflare_bool(&self) -> &bool {
        &self.cloudflare
    }
    pub fn get_ssl_bool(&self) -> &bool {
        &self.ssl
    }
    pub fn get_enom_bool(&self) -> &bool {
        &self.enom
    }
    pub fn get_ipcheck_bool(&self) -> &bool {
        &self.ipcheck
    }
}

const fn def_bool<const V: bool>() -> bool {
    V
}

#[derive(Deserialize)]
pub struct ListNginxQueryString {
    feature: Option<NginxFeatures>,
    server_name: Option<String>
}
impl ListNginxQueryString {
    pub fn get_feature(&self) -> &Option<NginxFeatures> {
        &self.feature
    }
    pub fn get_server_name(&self) -> &Option<String> {
        &self.server_name
    }
}