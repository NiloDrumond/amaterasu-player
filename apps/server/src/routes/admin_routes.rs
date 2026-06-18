use axum::{
    routing::{delete, get, patch, post},
    Router,
};

use crate::{
    handlers::{admin_handlers, musicbrainz_handlers},
    state::AppState,
};

pub fn admin_routes() -> Router<AppState> {
    Router::new()
        .route(
            "/admin/users",
            post(admin_handlers::create_user).get(admin_handlers::list_users),
        )
        .route("/admin/users/{id}", delete(admin_handlers::delete_user))
        .route(
            "/admin/users/{id}/reset-password",
            post(admin_handlers::reset_user_password),
        )
        .route("/admin/scan-library", post(admin_handlers::scan_library))
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
        .route("/admin/albums", post(admin_handlers::create_album))
        .route("/admin/albums/{id}", get(admin_handlers::get_album))
        .route("/admin/albums/{id}", patch(admin_handlers::update_album))
        .route("/admin/albums/{id}", delete(admin_handlers::delete_album))
        .route(
            "/admin/albums/{id}/force-rescan",
            post(admin_handlers::force_rescan_album),
        )
        .route(
            "/admin/albums/{id}/cover",
            post(admin_handlers::upload_album_cover),
        )
        .route(
            "/admin/albums/{id}/merge",
            post(admin_handlers::merge_album),
        )
        .route("/admin/artists", post(admin_handlers::create_artist))
        .route("/admin/artists/{id}", get(admin_handlers::get_artist))
        .route("/admin/artists/{id}", patch(admin_handlers::update_artist))
        .route("/admin/artists/{id}", delete(admin_handlers::delete_artist))
        .route(
            "/admin/artists/{id}/force-rescan",
            post(admin_handlers::force_rescan_artist),
        )
        .route(
            "/admin/artists/{id}/merge",
            post(admin_handlers::merge_artist),
        )
        .route("/admin/review/queue", get(admin_handlers::get_review_queue))
        .route(
            "/admin/review/counts",
            get(admin_handlers::get_review_counts),
        )
        .route(
            "/admin/tracks/{id}/approve",
            post(admin_handlers::approve_track),
        )
        .route(
            "/admin/tracks/{id}/unapprove",
            post(admin_handlers::unapprove_track),
        )
        .route(
            "/admin/albums/{id}/approve",
            post(admin_handlers::approve_album),
        )
        .route(
            "/admin/albums/{id}/unapprove",
            post(admin_handlers::unapprove_album),
        )
        .route(
            "/admin/albums/{id}/approve-cascade",
            post(admin_handlers::approve_album_cascade),
        )
        .route(
            "/admin/artists/{id}/approve",
            post(admin_handlers::approve_artist),
        )
        .route(
            "/admin/artists/{id}/unapprove",
            post(admin_handlers::unapprove_artist),
        )
        // MusicBrainz metadata suggestions
        .route(
            "/admin/albums/{id}/mb-lookup",
            post(musicbrainz_handlers::lookup_album),
        )
        .route(
            "/admin/artists/{id}/mb-lookup",
            post(musicbrainz_handlers::lookup_artist),
        )
        .route(
            "/admin/tracks/{id}/mb-lookup",
            post(musicbrainz_handlers::lookup_track),
        )
        .route(
            "/admin/mb-lookup/pending",
            post(musicbrainz_handlers::bulk_lookup_pending),
        )
        .route(
            "/admin/albums/{id}/mb-suggestions",
            get(musicbrainz_handlers::list_album_suggestions),
        )
        .route(
            "/admin/artists/{id}/mb-suggestions",
            get(musicbrainz_handlers::list_artist_suggestions),
        )
        .route(
            "/admin/tracks/{id}/mb-suggestions",
            get(musicbrainz_handlers::list_track_suggestions),
        )
        .route(
            "/admin/review/queue/mb-suggestions",
            get(musicbrainz_handlers::review_queue_mb_suggestions),
        )
        .route(
            "/admin/mb-suggestions/{id}/accept",
            post(musicbrainz_handlers::accept_suggestion),
        )
        .route(
            "/admin/mb-suggestions/{id}/reject",
            post(musicbrainz_handlers::reject_suggestion),
        )
}
