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