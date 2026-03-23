use crate::routes::{auth_routes::auth_routes, track_routes::tracks_routes};
use crate::state::AppState;
use axum::{routing::get, Router};
use std::sync::Arc;

mod auth_routes;
mod track_routes;

pub fn create_api_router() -> Router<Arc<AppState>> {
    Router::new().nest("/api", api_routes())
}

fn api_routes() -> Router<Arc<AppState>> {
    Router::new()
        .merge(tracks_routes())
        .merge(auth_routes())
        .route("/health", get(health_check))
}

async fn health_check() -> &'static str {
    "OK"
}
