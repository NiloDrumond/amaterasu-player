//! HTTP surface for MusicBrainz suggestion lookups, acceptance, and rejection.

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use serde::Deserialize;
use uuid::Uuid;

use crate::dto::response::{
    AdminAlbumResponse, AdminArtistResponse, AdminTrackResponse, BulkMbLookupResponse,
    MetadataSuggestionResponse,
};
use crate::error::{AppError, AppResult};
use crate::musicbrainz::{AcceptedEntity, LookupJob};
use crate::repositories::{
    AliasRepository, ArtistRepository, MbLookupStatusRepository, MetadataSuggestionRepository,
    SuggestionEntityType,
};
use crate::search::indexers;
use crate::state::AppState;

/// Cap on how many entities the bulk endpoint enqueues per call. Each entity
/// burns ~1 second of MB budget; this keeps a single bulk call from filling
/// the worker queue or dominating the rate limiter for too long.
const BULK_CAP_PER_KIND: i64 = 500;

// =====================================================================
// Per-entity lookups: enqueue jobs onto the worker.
// =====================================================================

pub async fn lookup_album(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> AppResult<StatusCode> {
    // Manual admin trigger: force=true so an already-'found' entity still
    // gets re-queried (the admin clicked the button intentionally).
    enqueue(&state, LookupJob::Album { id, force: true })
}

pub async fn lookup_artist(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> AppResult<StatusCode> {
    enqueue(&state, LookupJob::Artist { id, force: true })
}

pub async fn lookup_track(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> AppResult<StatusCode> {
    enqueue(&state, LookupJob::Track { id, force: true })
}

fn enqueue(state: &AppState, job: LookupJob) -> AppResult<StatusCode> {
    let sender = state
        .mb_lookup_sender
        .as_ref()
        .ok_or_else(|| AppError::BadRequest("MusicBrainz integration is disabled".to_string()))?;
    if sender.enqueue(job) {
        Ok(StatusCode::ACCEPTED)
    } else {
        // Either the channel is full or the worker is gone. Surface as 503
        // so the frontend retries instead of treating it as a hard error.
        Err(AppError::BadRequest(
            "MusicBrainz worker queue is full; try again shortly".to_string(),
        ))
    }
}

// =====================================================================
// Bulk lookup: enqueue every entity with mb_lookup_status null/failed.
// =====================================================================

pub async fn bulk_lookup_pending(
    State(state): State<AppState>,
) -> AppResult<Json<BulkMbLookupResponse>> {
    let sender = state
        .mb_lookup_sender
        .as_ref()
        .ok_or_else(|| AppError::BadRequest("MusicBrainz integration is disabled".to_string()))?;

    let album_ids =
        MbLookupStatusRepository::pending_album_ids(&state.db, BULK_CAP_PER_KIND).await?;
    let artist_ids =
        MbLookupStatusRepository::pending_artist_ids(&state.db, BULK_CAP_PER_KIND).await?;

    let mut enqueued = 0i64;
    // Bulk retry: force=false so already-'found' entries are no-ops at the
    // service layer. The query is already filtered to NULL/failed, but
    // belt-and-suspenders.
    for id in album_ids {
        if sender.enqueue(LookupJob::Album { id, force: false }) {
            enqueued += 1;
        } else {
            break;
        }
    }
    for id in artist_ids {
        if sender.enqueue(LookupJob::Artist { id, force: false }) {
            enqueued += 1;
        } else {
            break;
        }
    }
    Ok(Json(BulkMbLookupResponse { enqueued }))
}

// =====================================================================
// Read suggestions.
// =====================================================================

pub async fn list_album_suggestions(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> AppResult<Json<Vec<MetadataSuggestionResponse>>> {
    list_for(&state, SuggestionEntityType::Album, id).await
}

pub async fn list_artist_suggestions(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> AppResult<Json<Vec<MetadataSuggestionResponse>>> {
    list_for(&state, SuggestionEntityType::Artist, id).await
}

pub async fn list_track_suggestions(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> AppResult<Json<Vec<MetadataSuggestionResponse>>> {
    list_for(&state, SuggestionEntityType::Track, id).await
}

async fn list_for(
    state: &AppState,
    entity_type: SuggestionEntityType,
    id: Uuid,
) -> AppResult<Json<Vec<MetadataSuggestionResponse>>> {
    let suggestions =
        MetadataSuggestionRepository::find_by_entity(&state.db, entity_type, id).await?;
    Ok(Json(suggestions.into_iter().map(Into::into).collect()))
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReviewQueueSuggestionsQuery {
    /// Comma-separated list of album UUIDs. Empty -> empty result.
    #[serde(default)]
    pub album_ids: Option<String>,
}

/// Batch fetch used by the review-queue page-server load. Returns a flat list
/// of pending album suggestions for the supplied album IDs; the frontend
/// buckets by `entityId`.
pub async fn review_queue_album_suggestions(
    State(state): State<AppState>,
    Query(q): Query<ReviewQueueSuggestionsQuery>,
) -> AppResult<Json<Vec<MetadataSuggestionResponse>>> {
    let ids: Vec<Uuid> = q
        .album_ids
        .as_deref()
        .map(|s| {
            s.split(',')
                .filter_map(|p| Uuid::parse_str(p.trim()).ok())
                .collect()
        })
        .unwrap_or_default();
    if ids.is_empty() {
        return Ok(Json(Vec::new()));
    }
    let rows = MetadataSuggestionRepository::find_pending_by_entity_ids(
        &state.db,
        SuggestionEntityType::Album,
        &ids,
    )
    .await?;
    Ok(Json(rows.into_iter().map(Into::into).collect()))
}

// =====================================================================
// Accept / reject.
// =====================================================================

pub async fn accept_suggestion(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> AppResult<axum::response::Response> {
    let service = state
        .mb_service
        .as_ref()
        .ok_or_else(|| AppError::BadRequest("MusicBrainz integration is disabled".to_string()))?;
    let accepted = service.accept_suggestion(id).await?;
    // Re-index in the search index so admins see the new metadata immediately.
    use axum::response::IntoResponse;
    let response = match accepted {
        AcceptedEntity::Album(album) => {
            let artist_name = match album.artist_id {
                Some(aid) => ArtistRepository::find_by_id(&state.db, aid)
                    .await?
                    .map(|a| a.name),
                None => None,
            };
            indexers::index_album(&state.search, &album, artist_name.as_deref());
            let aliases =
                AliasRepository::find_album_aliases_by_album_id(&state.db, album.id).await?;
            let resp = AdminAlbumResponse::from(*album).with_aliases(aliases);
            Json(resp).into_response()
        }
        AcceptedEntity::Artist(artist) => {
            indexers::index_artist(&state.search, &artist);
            let aliases =
                AliasRepository::find_artist_aliases_by_artist_id(&state.db, artist.id).await?;
            let resp = AdminArtistResponse::from(*artist).with_aliases(aliases);
            Json(resp).into_response()
        }
        AcceptedEntity::Track(track) => {
            let artist_name = match track.artist_id {
                Some(aid) => ArtistRepository::find_by_id(&state.db, aid)
                    .await?
                    .map(|a| a.name),
                None => None,
            };
            let album_title = match track.album_id {
                Some(aid) => crate::repositories::AlbumRepository::find_by_id(&state.db, aid)
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
            let resp: AdminTrackResponse = (*track).into();
            Json(resp).into_response()
        }
    };
    Ok(response)
}

pub async fn reject_suggestion(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> AppResult<StatusCode> {
    let service = state
        .mb_service
        .as_ref()
        .ok_or_else(|| AppError::BadRequest("MusicBrainz integration is disabled".to_string()))?;
    service.reject_suggestion(id).await?;
    Ok(StatusCode::NO_CONTENT)
}
