use crate::{handlers::playlist_handlers, state::AppState};
use axum::{
    routing::{delete, get, patch, post},
    Router,
};

pub fn playlist_routes() -> Router<AppState> {
    Router::new()
        .route("/playlists", get(playlist_handlers::list_playlists))
        .route("/playlists", post(playlist_handlers::create_playlist))
        .route("/playlists/{id}", get(playlist_handlers::get_playlist))
        .route("/playlists/{id}", patch(playlist_handlers::rename_playlist))
        .route(
            "/playlists/{id}",
            delete(playlist_handlers::delete_playlist),
        )
        .route(
            "/playlists/{id}/tracks",
            get(playlist_handlers::list_playlist_tracks),
        )
        .route(
            "/playlists/{id}/tracks",
            post(playlist_handlers::add_tracks),
        )
        .route(
            "/playlists/{id}/tracks/{tid}",
            delete(playlist_handlers::remove_track),
        )
        .route(
            "/playlists/{id}/tracks/{tid}",
            patch(playlist_handlers::reorder_track),
        )
}
