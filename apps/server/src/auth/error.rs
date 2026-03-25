use argon2::password_hash;

#[derive(Debug, thiserror::Error)]
pub enum AuthError {
    #[error("Email already in use")]
    EmailAlreadyTaken,

    #[error("Argon2 error: {0}")]
    Argon2Error(password_hash::Error),

    #[error("User not found")]
    UserNotFound,

    #[error("Password doesn't match")]
    PasswordDoesntMatch,
}

pub type AuthResult<T> = Result<T, AuthError>;
