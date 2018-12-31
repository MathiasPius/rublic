use serde_derive::{Serialize, Deserialize};
use crate::schema::{certificates};


#[derive(Queryable, Insertable, Serialize, Deserialize)]
#[table_name = "domains"]
pub struct Domain {
    pub id: String,
    pub fqdn: String
}

#[derive(Queryable, Insertable, Serialize, Deserialize)]
#[table_name = "domain_groups"]
pub struct DomainGroup {
    pub id: String,
    pub friendly_name: String
}

/*
#[derive(Serialize, Deserialize)]
pub struct NewCertificateEntry {
    pub filepath: String
}

#[derive(Serialize, Deserialize)]
pub struct GetCertificateById {
    pub id: String
}
*/