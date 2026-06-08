use axum::{extract::DefaultBodyLimit, routing::post, Router};

use crate::{handlers::client_log_handlers, state::AppState};

/// Public (unauthenticated) so errors on the login page are captured too. The
/// optional session is still attached by `session_extractor`, letting the
/// handler stamp logs with the user when one is signed in.
pub fn client_log_routes() -> Router<AppState> {
    Router::new()
        .route("/client-logs", post(client_log_handlers::ingest))
        .layer(DefaultBodyLimit::max(256 * 1024))
}
