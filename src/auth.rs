use argon2::{
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
    password_hash::{SaltString, rand_core::OsRng},
};
use serde::{Deserialize, Serialize};

use crate::Error;
use crate::Result;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AuthClaim {
    pub user_id: u64,
    pub uuid: uuid::Uuid,
    pub exp: usize,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ResetClaim {
    pub user_id: u64,
    pub email: String,
    pub exp: usize,
}

pub fn hash_password(password: &str) -> Result<String> {
    Ok(Argon2::default()
        .hash_password(password.as_bytes(), &SaltString::generate(&mut OsRng))
        .map_err(|_| Error::Unauthorized)?
        .to_string())
}

pub fn verify_password(password: &str, hash: &str) -> bool {
    let hash = match PasswordHash::new(hash) {
        Ok(hash) => hash,
        Err(_) => return false,
    };

    Argon2::default()
        .verify_password(password.as_bytes(), &hash)
        .is_ok()
}
