
use rand::{Rng};
use uuid::Uuid;
use crypto::{
    pbkdf2::{pbkdf2_check, pbkdf2_simple}, 
    sha2::Sha256,
    digest::Digest
};

pub struct CryptoUtil {}

impl CryptoUtil {
    pub fn generate_key() -> String {
        rand::thread_rng()
            .sample_iter(&rand::distributions::Alphanumeric)
            .take(64)
            .collect()
    }

    pub fn hash_key(key: &str) -> String {
        let salted_key = format!("Rublic Salt Goes Here {}", key);

        pbkdf2_simple(&salted_key, 10000).unwrap()
    }

    pub fn hash_string(msg: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.input_str(msg);

        hasher.result_str()
    }

    pub fn generate_uuid() -> String {
        Uuid::new_v4().to_string()
    }

    pub fn check_key(key: &str, hashed_key: &str) -> bool {
        let salted_key = format!("Rublic Salt Goes Here {}", key);
        pbkdf2_check(&salted_key, hashed_key).unwrap_or(false)
    }
}