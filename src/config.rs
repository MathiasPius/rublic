use std::env;
use std::path::PathBuf;
use regex::Regex;
use chrono::Duration;
use jwt::{Header, Algorithm, Validation};

lazy_static! {
    pub static ref ADMIN_PASSWORD: String = env::var("RUBLIC_ADMIN_PASSWORD")
        .expect("RUBLIC_ADMIN_PASSWORD was not defined!");

    pub static ref CERT_PATTERN: Regex = Regex::new(r"(\w+)([0-9]+)\.(\w+)").unwrap();

    pub static ref DATABASE_URL: String = env::var("RUBLIC_DATABASE_URL")
        .expect("RUBLIC_DATABASE_URL must be set");

    pub static ref LETSENCRYPT_ARCHIVE: PathBuf = PathBuf::from(env::var("LETSENCRYPT_ARCHIVE")
        .unwrap_or_else(|_| "/etc/letsencrypt/archive".into()));


    // JWT settings
    pub static ref JWT_ACCESS_LIFETIME: Duration = Duration::hours(1);
    pub static ref JWT_REFRESH_LIFETIME: Duration = Duration::days(30);
    pub static ref JWT_HEADER: Header = Header::new(Algorithm::HS512);

    pub static ref JWT_SHARED_SECRET: String = std::env::var("RUBLIC_SHARED_SECRET")
        .expect("RUBLIC_SHARED_SECRET environment variable is not defined!");

    pub static ref JWT_AUDIENCE: String = env::var("RUBLIC_JWT_AUDIENCE")
        .unwrap_or_else(|_| "rublic-audience".into());

    pub static ref JWT_ISSUER: String = env::var("RUBLIC_JWT_ISSUER")
        .unwrap_or_else(|_| "rublic-issuer".into());

    pub static ref JWT_VALIDATION: Validation = Validation {
        leeway: 5,
        validate_exp: true,
        validate_iat: true,
        validate_nbf: true,
        aud: Some(JWT_AUDIENCE.to_string().into()),
        iss: Some(JWT_ISSUER.to_string()),
        sub: None,
        algorithms: vec![Algorithm::HS512]
    };
}

pub fn initialize() {
    lazy_static::initialize(&ADMIN_PASSWORD);
    lazy_static::initialize(&DATABASE_URL);
    lazy_static::initialize(&LETSENCRYPT_ARCHIVE);
    lazy_static::initialize(&JWT_SHARED_SECRET);
}