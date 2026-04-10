use std::net::SocketAddr;

use axum::{
    extract::{ConnectInfo, State},
    http::StatusCode,
    response::IntoResponse,
    Extension, Json,
};
use axum_extra::extract::cookie::{Cookie, CookieJar};
use axum_valid::Garde;

use crate::{
    auth::{AuthUser, ExtractedSession, SESSION_COOKIE_NAME, SESSION_DURATION_HOURS},
    dto::{
        request::auth::{RegisterEmailParams, SignInEmailParams},
        response::CurrentUserResponse,
    },
    error::AppResult,
    services::auth_service::AuthService,
    state::AppState,
};

pub async fn register_email(
    State(state): State<AppState>,
    Garde(Json(body)): Garde<Json<RegisterEmailParams>>,
) -> AppResult<StatusCode> {
    let service = AuthService::new(state.db.clone());
    service
        .register_email(body.email, body.name, body.password)
        .await?;

    Ok(StatusCode::CREATED)
}

pub async fn sign_in_email(
    State(state): State<AppState>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    Extension(session): Extension<ExtractedSession>,
    Garde(Json(body)): Garde<Json<SignInEmailParams>>,
) -> AppResult<impl IntoResponse> {
    let service = AuthService::new(state.db.clone());
    if let ExtractedSession::Valid(auth_user) = session {
        service.delete_session(&auth_user.session.id).await;
    }
    let session = service
        .sign_in_email(&body.email, &body.password, Some(addr.ip()))
        .await?;

    let max_age = SESSION_DURATION_HOURS * 60 * 60;
    let cookie = Cookie::build((SESSION_COOKIE_NAME, session.id))
        .path("/")
        .http_only(true)
        .secure(true)
        .same_site(axum_extra::extract::cookie::SameSite::Lax)
        .max_age(time::Duration::seconds(max_age as i64));

    Ok(CookieJar::new().add(cookie))
}

pub async fn sign_out(
    State(state): State<AppState>,
    auth_user: AuthUser,
) -> AppResult<impl IntoResponse> {
    let service = AuthService::new(state.db.clone());
    service.delete_session(&auth_user.session.id).await;

    let cookie = Cookie::build((SESSION_COOKIE_NAME, ""))
        .path("/")
        .http_only(true)
        .secure(true)
        .same_site(axum_extra::extract::cookie::SameSite::Lax)
        .max_age(time::Duration::ZERO);

    Ok(CookieJar::new().add(cookie))
}

pub async fn get_current_user(auth_user: AuthUser) -> Json<CurrentUserResponse> {
    Json(auth_user.user.into())
}
