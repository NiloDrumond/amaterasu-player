use axum::{
    routing::{get, post, put},
    Router,
};

use crate::{handlers::auth_handlers, state::AppState};

pub fn public_routes() -> Router<AppState> {
    Router::new().route("/auth/sign-in", post(auth_handlers::sign_in_email))
}

pub fn protected_routes() -> Router<AppState> {
    Router::new()
        .route("/auth/sign-out", post(auth_handlers::sign_out))
        .route("/auth/me", get(auth_handlers::get_current_user))
        .route(
            "/auth/me/preferences",
            put(auth_handlers::update_preferences),
        )
}
