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


#[derive(Serialize)]
pub struct Certificate {
    pub friendly_name: String,
    pub not_before: NaiveDateTime,
    pub not_after: NaiveDateTime
}

#[derive(Serialize)]
pub struct PluggableDomain {
    pub id: String,
    pub fqdn: String,
        
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<PluggableGroup>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificates: Option<Vec<Certificate>>
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