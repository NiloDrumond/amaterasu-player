use std::sync::Arc;

use axum::{routing::post, Router};

use crate::{handlers, state::AppState};

pub fn auth_routes() -> Router<Arc<AppState>> {
    Router::new().route("/auth/register", post(handlers::register_email))
}

