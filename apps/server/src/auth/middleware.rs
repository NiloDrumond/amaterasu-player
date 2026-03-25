use axum::{
    extract::{Request, State},
    http::HeaderMap,
    middleware::Next,
    response::Response,
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

pub async fn auth_guard(
    State(state): State<AppState>,
    headers: HeaderMap,
    mut request: Request,
    next: Next,
) -> AppResult<Response> {
    let cookie = headers
        .get(SESSION_COOKIE_NAME)
        .ok_or(AppError::Auth(AuthError::MissingSessionCookie))?;
    let session_id = cookie.to_str().map_err(|e| AppError::Internal(e.into()))?;

    let service = AuthService::new(state.db.clone());
    let (session, user) = service.validate_session(session_id).await?;
    let auth_user = AuthUser { session, user };

    request.extensions_mut().insert(auth_user);

    let response = next.run(request).await;

    Ok(response)
}

