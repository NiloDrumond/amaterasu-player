use crate::{handlers::covers_handlers, state::AppState};
use axum::{routing::get, Router};

pub fn covers_routes() -> Router<AppState> {
    Router::new().route("/covers/{filename}", get(covers_handlers::get_cover))
}
