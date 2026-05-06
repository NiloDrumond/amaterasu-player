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

    #[error("Album is not empty")]
    AlbumNotEmpty,

    #[error("Artist is not empty")]
    ArtistNotEmpty,

    #[error("Range not satisfiable")]
    RangeNotSatisfiable,

    #[error("Invalid Range header")]
    InvalidRangeHeader,

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
                | AuthError::UserNotFoundForSession(_) => {
                    (StatusCode::UNAUTHORIZED, "Unauthorized")
                }
                AuthError::Forbidden => (StatusCode::FORBIDDEN, "Admin access required"),
            },
            AppError::Database(e) => {
                tracing::error!("Database error: {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "Database error")
            }
            AppError::NotFound => (StatusCode::NOT_FOUND, "Resource not found"),
            AppError::AlbumNotEmpty => (StatusCode::CONFLICT, "Album still has tracks"),
            AppError::ArtistNotEmpty => (StatusCode::CONFLICT, "Artist still has albums or tracks"),
            AppError::RangeNotSatisfiable => {
                (StatusCode::RANGE_NOT_SATISFIABLE, "Range not satisfiable")
            }
            AppError::InvalidRangeHeader => (StatusCode::BAD_REQUEST, "Invalid Range header"),
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
