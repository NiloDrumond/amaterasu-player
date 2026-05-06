use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use axum_valid::Garde;
use uuid::Uuid;

use crate::{
    db::entities::{Album, Artist},
    dto::{
        request::{
            BatchUpdateTracksParams, BatchUpdateTracksResponse, CreateAlbumParams,
            CreateArtistParams, HardDeleteQuery, UpdateAlbumParams, UpdateArtistParams,
            UpdateTrackParams,
        },
        response::{
            AdminAlbumResponse, AdminArtistResponse, AdminDeletedTrackResponse, AdminTrackResponse,
        },
    },
    error::{AppError, AppResult},
    repositories::track_repository::TrackUpdate,
    repositories::{AlbumRepository, ArtistRepository, TrackRepository},
    state::AppState,
};

pub async fn scan_library(State(state): State<AppState>) -> StatusCode {
    let Some(permit) = state.library_scanner.try_acquire_scan() else {
        return StatusCode::CONFLICT;
    };
    let scanner = state.library_scanner.clone();
    tokio::spawn(async move {
        if let Err(e) = scanner.run_scan(permit).await {
            tracing::warn!("Admin-triggered library scan failed: {}", e);
        }
    });
    StatusCode::ACCEPTED
}

// =====================================================================
// Tracks
// =====================================================================

pub async fn get_track(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> AppResult<Json<AdminTrackResponse>> {
    let track = TrackRepository::find_by_id(&state.db, id)
        .await?
        .ok_or(AppError::NotFound)?;
    Ok(Json(track.into()))
}

pub async fn update_track(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Garde(Json(body)): Garde<Json<UpdateTrackParams>>,
) -> AppResult<Json<AdminTrackResponse>> {
    let patch = TrackUpdate {
        title: body.title,
        sort_title: body.sort_title,
        artist_id: body.artist_id.into_option_option(),
        album_id: body.album_id.into_option_option(),
        disc: body.disc.into_option_option(),
        track_no: body.track_no.into_option_option(),
        date: body.date.into_option_option(),
        composer: body.composer.into_option_option(),
        comment: body.comment.into_option_option(),
    };

    // Ensure track exists (and surface 404 distinctly).
    if TrackRepository::find_by_id(&state.db, id).await?.is_none() {
        return Err(AppError::NotFound);
    }

    let updated = TrackRepository::admin_update(&state.db, id, &patch).await?;
    Ok(Json(updated.into()))
}

pub async fn batch_update_tracks(
    State(state): State<AppState>,
    Garde(Json(body)): Garde<Json<BatchUpdateTracksParams>>,
) -> AppResult<Json<BatchUpdateTracksResponse>> {
    let patch = TrackUpdate {
        title: body.patch.title,
        sort_title: body.patch.sort_title,
        artist_id: body.patch.artist_id.into_option_option(),
        album_id: body.patch.album_id.into_option_option(),
        disc: body.patch.disc.into_option_option(),
        track_no: body.patch.track_no.into_option_option(),
        date: body.patch.date.into_option_option(),
        composer: body.patch.composer.into_option_option(),
        comment: body.patch.comment.into_option_option(),
    };

    let updated = TrackRepository::admin_update_many(&state.db, &body.ids, &patch).await?;
    Ok(Json(BatchUpdateTracksResponse {
        updated: updated as i64,
    }))
}

pub async fn soft_delete_track(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Query(q): Query<HardDeleteQuery>,
) -> AppResult<StatusCode> {
    if q.hard {
        let deleted = TrackRepository::hard_delete_if_soft_deleted(&state.db, id).await?;
        if deleted {
            Ok(StatusCode::NO_CONTENT)
        } else {
            // Either not found or not soft-deleted yet.
            Err(AppError::NotFound)
        }
    } else {
        if TrackRepository::find_by_id(&state.db, id).await?.is_none() {
            return Err(AppError::NotFound);
        }
        TrackRepository::soft_delete(&state.db, id).await?;
        Ok(StatusCode::NO_CONTENT)
    }
}

pub async fn restore_track(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> AppResult<StatusCode> {
    if TrackRepository::find_by_id(&state.db, id).await?.is_none() {
        return Err(AppError::NotFound);
    }
    TrackRepository::restore(&state.db, id).await?;
    Ok(StatusCode::NO_CONTENT)
}

pub async fn force_rescan_track(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> AppResult<StatusCode> {
    if TrackRepository::find_by_id(&state.db, id).await?.is_none() {
        return Err(AppError::NotFound);
    }
    TrackRepository::force_rescan(&state.db, id).await?;
    Ok(StatusCode::NO_CONTENT)
}

pub async fn list_deleted_tracks(
    State(state): State<AppState>,
) -> AppResult<Json<Vec<AdminDeletedTrackResponse>>> {
    let tracks = TrackRepository::find_deleted(&state.db).await?;

    let mut out = Vec::with_capacity(tracks.len());
    for t in tracks {
        let file_missing = !tokio::fs::try_exists(&t.file_path).await.unwrap_or(false);
        out.push(AdminDeletedTrackResponse {
            track: t.into(),
            file_missing,
        });
    }
    Ok(Json(out))
}

// =====================================================================
// Albums
// =====================================================================

pub async fn get_album(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> AppResult<Json<AdminAlbumResponse>> {
    let album = AlbumRepository::find_by_id(&state.db, id).await?;
    Ok(Json(album.into()))
}

pub async fn update_album(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Garde(Json(body)): Garde<Json<UpdateAlbumParams>>,
) -> AppResult<Json<AdminAlbumResponse>> {
    let updated = AlbumRepository::update(
        &state.db,
        id,
        &body.title,
        &body.sort_title,
        body.artist_id,
        body.date,
    )
    .await?;
    Ok(Json(updated.into()))
}

pub async fn create_album(
    State(state): State<AppState>,
    Garde(Json(body)): Garde<Json<CreateAlbumParams>>,
) -> AppResult<(StatusCode, Json<AdminAlbumResponse>)> {
    let sort_title = body.sort_title.unwrap_or_else(|| body.title.clone());
    let album = Album::new(
        body.artist_id,
        body.title,
        sort_title,
        body.date,
        None,
        None,
    );
    let created = AlbumRepository::create(&state.db, &album).await?;
    Ok((StatusCode::CREATED, Json(created.into())))
}

pub async fn delete_album(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> AppResult<StatusCode> {
    let deleted = AlbumRepository::delete_if_empty(&state.db, id).await?;
    if deleted {
        Ok(StatusCode::NO_CONTENT)
    } else {
        // Either album not found, or it still has live tracks. Distinguish:
        if AlbumRepository::find_by_id(&state.db, id).await.is_err() {
            Err(AppError::NotFound)
        } else {
            Err(AppError::AlbumNotEmpty)
        }
    }
}

pub async fn force_rescan_album(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> AppResult<StatusCode> {
    AlbumRepository::clear_lock(&state.db, id).await?;
    Ok(StatusCode::NO_CONTENT)
}

// =====================================================================
// Artists
// =====================================================================

pub async fn get_artist(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> AppResult<Json<AdminArtistResponse>> {
    let artist = ArtistRepository::find_by_id(&state.db, id)
        .await?
        .ok_or(AppError::NotFound)?;
    Ok(Json(artist.into()))
}

pub async fn update_artist(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Garde(Json(body)): Garde<Json<UpdateArtistParams>>,
) -> AppResult<Json<AdminArtistResponse>> {
    let updated = ArtistRepository::update(&state.db, id, &body.name, &body.sort_name).await?;
    Ok(Json(updated.into()))
}

pub async fn create_artist(
    State(state): State<AppState>,
    Garde(Json(body)): Garde<Json<CreateArtistParams>>,
) -> AppResult<(StatusCode, Json<AdminArtistResponse>)> {
    let sort_name = body.sort_name.unwrap_or_else(|| body.name.clone());
    let artist = Artist::new(body.name, sort_name);
    let created = ArtistRepository::create(&state.db, &artist).await?;
    Ok((StatusCode::CREATED, Json(created.into())))
}

pub async fn delete_artist(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> AppResult<StatusCode> {
    let deleted = ArtistRepository::delete_if_empty(&state.db, id).await?;
    if deleted {
        Ok(StatusCode::NO_CONTENT)
    } else if ArtistRepository::find_by_id(&state.db, id).await?.is_none() {
        Err(AppError::NotFound)
    } else {
        Err(AppError::ArtistNotEmpty)
    }
}

pub async fn force_rescan_artist(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> AppResult<StatusCode> {
    ArtistRepository::clear_lock(&state.db, id).await?;
    Ok(StatusCode::NO_CONTENT)
}
