use crate::{handlers, state::AppState};
use axum::{routing::get, Router};

pub fn tracks_routes() -> Router<AppState> {
    Router::new()
        .route("/tracks", get(handlers::get_tracks))
        .route("/tracks/{id}", get(handlers::get_track))
}
