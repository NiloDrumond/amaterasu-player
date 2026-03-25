use axum::{routing::post, Router};

use crate::{handlers::auth_handlers, state::AppState};

pub fn auth_routes() -> Router<AppState> {
    Router::new()
        .route("/auth/register", post(auth_handlers::register_email))
        .route("/auth/sign-in", post(auth_handlers::sign_in_email))
}
