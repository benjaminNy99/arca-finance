use std::{error::Error, fmt};

use argon2::{
    Argon2,
    password_hash::{self, PasswordHash, PasswordVerifier},
};

#[derive(Debug, PartialEq, Eq)]
pub enum PasswordError {
    InvalidLength,
    HashingFailed,
    InvalidHash,
}

impl fmt::Display for PasswordError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidLength => {
                formatter.write_str("La contraseña debe tener entre 15 y 128 caracteres.")
            }
            Self::HashingFailed => formatter.write_str("No se pudo proteger la contraseña."),
            Self::InvalidHash => formatter.write_str("No se pudo leer la credencial almacenada."),
        }
    }
}

impl Error for PasswordError {}

pub fn validate_password(password: &str) -> Result<(), PasswordError> {
    if !(15..=128).contains(&password.chars().count()) {
        return Err(PasswordError::InvalidLength);
    }
    Ok(())
}

pub fn verify_password(password: &str, hash: &str) -> Result<bool, PasswordError> {
    let parsed = PasswordHash::new(hash).map_err(|_| PasswordError::InvalidHash)?;
    match Argon2::default().verify_password(password.as_bytes(), &parsed) {
        Ok(()) => Ok(true),
        Err(password_hash::Error::Password) => Ok(false),
        Err(_) => Err(PasswordError::InvalidHash),
    }
}
