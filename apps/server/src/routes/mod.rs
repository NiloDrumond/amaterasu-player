use crate::auth::{admin_guard, auth_guard, session_extractor};
use crate::handlers::grafana_proxy_handlers;
use crate::routes::admin_routes::admin_routes;
use crate::routes::album_collection_routes::album_collection_routes;
use crate::routes::album_routes::albums_routes;
use crate::routes::artist_routes::artists_routes;
use crate::routes::cover_routes::covers_routes;
use crate::routes::home_routes::home_routes;
use crate::routes::playlist_routes::playlist_routes;
use crate::routes::search_routes::search_routes;
use crate::routes::tag_category_routes::tag_category_routes;
use crate::routes::tag_routes::tag_routes;
use crate::routes::track_routes::tracks_routes;
use crate::state::AppState;
use axum::extract::{DefaultBodyLimit, State};
use axum::http::{HeaderValue, Method, StatusCode};
use axum::middleware;
use axum::{
    routing::{any, get},
    Router,
};
use governor::clock::QuantaInstant;
use governor::middleware::NoOpMiddleware;
use tower_governor::governor::GovernorConfig;
use tower_governor::key_extractor::SmartIpKeyExtractor;
use tower_governor::GovernorLayer;
use tower_http::cors::CorsLayer;

mod admin_routes;
mod album_collection_routes;
mod album_routes;
mod artist_routes;
mod auth_routes;
mod cover_routes;
mod home_routes;
mod playlist_routes;
mod search_routes;
mod tag_category_routes;
mod tag_routes;
mod track_routes;

pub fn create_api_router(
    state: AppState,
    auth_governor: GovernorConfig<SmartIpKeyExtractor, NoOpMiddleware<QuantaInstant>>,
    protected_governor: GovernorConfig<SmartIpKeyExtractor, NoOpMiddleware<QuantaInstant>>,
    cors_origins: Option<&str>,
) -> Router {
    let admin_subtree = Router::new()
        .merge(admin_routes())
        .layer(middleware::from_fn(admin_guard));

    let protected_routes = Router::new()
        .merge(albums_routes())
        .merge(album_collection_routes())
        .merge(artists_routes())
        .merge(tracks_routes())
        .merge(covers_routes())
        .merge(home_routes())
        .merge(playlist_routes())
        .merge(search_routes())
        .merge(tag_category_routes())
        .merge(tag_routes())
        .merge(auth_routes::protected_routes())
        .merge(admin_subtree)
        .layer(middleware::from_fn(auth_guard))
        .layer(GovernorLayer::new(protected_governor));

    let rate_limited_routes = Router::new()
        .merge(auth_routes::public_routes())
        .layer(GovernorLayer::new(auth_governor));

    let public_routes = Router::new().merge(rate_limited_routes);

    let api_routes = Router::new()
        .merge(protected_routes)
        .merge(public_routes)
        .layer(middleware::from_fn_with_state(
            state.clone(),
            session_extractor,
        ));

    let grafana_proxy_routes: Router<AppState> = Router::new()
        .route("/admin/logs", any(grafana_proxy_handlers::proxy))
        .route("/admin/logs/", any(grafana_proxy_handlers::proxy))
        .route("/admin/logs/{*path}", any(grafana_proxy_handlers::proxy))
        .layer(middleware::from_fn(admin_guard))
        .layer(middleware::from_fn(auth_guard))
        .layer(middleware::from_fn_with_state(
            state.clone(),
            session_extractor,
        ));

    let cors = match cors_origins {
        Some(origins) => {
            let allowed: Vec<HeaderValue> = origins
                .split(',')
                .filter_map(|o| o.trim().parse().ok())
                .collect();
            CorsLayer::new()
                .allow_origin(allowed)
                .allow_methods([
                    Method::GET,
                    Method::POST,
                    Method::PUT,
                    Method::PATCH,
                    Method::DELETE,
                ])
                .allow_headers([axum::http::header::CONTENT_TYPE])
                .allow_credentials(true)
        }
        None => CorsLayer::new(),
    };

    Router::new()
        .route("/health", get(health_check))
        .nest("/api", api_routes)
        .merge(grafana_proxy_routes)
        .layer(cors)
        .layer(DefaultBodyLimit::max(16 * 1024 * 1024))
        .with_state(state)
}

async fn health_check(State(state): State<AppState>) -> Result<&'static str, StatusCode> {
    sqlx::query("SELECT 1")
        .execute(&state.db)
        .await
        .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?;
    Ok("OK")
}
