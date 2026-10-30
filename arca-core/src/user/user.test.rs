use super::{User, UserValidationError, normalize_username, validate_password};

// Dato ficticio: estas pruebas no generan ni verifican hashes de contraseñas.
const STORED_HASH: &str = "stored-password-hash";

#[test]
fn user_normalizes_username_and_preserves_stored_data() {
    let user = User::new(u32::MAX, "  Ana_01-B  ", STORED_HASH.to_owned()).unwrap();

    assert_eq!(user.id(), u32::MAX);
    assert_eq!(user.username(), "ana_01-b");
    assert_eq!(user.password_hash(), STORED_HASH);
}

#[test]
fn user_rejects_zero_id() {
    let error = User::new(0, "ana", STORED_HASH.to_owned()).unwrap_err();

    assert_eq!(error, UserValidationError::InvalidId);
}

#[test]
fn user_rejects_missing_hash() {
    for hash in ["", " \t\n"] {
        let error = User::new(1, "ana", hash.to_owned()).unwrap_err();

        assert_eq!(error, UserValidationError::MissingPasswordHash);
    }
}

#[test]
fn debug_does_not_expose_password_hash() {
    let user = User::new(1, "ana", STORED_HASH.to_owned()).unwrap();
    let debug = format!("{user:?}");

    assert!(!debug.contains(STORED_HASH));
    assert!(!debug.contains("password_hash"));
}

#[test]
fn username_length_is_checked_after_trimming() {
    for length in [3, 32] {
        let username = "a".repeat(length);

        assert_eq!(
            normalize_username(&format!("  {username}  ")).unwrap(),
            username
        );
    }

    for length in [0, 2, 33] {
        let username = "a".repeat(length);

        assert_eq!(
            normalize_username(&format!("  {username}  ")),
            Err(UserValidationError::InvalidUsernameLength)
        );
    }
}

#[test]
fn username_rejects_spaces_symbols_and_non_ascii_characters() {
    for username in [
        "ana maria",
        "ana.maria",
        "ana@",
        "josé",
        "用户名称",
        "ana\nb",
    ] {
        assert_eq!(
            normalize_username(username),
            Err(UserValidationError::InvalidUsernameCharacters)
        );
    }
}

#[test]
fn user_applies_username_validation() {
    for username in ["ab", "ana maria"] {
        assert!(User::new(1, username, STORED_HASH.to_owned()).is_err());
    }
}

#[test]
fn password_length_uses_unicode_characters_instead_of_bytes() {
    for length in [15, 128] {
        assert_eq!(validate_password(&"🔑".repeat(length)), Ok(()));
    }

    for length in [0, 14, 129] {
        assert_eq!(
            validate_password(&"🔑".repeat(length)),
            Err(UserValidationError::InvalidPasswordLength)
        );
    }
}

#[test]
fn password_accepts_spaces_without_trimming_or_complexity_rules() {
    assert_eq!(validate_password(&" ".repeat(15)), Ok(()));
    assert_eq!(validate_password("aaaaaaaaaaaaaaa"), Ok(()));
}
