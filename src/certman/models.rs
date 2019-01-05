use chrono::NaiveDateTime;

pub struct SingleCertificate {
    pub raw_data: Vec<u8>
}


pub struct PublicCertificate {
    pub not_before: NaiveDateTime,
    pub not_after: NaiveDateTime
}

pub struct PrivateKey {

}

pub enum PemFileContents {
    PrivateKey(PrivateKey),
    PublicCertificate(PublicCertificate)
}

/*
#[derive(Identifiable, Queryable, Insertable, Associations, Debug)]
#[primary_key(domain_id, id, friendly_name)]
pub struct Certificate {
    pub id: i32,
    pub domain_id: String,
    pub friendly_name: String,
    pub path: String,
    pub is_private: bool,
    pub not_before: Option<NaiveDateTime>,
    pub not_after: Option<NaiveDateTime>
}

*/