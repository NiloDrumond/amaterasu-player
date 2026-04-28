use crate::{handlers::albums_handlers, state::AppState};
use axum::{routing::get, Router};

pub fn albums_routes() -> Router<AppState> {
    Router::new()
        .route("/albums", get(albums_handlers::get_albums))
        .route("/albums/{id}", get(albums_handlers::get_album))
        .route("/albums/{id}/tracks", get(albums_handlers::get_album_tracks))
}
