use crate::{handlers::search_handlers, state::AppState};
use axum::{routing::get, Router};

pub fn search_routes() -> Router<AppState> {
    Router::new()
        .route("/search", get(search_handlers::palette_search))
        .route("/search/all", get(search_handlers::search_all))
}
