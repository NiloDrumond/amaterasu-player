use crate::auth::auth_guard;
use crate::routes::{auth_routes::auth_routes, track_routes::tracks_routes};
use crate::state::AppState;
use axum::middleware;
use axum::{routing::get, Router};

mod auth_routes;
mod track_routes;

pub fn create_api_router(state: AppState) -> Router {
    let protected_routes = Router::new()
        .merge(tracks_routes())
        .layer(middleware::from_fn_with_state(state.clone(), auth_guard));

    let api_routes = Router::new()
        .merge(protected_routes)
        .merge(auth_routes())
        .route("/health", get(health_check));

    Router::new().nest("/api", api_routes).with_state(state)
}

async fn health_check() -> &'static str {
    "OK"
}
