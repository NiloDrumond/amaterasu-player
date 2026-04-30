use crate::{handlers::tag_handlers, state::AppState};
use axum::{
    routing::{delete, get, patch, post, put},
    Router,
};

pub fn tag_routes() -> Router<AppState> {
    Router::new()
        .route("/tags", get(tag_handlers::list_tags))
        .route("/tags", post(tag_handlers::create_tag))
        .route("/tags/{id}", patch(tag_handlers::update_tag))
        .route("/tags/{id}", delete(tag_handlers::delete_tag))
        .route("/tracks/{id}/tags", get(tag_handlers::list_track_tags))
        .route(
            "/tracks/{id}/tags/{tagId}",
            put(tag_handlers::attach_track_tag),
        )
        .route(
            "/tracks/{id}/tags/{tagId}",
            delete(tag_handlers::detach_track_tag),
        )
        .route("/albums/{id}/tags", get(tag_handlers::list_album_tags))
        .route(
            "/albums/{id}/tags/{tagId}",
            put(tag_handlers::attach_album_tag),
        )
        .route(
            "/albums/{id}/tags/{tagId}",
            delete(tag_handlers::detach_album_tag),
        )
}
