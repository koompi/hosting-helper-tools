// use crate::cloudflare_datatype::ObjErr;

use super::{Client, HeaderMap, ObjResponse};

impl ObjResponse {
    pub fn get_client() -> Client {
        reqwest::Client::builder().build().unwrap()
    }

    pub fn get_headers() -> HeaderMap {
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

    // pub fn default() -> Self {
    //     Self {
    //         errors: ObjErr,
    //         messages: todo!(),
    //         success: todo!(),
    //         result_info: todo!(),
    //         result: todo!(),
    //     }
    // }
}