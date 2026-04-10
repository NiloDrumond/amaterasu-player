use crate::auth::{auth_guard, session_extractor};
use crate::routes::track_routes::tracks_routes;
use crate::state::AppState;
use axum::middleware;
use axum::{routing::get, Router};
use governor::clock::QuantaInstant;
use governor::middleware::NoOpMiddleware;
use tower_governor::governor::GovernorConfig;
use tower_governor::key_extractor::PeerIpKeyExtractor;
use tower_governor::GovernorLayer;

mod auth_routes;
mod track_routes;

pub fn create_api_router(
    state: AppState,
    governor_conf: GovernorConfig<PeerIpKeyExtractor, NoOpMiddleware<QuantaInstant>>,
) -> Router {
    let protected_routes = Router::new()
        .merge(tracks_routes())
        .merge(auth_routes::protected_routes())
        .layer(middleware::from_fn(auth_guard));

    let rate_limited_routes = Router::new()
        .merge(auth_routes::public_routes())
        .layer(GovernorLayer::new(governor_conf));

    let public_routes = Router::new()
        .merge(rate_limited_routes)
        .route("/health", get(health_check));

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
