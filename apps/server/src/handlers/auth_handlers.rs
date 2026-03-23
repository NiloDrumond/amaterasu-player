use std::sync::Arc;

use axum::{extract::State, http::StatusCode, Json};

use crate::{
    dto::request::auth::RegisterEmailParams, error::AppResult, services::auth_service::AuthService,
    state::AppState,
};

pub async fn register_email(
    State(state): State<Arc<AppState>>,
    Json(body): Json<RegisterEmailParams>,
) -> AppResult<StatusCode> {
    let service = AuthService::new(state.db.clone());
    service
        .register_email(body.email, body.name, body.password)
        .await?;

    Ok(StatusCode::CREATED)
}
