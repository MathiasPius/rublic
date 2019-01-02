
use rand::{Rng};
use uuid::Uuid;
use crypto::{
    pbkdf2::{/*pbkdf2_check,*/ pbkdf2_simple}, 
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

    pub fn hash_key(key: &String) -> String {
        let salted_key = format!("Rublic Salt Goes Here {}", key);

        pbkdf2_simple(&salted_key, 10000).unwrap()
    }

    pub fn hash_string(msg: &String) -> String {
        let mut hasher = Sha256::new();
        hasher.input_str(msg);

        hasher.result_str()
    }

    pub fn generate_uuid() -> String {
        Uuid::new_v4().to_string()
    }

/*
    pub fn check_key(key: &String, hashed_key: &String) -> bool {
        pbkdf2_check(key, hashed_key).unwrap()
    }
*/
}