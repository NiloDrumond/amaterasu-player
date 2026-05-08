use crate::{handlers::album_collection_handlers, state::AppState};
use axum::{
    routing::{delete, get, patch, post},
    Router,
};

pub fn album_collection_routes() -> Router<AppState> {
    Router::new()
        .route(
            "/collections",
            get(album_collection_handlers::list_collections),
        )
        .route(
            "/collections",
            post(album_collection_handlers::create_collection),
        )
        .route(
            "/collections/{id}",
            get(album_collection_handlers::get_collection),
        )
        .route(
            "/collections/{id}",
            patch(album_collection_handlers::rename_collection),
        )
        .route(
            "/collections/{id}",
            delete(album_collection_handlers::delete_collection),
        )
        .route(
            "/collections/{id}/filter",
            patch(album_collection_handlers::update_collection_filter),
        )
        .route(
            "/collections/{id}/albums",
            get(album_collection_handlers::list_collection_albums),
        )
}
