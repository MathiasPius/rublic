use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct PluggableDomain {
    pub id: String,
    pub fqdn: String,
        
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<PluggableGroup>>
}

#[derive(Serialize, Deserialize)]
pub struct PluggableGroup {
    pub id: String,
    pub friendly_name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub domains: Option<Vec<PluggableDomain>>
}