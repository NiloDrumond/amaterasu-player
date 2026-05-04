use crate::{handlers::tag_category_handlers, state::AppState};
use axum::{
    routing::{delete, get, patch, post, put},
    Router,
};

pub fn tag_category_routes() -> Router<AppState> {
    Router::new()
        .route(
            "/tag-categories",
            get(tag_category_handlers::list_tag_categories),
        )
        .route(
            "/tag-categories",
            post(tag_category_handlers::create_tag_category),
        )
        .route(
            "/tag-categories/reorder",
            put(tag_category_handlers::reorder_tag_categories),
        )
        .route(
            "/tag-categories/{id}",
            patch(tag_category_handlers::update_tag_category),
        )
        .route(
            "/tag-categories/{id}",
            delete(tag_category_handlers::delete_tag_category),
        )
}
