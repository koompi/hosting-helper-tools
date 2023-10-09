use super::ObjResponse;

impl ObjResponse {
    pub async fn put_record(subdomain: &str, target: &str, zone_id: &str, record_id: &str) -> Self {
        let client = Self::get_client();
        let headers = Self::get_headers();
        let request = client
            .put(format!(
                "https://api.cloudflare.com/client/v4/zones/{zone_id}/dns_records/{record_id}"
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
