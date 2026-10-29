use super::{PasswordError, hash_password, validate_password, verify_password};

#[test]
fn hashes_have_distinct_salts_and_verify_credentials() {
    let password = "test-only-password-123";
    let first = hash_password(password).unwrap();
    let second = hash_password(password).unwrap();
    assert_ne!(first, second);
    assert!(first.starts_with("$argon2id$"));
    assert!(verify_password(password, &first).unwrap());
    assert!(!verify_password("wrong-password", &first).unwrap());
}

#[test]
fn rejects_invalid_password_and_corrupt_hash() {
    assert!(matches!(
        hash_password("short"),
        Err(PasswordError::InvalidLength)
    ));
    assert!(matches!(
        verify_password("test", "invalid"),
        Err(PasswordError::InvalidHash)
    ));
}

#[test]
fn password_length_uses_unicode_characters_instead_of_bytes() {
    for length in [15, 128] {
        assert_eq!(validate_password(&"🔑".repeat(length)), Ok(()));
    }
    for length in [0, 14, 129] {
        assert_eq!(
            validate_password(&"🔑".repeat(length)),
            Err(PasswordError::InvalidLength)
        );
    }
}

#[test]
fn password_accepts_spaces_without_trimming_or_complexity_rules() {
    assert_eq!(validate_password(&" ".repeat(15)), Ok(()));
    assert_eq!(validate_password("aaaaaaaaaaaaaaa"), Ok(()));
}
