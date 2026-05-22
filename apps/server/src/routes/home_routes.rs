use crate::{handlers::home_handlers, state::AppState};
use axum::{
    routing::{delete, get, patch, post},
    Router,
};

pub fn home_routes() -> Router<AppState> {
    Router::new()
        .route(
            "/me/pinned-playlists",
            get(home_handlers::list_pinned_playlists),
        )
        .route("/me/pinned-playlists", post(home_handlers::pin_playlist))
        .route(
            "/me/pinned-playlists/reorder",
            patch(home_handlers::reorder_pinned_playlists),
        )
        .route(
            "/me/pinned-playlists/{playlist_id}",
            delete(home_handlers::unpin_playlist),
        )
        .route(
            "/me/recommendations/listen-again",
            get(home_handlers::listen_again),
        )
        .route(
            "/me/recommendations/forgotten-favorites",
            get(home_handlers::forgotten_favorites),
        )
}
