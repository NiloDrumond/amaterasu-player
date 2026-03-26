use axum::{
    extract::{Request, State},
    http::HeaderMap,
    middleware::Next,
    response::Response,
    Extension,
};

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

#[derive(Debug, Clone)]
pub enum ExtractedSession {
    Valid(AuthUser),
    Invalid(AuthError),
}

async fn extract_session(app_state: AppState, headers: HeaderMap) -> AppResult<ExtractedSession> {
    let cookie = headers.get(SESSION_COOKIE_NAME);
    if let Some(cookie) = cookie {
        let session_id = cookie.to_str().map_err(|e| AppError::Internal(e.into()))?;

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
    headers: HeaderMap,
    mut request: Request,
    next: Next,
) -> AppResult<Response> {
    let extracted_session = extract_session(state, headers).await?;

    request.extensions_mut().insert(extracted_session);

    let response = next.run(request).await;

    Ok(response)
}

pub async fn auth_guard(
    Extension(session): Extension<ExtractedSession>,
    request: Request,
    next: Next,
) -> AppResult<Response> {
    if let ExtractedSession::Invalid(err) = session {
        return Err(AppError::Auth(err));
    }

    let response = next.run(request).await;

    Ok(response)
}
