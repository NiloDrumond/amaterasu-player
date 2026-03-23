use argon2::password_hash;

#[derive(Debug, thiserror::Error)]
pub enum AuthError {
    #[error("Argon2 error: {0}")]
    Argon2Error(password_hash::Error),
}

pub type AuthResult<T> = Result<T, AuthError>;
