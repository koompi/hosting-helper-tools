use super::ObjResponse;

impl ObjResponse {
    pub async fn get_zone(domain_name: Option<&str>, pending_status: bool) -> Self {
        let client = Self::get_client();
        let headers = Self::get_headers();
        let url = match domain_name {
            Some(zone) => {
                format!("https://api.cloudflare.com/client/v4/zones?order=name&name={zone}")
            }
            None => format!(
                "https://api.cloudflare.com/client/v4/zones?order=name&per_page={}",
                dotenv::var("QUERY_LIMIT").unwrap()
            ),
        };

        let url = match pending_status {
            true => url + "&status=pending",
            false => url,
        };

        let request = client
            .request(reqwest::Method::GET, url)
            .headers(headers.clone());

        let response = request.send().await.unwrap();
        let body = response.text().await.unwrap();

        serde_json::from_str::<Self>(&body).unwrap()
    }

    pub async fn get_records(zone_id: &str, full_domain_name: Option<&str>) -> Self {
        let client = Self::get_client();
        let headers = Self::get_headers();
        let url = match full_domain_name {
            Some(full_domain_name) => format!("https://api.cloudflare.com/client/v4/zones/{zone_id}/dns_records?name={full_domain_name}"),
            None => format!("https://api.cloudflare.com/client/v4/zones/{zone_id}/dns_records")
        };

        let request = client
            .request(reqwest::Method::GET, url)
            .headers(headers.clone());

        let response = request.send().await.unwrap();
        let body = response.text().await.unwrap();

        serde_json::from_str::<Self>(&body).unwrap()
    }
}
