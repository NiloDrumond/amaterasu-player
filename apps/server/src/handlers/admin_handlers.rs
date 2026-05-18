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
            CreateArtistParams, HardDeleteQuery, MergeAlbumParams, MergeArtistParams,
            UpdateAlbumParams, UpdateArtistParams, UpdateTrackParams,
        },
        response::{
            AdminAlbumResponse, AdminArtistResponse, AdminDeletedTrackResponse, AdminTrackResponse,
            ReviewQueueAlbumGroup, ReviewQueueCounts, ReviewQueueResponse, SearchEntityType,
        },
    },
    error::{AppError, AppResult},
    repositories::track_repository::TrackUpdate,
    repositories::{AlbumRepository, AliasRepository, ArtistRepository, TrackRepository},
    search::indexers,
    state::AppState,
};

use serde::Deserialize;

pub async fn scan_library(State(state): State<AppState>) -> StatusCode {
    let Some(permit) = state.library_scanner.try_acquire_scan() else {
        return StatusCode::CONFLICT;
    };
    let scanner = state.library_scanner.clone();
    let pool = state.db.clone();
    let search = state.search.clone();
    tokio::spawn(async move {
        if let Err(e) = scanner.run_scan(permit).await {
            tracing::warn!("Admin-triggered library scan failed: {}", e);
        }
        if let Err(e) = crate::search::sync::rebuild_from_postgres(&pool, &search).await {
            tracing::warn!("search: post-scan rebuild failed: {}", e);
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
    let artist_name = match updated.artist_id {
        Some(aid) => ArtistRepository::find_by_id(&state.db, aid)
            .await?
            .map(|a| a.name),
        None => None,
    };
    let album_title = match updated.album_id {
        Some(aid) => AlbumRepository::find_by_id(&state.db, aid)
            .await
            .ok()
            .map(|a| a.title),
        None => None,
    };
    indexers::index_track(
        &state.search,
        &updated,
        artist_name.as_deref(),
        album_title.as_deref(),
    );
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
    for tid in &body.ids {
        if let Ok(Some(t)) = TrackRepository::find_by_id(&state.db, *tid).await {
            let artist_name = match t.artist_id {
                Some(aid) => ArtistRepository::find_by_id(&state.db, aid)
                    .await
                    .ok()
                    .flatten()
                    .map(|a| a.name),
                None => None,
            };
            let album_title = match t.album_id {
                Some(aid) => AlbumRepository::find_by_id(&state.db, aid)
                    .await
                    .ok()
                    .map(|a| a.title),
                None => None,
            };
            indexers::index_track(
                &state.search,
                &t,
                artist_name.as_deref(),
                album_title.as_deref(),
            );
        }
    }
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
            indexers::remove(&state.search, SearchEntityType::Track, id);
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
        indexers::remove(&state.search, SearchEntityType::Track, id);
        Ok(StatusCode::NO_CONTENT)
    }
}

pub async fn restore_track(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> AppResult<StatusCode> {
    let track = TrackRepository::find_by_id(&state.db, id)
        .await?
        .ok_or(AppError::NotFound)?;
    TrackRepository::restore(&state.db, id).await?;
    let artist_name = match track.artist_id {
        Some(aid) => ArtistRepository::find_by_id(&state.db, aid)
            .await?
            .map(|a| a.name),
        None => None,
    };
    let album_title = match track.album_id {
        Some(aid) => AlbumRepository::find_by_id(&state.db, aid)
            .await
            .ok()
            .map(|a| a.title),
        None => None,
    };
    indexers::index_track(
        &state.search,
        &track,
        artist_name.as_deref(),
        album_title.as_deref(),
    );
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
    let artist_name = match updated.artist_id {
        Some(aid) => ArtistRepository::find_by_id(&state.db, aid)
            .await?
            .map(|a| a.name),
        None => None,
    };
    indexers::index_album(&state.search, &updated, artist_name.as_deref());
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
    let artist_name = match created.artist_id {
        Some(aid) => ArtistRepository::find_by_id(&state.db, aid)
            .await?
            .map(|a| a.name),
        None => None,
    };
    indexers::index_album(&state.search, &created, artist_name.as_deref());
    Ok((StatusCode::CREATED, Json(created.into())))
}

pub async fn delete_album(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> AppResult<StatusCode> {
    let deleted = AlbumRepository::delete_if_empty(&state.db, id).await?;
    if deleted {
        indexers::remove(&state.search, SearchEntityType::Album, id);
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
    indexers::index_artist(&state.search, &updated);
    Ok(Json(updated.into()))
}

pub async fn create_artist(
    State(state): State<AppState>,
    Garde(Json(body)): Garde<Json<CreateArtistParams>>,
) -> AppResult<(StatusCode, Json<AdminArtistResponse>)> {
    let sort_name = body.sort_name.unwrap_or_else(|| body.name.clone());
    let artist = Artist::new(body.name, sort_name);
    let created = ArtistRepository::create(&state.db, &artist).await?;
    indexers::index_artist(&state.search, &created);
    Ok((StatusCode::CREATED, Json(created.into())))
}

pub async fn delete_artist(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> AppResult<StatusCode> {
    let deleted = ArtistRepository::delete_if_empty(&state.db, id).await?;
    if deleted {
        indexers::remove(&state.search, SearchEntityType::Artist, id);
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

// =====================================================================
// Merge
// =====================================================================

pub async fn merge_artist(
    State(state): State<AppState>,
    Path(target_id): Path<Uuid>,
    Garde(Json(body)): Garde<Json<MergeArtistParams>>,
) -> AppResult<Json<AdminArtistResponse>> {
    if body.source_id == target_id {
        return Err(AppError::BadRequest(
            "Cannot merge an artist into itself".into(),
        ));
    }

    let mut tx = state.db.begin().await?;

    let source = ArtistRepository::find_by_id(&mut *tx, body.source_id)
        .await?
        .ok_or(AppError::NotFound)?;
    let _target = ArtistRepository::find_by_id(&mut *tx, target_id)
        .await?
        .ok_or(AppError::NotFound)?;

    // Reject if source's albums would collide with target's on the
    // (source_album_artist_id, lower(source_title)) unique index. Admin must
    // merge those albums first.
    let collisions =
        ArtistRepository::find_album_source_title_collisions(&mut *tx, body.source_id, target_id)
            .await?;
    if !collisions.is_empty() {
        return Err(AppError::Conflict(format!(
            "Both artists have albums with the same title: {}. Merge those albums first.",
            collisions.join(", "),
        )));
    }

    // Re-point all referrers from source → target.
    TrackRepository::reassign_artist(&mut *tx, body.source_id, target_id).await?;
    ArtistRepository::reassign_albums_artist(&mut *tx, body.source_id, target_id).await?;
    AliasRepository::repoint_album_alias_artist(&mut *tx, body.source_id, target_id).await?;
    AliasRepository::repoint_artist_aliases(&mut *tx, body.source_id, target_id).await?;

    // Absorb the source's source_name so future scans map to target.
    AliasRepository::upsert_artist_alias(&mut *tx, &source.source_name, target_id).await?;

    // Apply admin-chosen field values to the target (sets locked_at = NOW()).
    let updated =
        ArtistRepository::update(&mut *tx, target_id, &body.name, &body.sort_name).await?;

    // Hard-delete source. All FKs were re-pointed above; CASCADE on
    // artist_aliases would have wiped its aliases, but we already moved them.
    ArtistRepository::delete(&mut *tx, body.source_id).await?;

    tx.commit().await?;
    indexers::remove(&state.search, SearchEntityType::Artist, body.source_id);
    indexers::index_artist(&state.search, &updated);
    // Tracks/albums that were re-pointed need their subtitles refreshed; queue
    // a full rebuild in the background to absorb that drift.
    let pool = state.db.clone();
    let search = state.search.clone();
    tokio::spawn(async move {
        if let Err(e) = crate::search::sync::rebuild_from_postgres(&pool, &search).await {
            tracing::warn!("search: post-merge rebuild failed: {}", e);
        }
    });
    Ok(Json(updated.into()))
}

// =====================================================================
// Approval
// =====================================================================

pub async fn approve_track(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> AppResult<Json<AdminTrackResponse>> {
    let updated = TrackRepository::set_approved(&state.db, id, true)
        .await?
        .ok_or(AppError::NotFound)?;
    Ok(Json(updated.into()))
}

pub async fn unapprove_track(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> AppResult<Json<AdminTrackResponse>> {
    let updated = TrackRepository::set_approved(&state.db, id, false)
        .await?
        .ok_or(AppError::NotFound)?;
    Ok(Json(updated.into()))
}

pub async fn approve_album(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> AppResult<Json<AdminAlbumResponse>> {
    let updated = AlbumRepository::set_approved(&state.db, id, true)
        .await?
        .ok_or(AppError::NotFound)?;
    Ok(Json(updated.into()))
}

pub async fn unapprove_album(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> AppResult<Json<AdminAlbumResponse>> {
    let updated = AlbumRepository::set_approved(&state.db, id, false)
        .await?
        .ok_or(AppError::NotFound)?;
    Ok(Json(updated.into()))
}

pub async fn approve_artist(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> AppResult<Json<AdminArtistResponse>> {
    let updated = ArtistRepository::set_approved(&state.db, id, true)
        .await?
        .ok_or(AppError::NotFound)?;
    Ok(Json(updated.into()))
}

pub async fn unapprove_artist(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> AppResult<Json<AdminArtistResponse>> {
    let updated = ArtistRepository::set_approved(&state.db, id, false)
        .await?
        .ok_or(AppError::NotFound)?;
    Ok(Json(updated.into()))
}

/// Approves the album, its (single) artist if still pending, and every
/// non-deleted track on it. Single transaction.
pub async fn approve_album_cascade(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> AppResult<Json<AdminAlbumResponse>> {
    let mut tx = state.db.begin().await?;

    let album = AlbumRepository::set_approved(&mut *tx, id, true)
        .await?
        .ok_or(AppError::NotFound)?;

    if let Some(artist_id) = album.artist_id {
        ArtistRepository::set_approved(&mut *tx, artist_id, true).await?;
    }

    TrackRepository::approve_all_for_album(&mut *tx, id).await?;

    tx.commit().await?;
    Ok(Json(album.into()))
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReviewQueueQuery {
    #[serde(default)]
    pub offset: Option<i64>,
    #[serde(default)]
    pub limit: Option<i64>,
}

const REVIEW_QUEUE_DEFAULT_LIMIT: i64 = 20;
const REVIEW_QUEUE_MAX_LIMIT: i64 = 100;
const STANDALONE_ARTISTS_CAP: i64 = 50;

pub async fn get_review_counts(
    State(state): State<AppState>,
) -> AppResult<Json<ReviewQueueCounts>> {
    Ok(Json(ReviewQueueCounts {
        pending_albums: AlbumRepository::count_pending(&state.db).await?,
        pending_tracks: TrackRepository::count_pending(&state.db).await?,
        pending_artists: ArtistRepository::count_pending(&state.db).await?,
    }))
}

pub async fn get_review_queue(
    State(state): State<AppState>,
    Query(q): Query<ReviewQueueQuery>,
) -> AppResult<Json<ReviewQueueResponse>> {
    let limit = q
        .limit
        .unwrap_or(REVIEW_QUEUE_DEFAULT_LIMIT)
        .clamp(1, REVIEW_QUEUE_MAX_LIMIT);
    let offset = q.offset.unwrap_or(0).max(0);

    let album_ids = AlbumRepository::find_pending_affected_ids(&state.db, limit, offset).await?;

    let mut album_groups = Vec::with_capacity(album_ids.len());
    for album_id in &album_ids {
        let album = AlbumRepository::find_by_id(&state.db, *album_id).await?;
        let artist = match album.artist_id {
            Some(aid) => ArtistRepository::find_by_id(&state.db, aid).await?,
            None => None,
        };
        let tracks = TrackRepository::find_by_album_id(&state.db, *album_id).await?;
        album_groups.push(ReviewQueueAlbumGroup {
            album: album.into(),
            artist: artist.map(Into::into),
            tracks: tracks.into_iter().map(Into::into).collect(),
        });
    }

    let standalone_artists = ArtistRepository::find_pending_excluding_album_artists(
        &state.db,
        &album_ids,
        STANDALONE_ARTISTS_CAP,
    )
    .await?;

    let counts = ReviewQueueCounts {
        pending_albums: AlbumRepository::count_pending(&state.db).await?,
        pending_tracks: TrackRepository::count_pending(&state.db).await?,
        pending_artists: ArtistRepository::count_pending(&state.db).await?,
    };

    Ok(Json(ReviewQueueResponse {
        albums: album_groups,
        standalone_artists: standalone_artists.into_iter().map(Into::into).collect(),
        counts,
    }))
}

pub async fn merge_album(
    State(state): State<AppState>,
    Path(target_id): Path<Uuid>,
    Garde(Json(body)): Garde<Json<MergeAlbumParams>>,
) -> AppResult<Json<AdminAlbumResponse>> {
    if body.source_id == target_id {
        return Err(AppError::BadRequest(
            "Cannot merge an album into itself".into(),
        ));
    }

    let mut tx = state.db.begin().await?;

    let source = AlbumRepository::find_by_id(&mut *tx, body.source_id).await?;
    let _target = AlbumRepository::find_by_id(&mut *tx, target_id).await?;

    // Re-point tracks from source → target.
    TrackRepository::reassign_album(&mut *tx, body.source_id, target_id).await?;
    AliasRepository::repoint_album_aliases(&mut *tx, body.source_id, target_id).await?;

    // Absorb the source's scanner keys so future scans map to target.
    AliasRepository::upsert_album_alias(
        tx.as_mut(),
        &source.source_title,
        source.source_album_artist_id,
        target_id,
    )
    .await?;

    // Apply admin-chosen field values to the target (sets locked_at = NOW()).
    AlbumRepository::update(
        &mut *tx,
        target_id,
        &body.title,
        &body.sort_title,
        body.artist_id,
        body.date,
    )
    .await?;

    // Cover is updated separately because the standard `update` method
    // doesn't touch it (the scanner owns covers; merge is the one admin path
    // that does).
    if let Some(cover) = body.cover_path.as_deref() {
        let cover_opt = if cover.is_empty() { None } else { Some(cover) };
        AlbumRepository::set_cover_path(&mut *tx, target_id, cover_opt).await?;
    }

    // Hard-delete source.
    AlbumRepository::delete(&mut *tx, body.source_id).await?;

    let updated = AlbumRepository::find_by_id(&mut *tx, target_id).await?;
    tx.commit().await?;
    indexers::remove(&state.search, SearchEntityType::Album, body.source_id);
    let artist_name = match updated.artist_id {
        Some(aid) => ArtistRepository::find_by_id(&state.db, aid)
            .await?
            .map(|a| a.name),
        None => None,
    };
    indexers::index_album(&state.search, &updated, artist_name.as_deref());
    let pool = state.db.clone();
    let search = state.search.clone();
    tokio::spawn(async move {
        if let Err(e) = crate::search::sync::rebuild_from_postgres(&pool, &search).await {
            tracing::warn!("search: post-merge rebuild failed: {}", e);
        }
    });
    Ok(Json(updated.into()))
}
