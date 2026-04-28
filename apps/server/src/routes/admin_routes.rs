use axum::{routing::post, Router};

use crate::{handlers::admin_handlers, state::AppState};

pub fn admin_routes() -> Router<AppState> {
    Router::new().route("/admin/scan-library", post(admin_handlers::scan_library))
}
