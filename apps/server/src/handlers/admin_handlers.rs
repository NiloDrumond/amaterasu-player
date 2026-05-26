use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use axum_valid::Garde;
use uuid::Uuid;

use crate::{
    auth::middleware::AdminUser,
    db::entities::{Album, Artist},
    dto::{
        request::{
            BatchUpdateTracksParams, BatchUpdateTracksResponse, CreateAlbumParams,
            CreateArtistParams, HardDeleteQuery, MergeAlbumParams, MergeArtistParams,
            RegisterEmailParams, UpdateAlbumParams, UpdateArtistParams, UpdateTrackParams,
        },
        response::{
            AdminAlbumResponse, AdminArtistResponse, AdminDeletedTrackResponse, AdminTrackResponse,
            ReviewQueueAlbumGroup, ReviewQueueCounts, ReviewQueueResponse,
            ReviewQueueStandaloneArtist, SearchEntityType,
        },
    },
    error::{AppError, AppResult},
    repositories::track_repository::TrackUpdate,
    repositories::{
        AlbumRepository, AliasRepository, ArtistRepository, MetadataSuggestionRepository,
        SuggestionEntityType, TrackRepository,
    },
    search::indexers,
    services::auth_service::AuthService,
    state::AppState,
};

use serde::Deserialize;
use sqlx::PgPool;

async fn artist_with_aliases(
    db: &PgPool,
    artist: crate::db::entities::Artist,
) -> AppResult<AdminArtistResponse> {
    let aliases = AliasRepository::find_artist_aliases_by_artist_id(db, artist.id).await?;
    Ok(AdminArtistResponse::from(artist).with_aliases(aliases))
}

async fn album_with_aliases(db: &PgPool, album: Album) -> AppResult<AdminAlbumResponse> {
    let aliases = AliasRepository::find_album_aliases_by_album_id(db, album.id).await?;
    Ok(AdminAlbumResponse::from(album).with_aliases(aliases))
}

pub async fn create_user(
    State(state): State<AppState>,
    _admin: AdminUser,
    Garde(Json(body)): Garde<Json<RegisterEmailParams>>,
) -> AppResult<StatusCode> {
    let service = AuthService::new(state.db.clone());
    service
        .register_email(body.email, body.name, body.password)
        .await?;
    Ok(StatusCode::CREATED)
}

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

    let tracks = TrackRepository::find_by_ids(&state.db, &body.ids).await?;
    let artist_ids: Vec<Uuid> = tracks.iter().filter_map(|t| t.artist_id).collect();
    let album_ids: Vec<Uuid> = tracks.iter().filter_map(|t| t.album_id).collect();
    let artists = ArtistRepository::find_by_ids(&state.db, &artist_ids).await?;
    let albums = AlbumRepository::find_by_ids(&state.db, &album_ids).await?;
    let artist_map: std::collections::HashMap<Uuid, &str> =
        artists.iter().map(|a| (a.id, a.name.as_str())).collect();
    let album_map: std::collections::HashMap<Uuid, &str> =
        albums.iter().map(|a| (a.id, a.title.as_str())).collect();
    for t in &tracks {
        let artist_name = t.artist_id.and_then(|id| artist_map.get(&id).copied());
        let album_title = t.album_id.and_then(|id| album_map.get(&id).copied());
        indexers::index_track(&state.search, t, artist_name, album_title);
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
    Ok(Json(album_with_aliases(&state.db, album).await?))
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
    Ok(Json(album_with_aliases(&state.db, updated).await?))
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
    let mut tx = state.db.begin().await?;
    let created = AlbumRepository::create(&mut *tx, &album).await?;
    AliasRepository::insert_album_alias(&mut *tx, &created.title, created.artist_id, created.id)
        .await?;
    tx.commit().await?;
    let artist_name = match created.artist_id {
        Some(aid) => ArtistRepository::find_by_id(&state.db, aid)
            .await?
            .map(|a| a.name),
        None => None,
    };
    indexers::index_album(&state.search, &created, artist_name.as_deref());
    Ok((
        StatusCode::CREATED,
        Json(album_with_aliases(&state.db, created).await?),
    ))
}

pub async fn delete_album(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> AppResult<StatusCode> {
    let mut tx = state.db.begin().await?;
    let exists_before = AlbumRepository::find_by_id(&mut *tx, id).await.is_ok();
    // Clean up MB suggestions before the album row goes -- the table has no
    // FK so we'd otherwise leak orphan rows.
    if exists_before {
        MetadataSuggestionRepository::delete_for_entity(&mut tx, SuggestionEntityType::Album, id)
            .await?;
    }
    let deleted = AlbumRepository::delete_if_empty(&mut *tx, id).await?;
    tx.commit().await?;
    if deleted {
        indexers::remove(&state.search, SearchEntityType::Album, id);
        Ok(StatusCode::NO_CONTENT)
    } else if !exists_before {
        Err(AppError::NotFound)
    } else {
        Err(AppError::AlbumNotEmpty)
    }
}

pub async fn force_rescan_album(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> AppResult<StatusCode> {
    AlbumRepository::clear_lock(&state.db, id).await?;
    Ok(StatusCode::NO_CONTENT)
}

/// Accepts a multipart upload with one file field, stores it as the album's
/// cover. Reuses `services::cover_storage` so the on-disk shape matches the
/// MB/CAA download path exactly (sha256-named, dedupe-on-disk).
pub async fn upload_album_cover(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    mut multipart: axum::extract::Multipart,
) -> AppResult<Json<AdminAlbumResponse>> {
    // 404 early if the album doesn't exist -- avoids writing an orphan file.
    AlbumRepository::find_by_id(&state.db, id).await?;

    let mut bytes: Option<Vec<u8>> = None;
    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|e| AppError::BadRequest(format!("invalid multipart: {e}")))?
    {
        let data = field
            .bytes()
            .await
            .map_err(|e| AppError::BadRequest(format!("failed to read upload: {e}")))?;
        if !data.is_empty() {
            bytes = Some(data.to_vec());
            break; // first non-empty field wins
        }
    }
    let bytes = bytes.ok_or_else(|| AppError::BadRequest("no file uploaded".to_string()))?;

    let filename = crate::services::cover_storage::store_cover_bytes(&state.covers_dir, &bytes)
        .await
        .map_err(|e| match e {
            crate::services::cover_storage::CoverStorageError::UnsupportedImageType
            | crate::services::cover_storage::CoverStorageError::TooLarge(_) => {
                AppError::BadRequest(e.to_string())
            }
            crate::services::cover_storage::CoverStorageError::Io(io) => {
                AppError::Internal(anyhow::anyhow!("write cover: {io}"))
            }
        })?;

    AlbumRepository::set_cover_path(&state.db, id, Some(filename.as_str())).await?;
    let updated = AlbumRepository::find_by_id(&state.db, id).await?;
    Ok(Json(album_with_aliases(&state.db, updated).await?))
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
    Ok(Json(artist_with_aliases(&state.db, artist).await?))
}

pub async fn update_artist(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Garde(Json(body)): Garde<Json<UpdateArtistParams>>,
) -> AppResult<Json<AdminArtistResponse>> {
    let updated = ArtistRepository::update(&state.db, id, &body.name, &body.sort_name).await?;
    indexers::index_artist(&state.search, &updated);
    Ok(Json(artist_with_aliases(&state.db, updated).await?))
}

pub async fn create_artist(
    State(state): State<AppState>,
    Garde(Json(body)): Garde<Json<CreateArtistParams>>,
) -> AppResult<(StatusCode, Json<AdminArtistResponse>)> {
    let sort_name = body.sort_name.unwrap_or_else(|| body.name.clone());
    let artist = Artist::new(body.name, sort_name);
    let mut tx = state.db.begin().await?;
    let created = ArtistRepository::create(&mut *tx, &artist).await?;
    // New artist gets one alias from the name; future scans of files tagged
    // with this exact name resolve to this artist.
    AliasRepository::insert_artist_alias(&mut *tx, &created.name, created.id).await?;
    tx.commit().await?;
    indexers::index_artist(&state.search, &created);
    Ok((
        StatusCode::CREATED,
        Json(artist_with_aliases(&state.db, created).await?),
    ))
}

pub async fn delete_artist(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> AppResult<StatusCode> {
    let mut tx = state.db.begin().await?;
    let exists_before = ArtistRepository::find_by_id(&mut *tx, id).await?.is_some();
    if exists_before {
        MetadataSuggestionRepository::delete_for_entity(&mut tx, SuggestionEntityType::Artist, id)
            .await?;
    }
    let deleted = ArtistRepository::delete_if_empty(&mut *tx, id).await?;
    tx.commit().await?;
    if deleted {
        indexers::remove(&state.search, SearchEntityType::Artist, id);
        Ok(StatusCode::NO_CONTENT)
    } else if !exists_before {
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

    let _source = ArtistRepository::find_by_id(&mut *tx, body.source_id)
        .await?
        .ok_or(AppError::NotFound)?;
    let _target = ArtistRepository::find_by_id(&mut *tx, target_id)
        .await?
        .ok_or(AppError::NotFound)?;

    // Reject if any source_title is shared between the two artists' alias
    // rows. Repointing source_album_artist_id from `source` to `target`
    // would violate the (source_album_artist_id, lower(source_title))
    // partial unique index on album_aliases.
    let collisions =
        AliasRepository::find_album_alias_artist_collisions(&mut *tx, body.source_id, target_id)
            .await?;
    if !collisions.is_empty() {
        return Err(AppError::Conflict(format!(
            "Both artists have albums with the same title: {}. Merge those albums first.",
            collisions.join(", "),
        )));
    }

    // Re-point all referrers from source → target. Aliases carry the
    // scan-side identity now; the source's artist_aliases rows move with
    // `repoint_artist_aliases`, and the album_aliases rows that named
    // `source` as their source_artist move via `repoint_album_alias_artist`.
    TrackRepository::reassign_artist(&mut *tx, body.source_id, target_id).await?;
    ArtistRepository::reassign_albums_artist(&mut *tx, body.source_id, target_id).await?;
    AliasRepository::repoint_album_alias_artist(&mut *tx, body.source_id, target_id).await?;
    AliasRepository::repoint_artist_aliases(&mut *tx, body.source_id, target_id).await?;

    // Apply admin-chosen field values to the target (sets locked_at = NOW()).
    let updated =
        ArtistRepository::update(&mut *tx, target_id, &body.name, &body.sort_name).await?;

    // Hard-delete source. All FKs were re-pointed above.
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
    Ok(Json(artist_with_aliases(&state.db, updated).await?))
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
    Ok(Json(album_with_aliases(&state.db, updated).await?))
}

pub async fn unapprove_album(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> AppResult<Json<AdminAlbumResponse>> {
    let updated = AlbumRepository::set_approved(&state.db, id, false)
        .await?
        .ok_or(AppError::NotFound)?;
    Ok(Json(album_with_aliases(&state.db, updated).await?))
}

pub async fn approve_artist(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> AppResult<Json<AdminArtistResponse>> {
    let updated = ArtistRepository::set_approved(&state.db, id, true)
        .await?
        .ok_or(AppError::NotFound)?;
    Ok(Json(artist_with_aliases(&state.db, updated).await?))
}

pub async fn unapprove_artist(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> AppResult<Json<AdminArtistResponse>> {
    let updated = ArtistRepository::set_approved(&state.db, id, false)
        .await?
        .ok_or(AppError::NotFound)?;
    Ok(Json(artist_with_aliases(&state.db, updated).await?))
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

    use std::collections::{HashMap, HashSet};

    let album_ids = AlbumRepository::find_pending_affected_ids(&state.db, limit, offset).await?;

    // Batch-fetch albums + their tracks in two queries instead of N+1.
    let albums = AlbumRepository::find_by_ids(&state.db, &album_ids).await?;
    let album_map: HashMap<Uuid, &Album> = albums.iter().map(|a| (a.id, a)).collect();
    let all_album_tracks = TrackRepository::find_by_album_ids(&state.db, &album_ids).await?;
    let mut tracks_by_album: HashMap<Uuid, Vec<_>> = HashMap::new();
    for t in &all_album_tracks {
        if let Some(aid) = t.album_id {
            tracks_by_album.entry(aid).or_default().push(t);
        }
    }

    // Collect every artist ID we'll need, then batch-fetch all artists once.
    let mut needed_artist_ids: HashSet<Uuid> = HashSet::new();
    for a in &albums {
        if let Some(aid) = a.artist_id {
            needed_artist_ids.insert(aid);
        }
    }
    for t in &all_album_tracks {
        if let Some(aid) = t.artist_id {
            needed_artist_ids.insert(aid);
        }
    }

    // Standalone artists: pending artists whose albums aren't in the album queue.
    let standalone_artists_raw = ArtistRepository::find_pending_excluding_album_artists(
        &state.db,
        &album_ids,
        STANDALONE_ARTISTS_CAP,
    )
    .await?;
    let standalone_artist_ids: Vec<Uuid> = standalone_artists_raw.iter().map(|a| a.id).collect();

    // Fetch tracks for standalone artists in one query.
    let standalone_tracks =
        TrackRepository::find_by_artist_ids(&state.db, &standalone_artist_ids).await?;
    let mut tracks_by_artist: HashMap<Uuid, Vec<_>> = HashMap::new();
    for t in &standalone_tracks {
        if let Some(aid) = t.artist_id {
            tracks_by_artist.entry(aid).or_default().push(t);
        }
    }

    // Collect album IDs from standalone tracks for their track_albums.
    let mut standalone_album_ids: HashSet<Uuid> = HashSet::new();
    for t in &standalone_tracks {
        if let Some(aid) = t.album_id {
            standalone_album_ids.insert(aid);
        }
    }
    let standalone_albums = AlbumRepository::find_by_ids(
        &state.db,
        &standalone_album_ids.into_iter().collect::<Vec<_>>(),
    )
    .await?;
    let standalone_album_map: HashMap<Uuid, &Album> =
        standalone_albums.iter().map(|a| (a.id, a)).collect();

    for a in &standalone_artists_raw {
        needed_artist_ids.insert(a.id);
    }

    // Single batch fetch for all artists.
    let all_artist_ids_vec: Vec<Uuid> = needed_artist_ids.into_iter().collect();
    let all_artists = ArtistRepository::find_by_ids(&state.db, &all_artist_ids_vec).await?;
    let artist_map: HashMap<Uuid, &Artist> = all_artists.iter().map(|a| (a.id, a)).collect();

    // Bulk fetch aliases.
    let mut all_album_ids: Vec<Uuid> = album_ids.clone();
    all_album_ids.extend(standalone_albums.iter().map(|a| a.id));
    let artist_aliases =
        AliasRepository::find_artist_aliases_by_artist_ids(&state.db, &all_artist_ids_vec).await?;
    let album_aliases =
        AliasRepository::find_album_aliases_by_album_ids(&state.db, &all_album_ids).await?;
    let mut artist_alias_buckets: HashMap<Uuid, Vec<_>> = HashMap::new();
    for a in artist_aliases {
        artist_alias_buckets.entry(a.artist_id).or_default().push(a);
    }
    let mut album_alias_buckets: HashMap<Uuid, Vec<_>> = HashMap::new();
    for a in album_aliases {
        album_alias_buckets.entry(a.album_id).or_default().push(a);
    }

    let take_artist_aliases = |buckets: &mut HashMap<Uuid, Vec<_>>, id: Uuid| -> Vec<_> {
        buckets.remove(&id).unwrap_or_default()
    };
    let take_album_aliases = |buckets: &mut HashMap<Uuid, Vec<_>>, id: Uuid| -> Vec<_> {
        buckets.remove(&id).unwrap_or_default()
    };

    // Assemble album groups.
    let mut album_groups: Vec<ReviewQueueAlbumGroup> = Vec::with_capacity(album_ids.len());
    for album_id in &album_ids {
        let Some(album) = album_map.get(album_id) else {
            continue;
        };
        let album_aliases_for = take_album_aliases(&mut album_alias_buckets, album.id);
        let album_resp = AdminAlbumResponse::from((*album).clone()).with_aliases(album_aliases_for);

        let artist_resp = album.artist_id.and_then(|aid| {
            artist_map.get(&aid).map(|a| {
                let aliases = take_artist_aliases(&mut artist_alias_buckets, a.id);
                AdminArtistResponse::from((*a).clone()).with_aliases(aliases)
            })
        });

        let tracks = tracks_by_album.remove(album_id).unwrap_or_default();
        let album_artist_id = album.artist_id;
        let mut seen: HashSet<Uuid> = HashSet::new();
        let mut track_artists_resp: Vec<AdminArtistResponse> = Vec::new();
        for t in &tracks {
            let Some(tid) = t.artist_id else { continue };
            if Some(tid) == album_artist_id || !seen.insert(tid) {
                continue;
            }
            if let Some(a) = artist_map.get(&tid) {
                let aliases = take_artist_aliases(&mut artist_alias_buckets, a.id);
                track_artists_resp
                    .push(AdminArtistResponse::from((*a).clone()).with_aliases(aliases));
            }
        }

        album_groups.push(ReviewQueueAlbumGroup {
            album: album_resp,
            artist: artist_resp,
            tracks: tracks.into_iter().cloned().map(Into::into).collect(),
            track_artists: track_artists_resp,
        });
    }

    // Assemble standalone artists.
    let mut standalone_artists: Vec<ReviewQueueStandaloneArtist> =
        Vec::with_capacity(standalone_artists_raw.len());
    for artist in standalone_artists_raw {
        let artist_aliases_for = take_artist_aliases(&mut artist_alias_buckets, artist.id);
        let artist_resp =
            AdminArtistResponse::from(artist.clone()).with_aliases(artist_aliases_for);

        let tracks = tracks_by_artist.remove(&artist.id).unwrap_or_default();
        let mut seen_album_ids: HashSet<Uuid> = HashSet::new();
        for t in &tracks {
            if let Some(aid) = t.album_id {
                seen_album_ids.insert(aid);
            }
        }
        let track_albums_resp: Vec<AdminAlbumResponse> = seen_album_ids
            .into_iter()
            .filter_map(|aid| standalone_album_map.get(&aid))
            .map(|al| {
                let aliases = take_album_aliases(&mut album_alias_buckets, al.id);
                AdminAlbumResponse::from((*al).clone()).with_aliases(aliases)
            })
            .collect();

        standalone_artists.push(ReviewQueueStandaloneArtist {
            artist: artist_resp,
            tracks: tracks.into_iter().cloned().map(Into::into).collect(),
            track_albums: track_albums_resp,
        });
    }

    let counts = ReviewQueueCounts {
        pending_albums: AlbumRepository::count_pending(&state.db).await?,
        pending_tracks: TrackRepository::count_pending(&state.db).await?,
        pending_artists: ArtistRepository::count_pending(&state.db).await?,
    };

    Ok(Json(ReviewQueueResponse {
        albums: album_groups,
        standalone_artists,
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

    let _source = AlbumRepository::find_by_id(&mut *tx, body.source_id).await?;
    let _target = AlbumRepository::find_by_id(&mut *tx, target_id).await?;

    // Re-point tracks and aliases from source → target. The source album's
    // scanner keys live in album_aliases (post-demotion); moving them with
    // the rest preserves scan stability for files originally tagged for
    // either album.
    TrackRepository::reassign_album(&mut *tx, body.source_id, target_id).await?;
    AliasRepository::repoint_album_aliases(&mut *tx, body.source_id, target_id).await?;

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
    Ok(Json(album_with_aliases(&state.db, updated).await?))
}
