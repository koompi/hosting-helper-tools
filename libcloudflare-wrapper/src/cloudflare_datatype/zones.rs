use super::Deserialize;

#[derive(Deserialize, Default)]
pub struct ObjAcc {
    pub id: Option<String>,
    pub name: Option<String>,
}

#[derive(Deserialize, Default)]
pub struct ObjTenant(ObjAcc);

#[derive(Deserialize, Default)]
pub struct ObjTenantUnit {
    pub id: Option<String>
}

#[derive(Deserialize, Default)]
pub struct ObjPlan {
    pub id: String,
    pub name: String,
    pub price: u16,
    pub currency: String,
    pub frequency: String,
    pub is_subscribed: bool,
    pub can_subscribe: bool,
    pub legacy_id: String,
    pub legacy_discount: bool,
    pub externally_managed: bool
}

#[derive(Deserialize, Default)]
pub struct ObjOwner {
    pub id: Option<String>,
    pub name: Option<String>,
    pub r#type: Option<String>,
    pub email: Option<String>
}

#[derive(Deserialize, Default)]
pub struct ObjMeta {
    pub cdn_only: Option<bool>,
    pub custom_certificate_quota: u8,
    pub dns_only: Option<bool>,
    pub foundation_dns: Option<bool>,
    pub page_rule_quota: u8,
    pub phishing_detected: bool,
    pub step: u8,
}

#[derive(Deserialize, Default)]
pub struct ZoneRes {
    pub id: String,
    pub name: String,
    pub status: String,
    pub paused: bool,
    pub r#type: String,
    pub development_mode: i32,
    pub name_servers: Vec<String>,
    pub original_name_servers: Option<Vec<String>>,
    pub original_registrar: Option<String>,
    pub original_dnshost: Option<String>,
    pub modified_on: String,
    pub created_on: String,
    pub activated_on: Option<String>,
    pub meta: ObjMeta,
    pub owner: ObjOwner,
    pub account: Option<ObjAcc>,
    pub tenant: ObjTenant,
    pub tenant_unit: ObjTenantUnit,
    pub permissions: Vec<String>,
    pub plan: ObjPlan,
    pub vanity_name_servers: Option<Vec<String>>
}