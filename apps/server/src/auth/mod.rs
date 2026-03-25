pub mod error;

use argon2::{
    password_hash::{rand_core::OsRng, Error as PasswordHashError, PasswordHasher, SaltString},
    Argon2, PasswordHash, PasswordVerifier,
};

use crate::auth::error::{AuthError, AuthResult};

pub const SESSION_DURATION_HOURS: u32 = 24 * 30;
pub const SESSION_COOKIE_NAME: &str = "SESSION";

pub fn hash_password(password: &str) -> AuthResult<String> {
    let salt = SaltString::generate(&mut OsRng);

    let argon2 = Argon2::default();

    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(AuthError::Argon2Error)?
        .to_string();

    Ok(password_hash)
}

pub fn verify_password(password: &str, password_hash: &str) -> AuthResult<bool> {
    let parsed_hash = PasswordHash::new(password_hash).map_err(AuthError::Argon2Error)?;
    let argon2 = Argon2::default();

    let result = argon2.verify_password(password.as_bytes(), &parsed_hash);

    match result {
        Ok(_) => Ok(true),
        Err(PasswordHashError::Password) => Ok(false),
        Err(err) => Err(AuthError::Argon2Error(err)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_password_returns_valid_argon2_hash() {
        let hash = hash_password("my_secret").unwrap();
        assert!(hash.starts_with("$argon2"));
    }

    #[test]
    fn test_hash_password_produces_unique_hashes() {
        let hash1 = hash_password("same_password").unwrap();
        let hash2 = hash_password("same_password").unwrap();
        assert_ne!(hash1, hash2);
    }

    #[test]
    fn test_verify_password_correct() {
        let hash = hash_password("correct_password").unwrap();
        assert!(verify_password("correct_password", &hash).unwrap());
    }

    #[test]
    fn test_verify_password_incorrect() {
        let hash = hash_password("correct_password").unwrap();
        assert!(!verify_password("wrong_password", &hash).unwrap());
    }

    #[test]
    fn test_verify_password_invalid_hash() {
        let result = verify_password("any_password", "not_a_valid_hash");
        assert!(result.is_err());
    }
}
