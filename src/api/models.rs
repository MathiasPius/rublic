use serde_derive::{Serialize, Deserialize};
use chrono::NaiveDateTime;

#[derive(Deserialize)]
pub struct NewUserRequest {
    pub friendly_name: String
}

#[derive(Deserialize)]
pub struct NewGroupRequest {
    pub friendly_name: String
}

pub struct RawCertificate {
    pub is_private: bool,
    pub raw_data: Vec<u8>
}

#[derive(Serialize)]
pub struct Certificate {
    pub version: i32,
    pub friendly_name: String,
    pub is_private: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_before: Option<NaiveDateTime>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_after: Option<NaiveDateTime>
}

#[derive(Serialize)]
pub struct PluggableDomain {
    pub id: String,
    pub fqdn: String,
        
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<PluggableGroup>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_certs: Option<Vec<Certificate>>
}

#[derive(Serialize)]
pub struct PluggableGroup {
    pub id: String,
    pub friendly_name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub domains: Option<Vec<PluggableDomain>>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub users: Option<Vec<PluggableUser>>
}

#[derive(Serialize)]
pub struct PluggableUser {
    pub id: String,
    pub friendly_name: String,

    // This is only ever populated when a new user is created
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_key: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<PluggableGroup>>
}