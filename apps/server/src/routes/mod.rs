use crate::auth::{auth_guard, session_extractor};
use crate::routes::track_routes::tracks_routes;
use crate::state::AppState;
use axum::middleware;
use axum::{routing::get, Router};

mod auth_routes;
mod track_routes;

pub fn create_api_router(state: AppState) -> Router {
    let protected_routes = Router::new()
        .merge(tracks_routes())
        .merge(auth_routes::protected_routes())
        .layer(middleware::from_fn(auth_guard));

    let public_routes = Router::new()
        .merge(auth_routes::public_routes())
        .route("health", get(health_check));

    let api_routes = Router::new()
        .merge(protected_routes)
        .merge(public_routes)
        .layer(middleware::from_fn_with_state(
            state.clone(),
            session_extractor,
        ));

    Router::new().nest("/api", api_routes).with_state(state)
}

async fn health_check() -> &'static str {
    "OK"
}
