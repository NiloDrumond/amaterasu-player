use crate::{
    handlers::{stream_handlers, tracks_handlers},
    state::AppState,
};
use axum::{
    routing::{get, post},
    Router,
};

pub fn tracks_routes() -> Router<AppState> {
    Router::new()
        .route("/tracks", get(tracks_handlers::get_tracks))
        .route("/tracks/{id}", get(tracks_handlers::get_track))
        .route("/tracks/{id}/stream", get(stream_handlers::stream_track))
        .route(
            "/tracks/{id}/scrobble",
            post(tracks_handlers::scrobble_track),
        )
}
