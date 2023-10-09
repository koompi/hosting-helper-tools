use super::ObjResponse;

impl ObjResponse {
    pub async fn post_zone(domain_name: &str) -> Self {
        let client = Self::get_client();
        let headers = Self::get_headers();
        let request = client
            .post("https://api.cloudflare.com/client/v4/zones")
            .headers(headers.clone())
            .body(
                serde_json::json!({
                    "account": {
                        "id": format!("{}", dotenv::var("CLOUDFLARE_ACC_ID").unwrap())
                    },
                    "name": format!("{}", domain_name),
                    "type": "full"
                })
                .to_string(),
            );

        let response = request.send().await.unwrap();
        let body = response.text().await.unwrap();

        serde_json::from_str::<Self>(&body).unwrap()
    }

    pub async fn post_record(subdomain: &str, target: &str, zone_id: &str) -> Self {
        let client = Self::get_client();
        let headers = Self::get_headers();
        let request = client
            .post(format!(
                "https://api.cloudflare.com/client/v4/zones/{zone_id}/dns_records"
            ))
            .headers(headers.clone())
            .body(
                serde_json::json!({
                  "content": format!("{}", target),
                  "name": format!("{}", subdomain),
                  "proxied": false,
                  "type": "A"
                })
                .to_string(),
            );

        let response = request.send().await.unwrap();
        let body = response.text().await.unwrap();

        serde_json::from_str::<Self>(&body).unwrap()
    }
}
