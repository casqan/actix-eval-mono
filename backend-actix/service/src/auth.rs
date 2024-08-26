use std::env;
use hmac::{Hmac, KeyInit};
use sha2::{Sha256};

pub struct AuthService {}

impl AuthService {

    pub async fn sign_in_user() -> String {
        let binary: &[u8] = env::var("JWT_SECRET")
            .expect("A JWT SECRET NEEDS TO BE SET").as_ref();
        let key: Hmac<Sha256> = Hmac::new_from_slice(binary)?;
    }

    pub async fn get_salt(){

    }

    async fn generate_jwt(uuid){

    }
}