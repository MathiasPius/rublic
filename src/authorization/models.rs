use serde_derive::{Serialize, Deserialize};

#[derive(PartialEq, Eq, Hash, Debug, Serialize, Deserialize, Clone)]
pub struct Claim {
    pub subject: String,
    pub permission: String
}


#[derive(Serialize, Deserialize)]
pub struct Token {
    pub iat: i64,
    pub exp: i64,
    pub nbf: i64,
    pub iss: String,
    pub aud: String,
    pub claims: Vec<Claim>
}