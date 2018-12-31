
use rand::{Rng};
use crypto::{pbkdf2::{/*pbkdf2_check,*/ pbkdf2_simple}};

pub struct CryptUtil {}

impl CryptUtil {
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

/*
    pub fn check_key(key: &String, hashed_key: &String) -> bool {
        pbkdf2_check(key, hashed_key).unwrap()
    }
*/
}