pub mod password;

mod user;

#[path = "user.validation.rs"]
mod validation;

pub use user::User;
pub use validation::{UserValidationError, normalize_username, validate_password};

#[cfg(test)]
#[path = "user.test.rs"]
mod test;
