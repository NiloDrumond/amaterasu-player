use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use axum_valid::Garde;
use uuid::Uuid;

use crate::{
    auth::AuthUser,
    dto::{
        request::{AddTracksParams, CreatePlaylistParams, RenamePlaylistParams, ReorderTrackParams},
        response::{PlaylistResponse, PlaylistTrackResponse},
    },
    error::{AppError, AppResult},
    repositories::PlaylistRepository,
    state::AppState,
};

const INITIAL_POSITION: f64 = 1000.0;
const POSITION_STEP: f64 = 1000.0;
const MIN_GAP: f64 = 1e-9;

pub async fn list_playlists(
    State(state): State<AppState>,
    auth_user: AuthUser,
) -> AppResult<Json<Vec<PlaylistResponse>>> {
    let playlists =
        PlaylistRepository::list_by_user(&state.db, auth_user.user.id).await?;

    Ok(Json(playlists.into_iter().map(Into::into).collect()))
}

pub async fn create_playlist(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Garde(Json(body)): Garde<Json<CreatePlaylistParams>>,
) -> AppResult<(StatusCode, Json<PlaylistResponse>)> {
    let playlist =
        PlaylistRepository::create(&state.db, auth_user.user.id, &body.name).await?;

    let stats = PlaylistRepository::find_by_id_and_user(
        &state.db,
        playlist.id,
        auth_user.user.id,
    )
    .await?
    .ok_or(AppError::NotFound)?;

    Ok((StatusCode::CREATED, Json(stats.into())))
}

pub async fn get_playlist(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(id): Path<Uuid>,
) -> AppResult<Json<PlaylistResponse>> {
    let stats =
        PlaylistRepository::find_by_id_and_user(&state.db, id, auth_user.user.id).await?
            .ok_or(AppError::NotFound)?;

    Ok(Json(stats.into()))
}

pub async fn rename_playlist(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(id): Path<Uuid>,
    Garde(Json(body)): Garde<Json<RenamePlaylistParams>>,
) -> AppResult<Json<PlaylistResponse>> {
    PlaylistRepository::rename(&state.db, id, auth_user.user.id, &body.name)
        .await?
        .ok_or(AppError::NotFound)?;

    let stats =
        PlaylistRepository::find_by_id_and_user(&state.db, id, auth_user.user.id).await?
            .ok_or(AppError::NotFound)?;

    Ok(Json(stats.into()))
}

pub async fn delete_playlist(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(id): Path<Uuid>,
) -> AppResult<StatusCode> {
    let deleted =
        PlaylistRepository::delete(&state.db, id, auth_user.user.id).await?;

    if deleted {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(AppError::NotFound)
    }
}

pub async fn list_playlist_tracks(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(id): Path<Uuid>,
) -> AppResult<Json<Vec<PlaylistTrackResponse>>> {
    // Verify ownership
    if !PlaylistRepository::belongs_to_user(&state.db, id, auth_user.user.id).await? {
        return Err(AppError::NotFound);
    }

    let tracks =
        PlaylistRepository::list_tracks(&state.db, id, auth_user.user.id).await?;

    Ok(Json(tracks.into_iter().map(Into::into).collect()))
}

pub async fn add_tracks(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(id): Path<Uuid>,
    Garde(Json(body)): Garde<Json<AddTracksParams>>,
) -> AppResult<StatusCode> {
    // Verify ownership
    if !PlaylistRepository::belongs_to_user(&state.db, id, auth_user.user.id).await? {
        return Err(AppError::NotFound);
    }

    // Get current max position once, then increment for each track
    let max_pos = PlaylistRepository::get_max_position(&state.db, id).await?;
    let mut next_pos = max_pos.map(|p| p + POSITION_STEP).unwrap_or(INITIAL_POSITION);

    for track_id in body.track_ids {
        // ON CONFLICT DO NOTHING in insert_track handles duplicates
        PlaylistRepository::insert_track(&state.db, id, track_id, next_pos).await?;
        next_pos += POSITION_STEP;
    }

    Ok(StatusCode::NO_CONTENT)
}

pub async fn remove_track(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path((id, tid)): Path<(Uuid, Uuid)>,
) -> AppResult<StatusCode> {
    let deleted =
        PlaylistRepository::remove_track(&state.db, id, auth_user.user.id, tid).await?;

    if deleted {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(AppError::NotFound)
    }
}

pub async fn reorder_track(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path((id, tid)): Path<(Uuid, Uuid)>,
    Garde(Json(body)): Garde<Json<ReorderTrackParams>>,
) -> AppResult<StatusCode> {
    // Verify ownership (outside the transaction — read-only, no race condition risk)
    if !PlaylistRepository::belongs_to_user(&state.db, id, auth_user.user.id).await? {
        return Err(AppError::NotFound);
    }

    let mut tx = state.db.begin().await?;

    let (prev, next) = PlaylistRepository::get_neighbor_positions(&mut *tx, id, body.after_id)
        .await?
        .ok_or(AppError::NotFound)?;

    let new_position = compute_position(prev, next);

    // Check if gap is too small — redistribute then recompute
    let new_position = if let (Some(p), Some(n)) = (prev, next) {
        if (n - p).abs() < MIN_GAP {
            PlaylistRepository::redistribute_positions(&mut *tx, id).await?;
            let (prev2, next2) =
                PlaylistRepository::get_neighbor_positions(&mut *tx, id, body.after_id)
                    .await?
                    .ok_or(AppError::NotFound)?;
            compute_position(prev2, next2)
        } else {
            new_position
        }
    } else {
        new_position
    };

    let updated =
        PlaylistRepository::update_track_position(&mut *tx, id, tid, new_position).await?;

    tx.commit().await?;

    if updated {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(AppError::NotFound)
    }
}

fn compute_position(prev: Option<f64>, next: Option<f64>) -> f64 {
    match (prev, next) {
        (None, None) => INITIAL_POSITION,
        (None, Some(n)) => n / 2.0,
        (Some(p), None) => p + POSITION_STEP,
        (Some(p), Some(n)) => (p + n) / 2.0,
    }
}
