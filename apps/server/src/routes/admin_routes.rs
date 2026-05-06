use axum::{
    routing::{delete, get, patch, post},
    Router,
};

use crate::{handlers::admin_handlers, state::AppState};

pub fn admin_routes() -> Router<AppState> {
    Router::new()
        .route("/admin/scan-library", post(admin_handlers::scan_library))
        // Tracks
        .route(
            "/admin/tracks/deleted",
            get(admin_handlers::list_deleted_tracks),
        )
        .route(
            "/admin/tracks/batch",
            patch(admin_handlers::batch_update_tracks),
        )
        .route("/admin/tracks/{id}", get(admin_handlers::get_track))
        .route("/admin/tracks/{id}", patch(admin_handlers::update_track))
        .route(
            "/admin/tracks/{id}",
            delete(admin_handlers::soft_delete_track),
        )
        .route(
            "/admin/tracks/{id}/restore",
            post(admin_handlers::restore_track),
        )
        .route(
            "/admin/tracks/{id}/force-rescan",
            post(admin_handlers::force_rescan_track),
        )
        // Albums
        .route("/admin/albums", post(admin_handlers::create_album))
        .route("/admin/albums/{id}", get(admin_handlers::get_album))
        .route("/admin/albums/{id}", patch(admin_handlers::update_album))
        .route("/admin/albums/{id}", delete(admin_handlers::delete_album))
        .route(
            "/admin/albums/{id}/force-rescan",
            post(admin_handlers::force_rescan_album),
        )
        // Artists
        .route("/admin/artists", post(admin_handlers::create_artist))
        .route("/admin/artists/{id}", get(admin_handlers::get_artist))
        .route("/admin/artists/{id}", patch(admin_handlers::update_artist))
        .route("/admin/artists/{id}", delete(admin_handlers::delete_artist))
        .route(
            "/admin/artists/{id}/force-rescan",
            post(admin_handlers::force_rescan_artist),
        )
}
