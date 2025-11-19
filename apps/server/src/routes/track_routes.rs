use crate::{handlers, state::AppState};
use axum::{routing::get, Router};
use std::sync::Arc;

pub fn tracks_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/tracks", get(handlers::get_tracks))
        .route("/tracks/{id}", get(handlers::get_track))
}
