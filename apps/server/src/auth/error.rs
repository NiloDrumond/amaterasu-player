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

    #[error("Missing session cookie on protected route")]
    MissingSessionCookie,

    #[error("Session not found for cookie: {0}")]
    SessionNotFound(String),

    #[error("Session is no longer valid")]
    ExpiredSession,

    #[error("User not found for session: {0}")]
    UserNotFoundForSesssion(String)
}

pub type AuthResult<T> = Result<T, AuthError>;
