use super::{HttpResponse, ResponseError};
use libnginx_wrapper::http_server::nginx_obj::NginxObj;
use serde::Serialize;

#[derive(Serialize, Debug)]
#[serde(untagged)]
pub enum BodyMessage {
    Text(String),
    DnsObj(CustomDnsStruct),
    NginxObj(NginxObj),
    HostingObj(Vec<NginxObj>),
}

#[derive(Serialize, Debug)]
pub struct ActixCustomResponse {
    pub code: u16,
    pub message: BodyMessage,
}

impl ActixCustomResponse {
    pub fn new_text(code: u16, message: String) -> Self {
        Self {
            code,
            message: BodyMessage::Text(message),
        }
    }
    pub fn new_vec_obj(code: u16, message: Vec<NginxObj>) -> Self {
        Self {
            code,
            message: BodyMessage::HostingObj(message),
        }
    }
    pub fn new_nginx_obj(code: u16, message: NginxObj) -> Self {
        Self {
            code,
            message: BodyMessage::NginxObj(message),
        }
    }
    pub fn new_dns_obj(code: u16, message: CustomDnsStruct) -> Self {
        Self {
            code,
            message: BodyMessage::DnsObj(message),
        }
    }
}

impl std::fmt::Display for ActixCustomResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl ResponseError for ActixCustomResponse {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(self)
    }
    fn status_code(&self) -> actix_web::http::StatusCode {
        actix_web::http::StatusCode::from_u16(self.code).unwrap()
    }
}

#[derive(Serialize, Debug)]
pub struct CustomDnsStruct {
    name: String,
    r#type: String,
    content: String,
    proxy: bool,
    ttl: String,
}
impl CustomDnsStruct {
    pub fn new(name: String, r#type: String, content: String, proxy: bool, ttl: String) -> Self {
        Self {
            name,
            r#type,
            content,
            proxy,
            ttl,
        }
    }
}
