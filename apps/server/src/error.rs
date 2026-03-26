use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

use crate::auth::error::AuthError;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error(transparent)]
    Auth(#[from] AuthError),

    #[error(transparent)]
    Validation(#[from] garde::Error),

    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Not found")]
    NotFound,

    #[error("Internal server error")]
    Internal(#[from] anyhow::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            AppError::Validation(e) => (StatusCode::BAD_REQUEST, e.message()),
            AppError::Auth(e) => match e {
                AuthError::UserNotFound | AuthError::PasswordDoesntMatch => {
                    (StatusCode::UNAUTHORIZED, "Wrong email or password")
                }
                AuthError::EmailAlreadyTaken => (StatusCode::CONFLICT, "Email already in use"),
                AuthError::Argon2Error(e) => {
                    tracing::error!("Auth error: {:?}", e);
                    (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
                }
                AuthError::MissingSessionCookie
                | AuthError::SessionNotFound(_)
                | AuthError::ExpiredSession
                | AuthError::UserNotFoundForSesssion(_) => {
                    (StatusCode::UNAUTHORIZED, "Unauthorized")
                }
            },
            AppError::Database(e) => {
                tracing::error!("Database error: {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "Database error")
            }
            AppError::NotFound => (StatusCode::NOT_FOUND, "Resource not found"),
            AppError::Internal(e) => {
                tracing::error!("Internal server error: {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
            }
        };

        let body = Json(json!({
            "error": message,
        }));

        (status, body).into_response()
    }
}

pub type AppResult<T> = Result<T, AppError>;
