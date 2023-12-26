// use crate::cloudflare_datatype::ObjErr;

use super::{Client, HeaderMap, ObjResponse};

impl ObjResponse {
    pub fn get_client() -> Client {
        reqwest::Client::builder().local_address("0.0.0.0".parse::<std::net::IpAddr>().unwrap()).build().unwrap()
    }

    pub fn get_headers_cloudflare() -> HeaderMap {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("Content-Type", "application/json".parse().unwrap());
        headers.insert(
            "Authorization",
            format!("Bearer {}", dotenv::var("AUTHTOKEN").unwrap())
                .parse()
                .unwrap(),
        );
        headers
    }

    pub fn get_headers_default() -> HeaderMap {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("Content-Type", "application/json".parse().unwrap());
        headers
    }
}