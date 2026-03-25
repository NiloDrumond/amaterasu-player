use axum::{extract::State, http::StatusCode, Json};
use axum_valid::Garde;

use crate::{
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
) -> AppResult<()> {
    Ok(())
}
