use crate::auth::{admin_guard, auth_guard, session_extractor};
use crate::routes::admin_routes::admin_routes;
use crate::routes::album_routes::albums_routes;
use crate::routes::artist_routes::artists_routes;
use crate::routes::cover_routes::covers_routes;
use crate::routes::playlist_routes::playlist_routes;
use crate::routes::tag_category_routes::tag_category_routes;
use crate::routes::tag_routes::tag_routes;
use crate::routes::track_routes::tracks_routes;
use crate::state::AppState;
use axum::middleware;
use axum::{routing::get, Router};
use governor::clock::QuantaInstant;
use governor::middleware::NoOpMiddleware;
use tower_governor::governor::GovernorConfig;
use tower_governor::key_extractor::PeerIpKeyExtractor;
use tower_governor::GovernorLayer;

mod admin_routes;
mod album_routes;
mod artist_routes;
mod auth_routes;
mod cover_routes;
mod playlist_routes;
mod tag_category_routes;
mod tag_routes;
mod track_routes;

pub fn create_api_router(
    state: AppState,
    governor_conf: GovernorConfig<PeerIpKeyExtractor, NoOpMiddleware<QuantaInstant>>,
) -> Router {
    let admin_subtree = Router::new()
        .merge(admin_routes())
        .layer(middleware::from_fn(admin_guard));

    let protected_routes = Router::new()
        .merge(albums_routes())
        .merge(artists_routes())
        .merge(tracks_routes())
        .merge(covers_routes())
        .merge(playlist_routes())
        .merge(tag_category_routes())
        .merge(tag_routes())
        .merge(auth_routes::protected_routes())
        .merge(admin_subtree)
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
