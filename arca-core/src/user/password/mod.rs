mod password;

#[path = "password.validation.rs"]
mod validation;

pub use password::hash_password;
pub use validation::{PasswordError, validate_password, verify_password};

#[cfg(test)]
#[path = "password.test.rs"]
mod test;
