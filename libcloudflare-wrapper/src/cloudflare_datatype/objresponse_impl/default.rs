// use crate::cloudflare_datatype::ObjErr;

use super::{Client, HeaderMap, ObjResponse};

impl ObjResponse {
    pub fn get_client() -> Client {
        reqwest::Client::builder()
            .local_address("0.0.0.0".parse::<std::net::IpAddr>().unwrap())
            .build()
            .unwrap()
    }

    pub fn get_headers_cloudflare() -> HeaderMap {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("Content-Type", "application/json".parse().unwrap());
        let bearer = match dotenv::var("AUTHTOKEN") {
            Ok(data) => data,
            Err(_) => {
                eprintln!("Warning: Missing Bearer Authtoken\nWarning: If you're using offline without any SSL or IPCHECK or Cloudflare, Ignore this.");
                String::new()
            }
        };
        headers.insert("Authorization", bearer.parse().unwrap());
        headers
    }

    pub fn get_headers_default() -> HeaderMap {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("Content-Type", "application/json".parse().unwrap());
        headers
    }
}
