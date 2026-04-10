use axum::{
    extract::{rejection::ExtensionRejection::MissingExtension, FromRequestParts, Request, State},
    middleware::Next,
    response::Response,
    Extension, RequestPartsExt,
};
use axum_extra::extract::CookieJar;

use crate::{
    auth::{error::AuthError, SESSION_COOKIE_NAME},
    db::entities::{Session, User},
    error::{AppError, AppResult},
    services::auth_service::AuthService,
    state::AppState,
};

#[derive(Debug, Clone)]
pub struct AuthUser {
    pub session: Session,
    pub user: User,
}

// Short-lived per-request enum; stack size difference is negligible compared to
// the heap allocation a Box would add on every authenticated request.
#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone)]
pub enum ExtractedSession {
    Valid(AuthUser),
    Invalid(AuthError),
}

impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        _state: &S,
    ) -> Result<Self, Self::Rejection> {
        let Extension(extracted_session) = parts
            .extract::<Extension<ExtractedSession>>()
            .await
            .map_err(|err| match err {
                MissingExtension(err) => AppError::Internal(anyhow::anyhow!(
                    "Missing Extension ExtractedSession for route {} that expected it. Err: {}",
                    parts.uri,
                    err
                )),
                _ => AppError::Internal(anyhow::anyhow!(
                    "Failed to extract Extension ExtractedSession for route {}",
                    parts.uri
                )),
            })?;
        match extracted_session {
            ExtractedSession::Valid(auth_user) => Ok(auth_user),
            ExtractedSession::Invalid(auth_error) => Err(AppError::Auth(auth_error)),
        }
    }
}

async fn extract_session(app_state: AppState, jar: CookieJar) -> AppResult<ExtractedSession> {
    let cookie = jar.get(SESSION_COOKIE_NAME);
    if let Some(cookie) = cookie {
        let session_id = cookie.value();

        let service = AuthService::new(app_state.db.clone());
        let result = service.validate_session(session_id).await;
        match result {
            Ok((session, user)) => {
                let auth_user = AuthUser { session, user };
                Ok(ExtractedSession::Valid(auth_user))
            }
            Err(AppError::Auth(auth_errr)) => Ok(ExtractedSession::Invalid(auth_errr)),
            Err(err) => Err(err),
        }
    } else {
        Ok(ExtractedSession::Invalid(AuthError::MissingSessionCookie))
    }
}

pub async fn session_extractor(
    State(state): State<AppState>,
    jar: CookieJar,
    mut request: Request,
    next: Next,
) -> AppResult<Response> {
    let extracted_session = extract_session(state, jar).await?;

    request.extensions_mut().insert(extracted_session);

    let response = next.run(request).await;

    Ok(response)
}

// NOTE: this is a middleware which its only purpose is to guarantee user authentication, even if
// the underlying route handler has no use for the user info
pub async fn auth_guard(_auth: AuthUser, request: Request, next: Next) -> AppResult<Response> {
    let response = next.run(request).await;
    Ok(response)
}
