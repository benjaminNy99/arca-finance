use std::fmt;

use super::validation::{UserValidationError, normalize_username};

/// Usuario persistido. El hash debe generarse mediante la biblioteca de autenticación.
pub struct User {
    id: u32,
    username: String,
    password_hash: String,
}

impl User {
    /// Construye un usuario con su identificador y un hash previamente generado.
    /// No genera hashes ni comprueba su formato o las credenciales.
    pub fn new(
        id: u32,
        username: &str,
        password_hash: String,
    ) -> Result<Self, UserValidationError> {
        if id == 0 {
            return Err(UserValidationError::InvalidId);
        }

        let username = normalize_username(username)?;

        if password_hash.trim().is_empty() {
            return Err(UserValidationError::MissingPasswordHash);
        }

        Ok(Self {
            id,
            username,
            password_hash,
        })
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn username(&self) -> &str {
        &self.username
    }

    /// Hash destinado al almacenamiento y a la verificación; no incluirlo en logs.
    pub fn password_hash(&self) -> &str {
        &self.password_hash
    }
}

impl fmt::Debug for User {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("User")
            .field("id", &self.id)
            .field("username", &self.username)
            .finish_non_exhaustive()
    }
}
