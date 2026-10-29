use argon2::{
    Argon2,
    password_hash::{PasswordHasher, SaltString, rand_core::OsRng},
};

use super::{PasswordError, validate_password};

pub fn hash_password(password: &str) -> Result<String, PasswordError> {
    use argon2::password_hash::rand_core::RngCore;

    validate_password(password)?;
    let mut salt_bytes = [0; 16];
    OsRng
        .try_fill_bytes(&mut salt_bytes)
        .map_err(|_| PasswordError::HashingFailed)?;
    let salt = SaltString::encode_b64(&salt_bytes).map_err(|_| PasswordError::HashingFailed)?;
    Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .map(|hash| hash.to_string())
        .map_err(|_| PasswordError::HashingFailed)
}
