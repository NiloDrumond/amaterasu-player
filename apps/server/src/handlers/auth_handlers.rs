use anyhow::Context;
use axum::{
    extract::State,
    http::{header::SET_COOKIE, HeaderMap, StatusCode},
    response::IntoResponse,
    Json,
};
use axum_valid::Garde;

use crate::{
    auth::{SESSION_COOKIE_NAME, SESSION_DURATION_HOURS},
    dto::request::auth::{RegisterEmailParams, SignInEmailParams},
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
    Garde(Json(body)): Garde<Json<SignInEmailParams>>,
) -> AppResult<impl IntoResponse> {
    let service = AuthService::new(state.db.clone());
    let session = service.sign_in_email(&body.email, &body.password).await?;

    let cookie = session.id;
    let max_age = SESSION_DURATION_HOURS * 60 * 60;
    let cookie = format!(
        "{SESSION_COOKIE_NAME}={cookie}; SameSite=Lax; HttpOnly; Secure; Path=/; Max-Age={max_age}"
    );
    let mut headers = HeaderMap::new();
    headers.insert(
        SET_COOKIE,
        cookie.parse().context("failed to parse cookie")?,
    );

    Ok(headers)
}
