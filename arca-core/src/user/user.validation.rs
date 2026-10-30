use std::{error::Error, fmt};

#[derive(Debug, PartialEq, Eq)]
pub enum UserValidationError {
    InvalidId,
    InvalidUsernameLength,
    InvalidUsernameCharacters,
    InvalidPasswordLength,
    MissingPasswordHash,
}

impl fmt::Display for UserValidationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            Self::InvalidId => "El identificador del usuario es invalido.",
            Self::InvalidUsernameLength => {
                "El nombre de usuario debe tener entre 3 y 32 caracteres."
            }
            Self::InvalidUsernameCharacters => {
                "El nombre de usuario solo admite letras a-z, números, guion y guion bajo."
            }
            Self::InvalidPasswordLength => "La contraseña debe tener entre 15 y 128 caracteres.",
            Self::MissingPasswordHash => "El hash de la contraseña es obligatorio.",
        };

        formatter.write_str(message)
    }
}

impl Error for UserValidationError {}

/// Aplica las mismas reglas para el registro y la búsqueda al iniciar sesión.
pub fn normalize_username(username: &str) -> Result<String, UserValidationError> {
    let username = username.trim().to_ascii_lowercase();

    if !username.bytes().all(|byte| {
        byte.is_ascii_lowercase() || byte.is_ascii_digit() || matches!(byte, b'_' | b'-')
    }) {
        return Err(UserValidationError::InvalidUsernameCharacters);
    }

    if !(3..=32).contains(&username.len()) {
        return Err(UserValidationError::InvalidUsernameLength);
    }

    Ok(username)
}

/// Comprueba la longitud en caracteres Unicode sin modificar la contraseña.
pub fn validate_password(password: &str) -> Result<(), UserValidationError> {
    if !(15..=128).contains(&password.chars().count()) {
        return Err(UserValidationError::InvalidPasswordLength);
    }

    Ok(())
}
