use crate::{handlers::artists_handlers, state::AppState};
use axum::{routing::get, Router};

pub fn artists_routes() -> Router<AppState> {
    Router::new()
        .route("/artists", get(artists_handlers::get_artists))
        .route("/artists/{id}", get(artists_handlers::get_artist))
        .route("/artists/{id}/albums", get(artists_handlers::get_artist_albums))
}
