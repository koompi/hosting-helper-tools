use serde::Deserialize;

#[derive(Debug, Deserialize, Default)]
pub struct ObjMeta {
    pub auto_added: bool,
    pub managed_by_argo_tunnel: bool,
    pub managed_by_apps: bool,
    pub source: String,
}

#[derive(Debug, Deserialize, Default)]
pub struct RecordsRes {
    pub id: String,
    pub zone_id: String,
    pub zone_name: String,
    pub name: String,
    pub r#type: String,
    pub content: String,
    pub proxied: bool,
    pub proxiable: bool,
    pub ttl: u32,
    pub locked: bool,
    pub meta: ObjMeta,
    pub comment: Option<String>,
    pub tags: Vec<String>,
    pub created_on: String,
    pub modified_on: String,
}
