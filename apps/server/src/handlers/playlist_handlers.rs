use std::str::FromStr;

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use axum_valid::Garde;
use uuid::Uuid;

use crate::{
    auth::AuthUser,
    dto::{
        request::{
            AddTracksParams, CreatePlaylistParams, RenamePlaylistParams, ReorderTrackParams,
            SearchPaginationParams, UpdatePlaylistFilterParams,
        },
        response::{
            PaginatedResponse, PlaylistResponse, PlaylistTrackResponse, RecentPlaylistResponse,
            SearchEntityType,
        },
    },
    error::{AppError, AppResult},
    repositories::{PlaylistRepository, PlaylistSortKey, TrackPlayRepository},
    search::indexers,
    state::AppState,
};

use std::collections::HashMap;

const INITIAL_POSITION: f64 = 1000.0;
const POSITION_STEP: f64 = 1000.0;
const MIN_GAP: f64 = 1e-9;

pub async fn list_playlists(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Query(params): Query<SearchPaginationParams>,
) -> AppResult<Json<PaginatedResponse<PlaylistResponse>>> {
    let sort = params
        .sort
        .as_deref()
        .map(PlaylistSortKey::from_str)
        .transpose()?;
    let playlists = PlaylistRepository::list_by_user_with_query(
        &state.db,
        auth_user.user.id,
        params.q.as_deref(),
        params.limit,
        params.offset,
        sort,
        params.dir,
        params.seed,
    )
    .await?;
    let total = PlaylistRepository::count_by_user_with_query(
        &state.db,
        auth_user.user.id,
        params.q.as_deref(),
    )
    .await?;

    let ids: Vec<Uuid> = playlists.iter().map(|p| p.playlist.id).collect();
    let plays =
        TrackPlayRepository::play_counts_for_playlists(&state.db, auth_user.user.id, &ids).await?;
    let plays_by_id: HashMap<Uuid, i64> = plays.into_iter().collect();

    Ok(Json(PaginatedResponse {
        data: playlists
            .into_iter()
            .map(|stats| {
                let pc = plays_by_id.get(&stats.playlist.id).copied().unwrap_or(0);
                PlaylistResponse::from_stats(stats, pc)
            })
            .collect(),
        total,
        limit: params.limit,
        offset: params.offset,
    }))
}

async fn playlist_play_count(
    pool: &sqlx::PgPool,
    user_id: Uuid,
    playlist_id: Uuid,
) -> AppResult<i64> {
    Ok(
        TrackPlayRepository::play_counts_for_playlists(pool, user_id, &[playlist_id])
            .await?
            .into_iter()
            .next()
            .map(|(_, c)| c)
            .unwrap_or(0),
    )
}

#[derive(serde::Deserialize)]
pub struct RecentQuery {
    #[serde(default = "default_recent_limit")]
    pub limit: i64,
}

fn default_recent_limit() -> i64 {
    10
}

pub async fn recent_playlists(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Query(params): Query<RecentQuery>,
) -> AppResult<Json<Vec<RecentPlaylistResponse>>> {
    let limit = params.limit.clamp(1, 50);
    let rows =
        TrackPlayRepository::recent_playlists_for_user(&state.db, auth_user.user.id, limit).await?;
    Ok(Json(
        rows.into_iter()
            .map(|(id, name)| RecentPlaylistResponse { id, name })
            .collect(),
    ))
}

pub async fn create_playlist(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Garde(Json(body)): Garde<Json<CreatePlaylistParams>>,
) -> AppResult<(StatusCode, Json<PlaylistResponse>)> {
    let playlist = PlaylistRepository::create(
        &state.db,
        auth_user.user.id,
        &body.name,
        body.filter_definition.as_ref(),
    )
    .await?;
    indexers::index_playlist(&state.search, &playlist);

    let stats = PlaylistRepository::find_by_id_and_user(&state.db, playlist.id, auth_user.user.id)
        .await?
        .ok_or(AppError::NotFound)?;

    let pc = playlist_play_count(&state.db, auth_user.user.id, stats.playlist.id).await?;
    Ok((
        StatusCode::CREATED,
        Json(PlaylistResponse::from_stats(stats, pc)),
    ))
}

pub async fn get_playlist(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(id): Path<Uuid>,
) -> AppResult<Json<PlaylistResponse>> {
    let stats = PlaylistRepository::find_by_id_and_user(&state.db, id, auth_user.user.id)
        .await?
        .ok_or(AppError::NotFound)?;

    let pc = playlist_play_count(&state.db, auth_user.user.id, stats.playlist.id).await?;
    Ok(Json(PlaylistResponse::from_stats(stats, pc)))
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

    let stats = PlaylistRepository::find_by_id_and_user(&state.db, id, auth_user.user.id)
        .await?
        .ok_or(AppError::NotFound)?;
    indexers::index_playlist(&state.search, &stats.playlist);

    let pc = playlist_play_count(&state.db, auth_user.user.id, stats.playlist.id).await?;
    Ok(Json(PlaylistResponse::from_stats(stats, pc)))
}

pub async fn delete_playlist(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(id): Path<Uuid>,
) -> AppResult<StatusCode> {
    let deleted = PlaylistRepository::delete(&state.db, id, auth_user.user.id).await?;

    if deleted {
        indexers::remove(&state.search, SearchEntityType::Playlist, id);
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
    let stats = PlaylistRepository::find_by_id_and_user(&state.db, id, auth_user.user.id)
        .await?
        .ok_or(AppError::NotFound)?;

    let tracks = if stats.playlist.playlist_type == "dynamic" {
        match stats.playlist.filter_definition.as_ref() {
            Some(filter) => {
                PlaylistRepository::list_dynamic_tracks(&state.db, id, &filter.0).await?
            }
            None => Vec::new(),
        }
    } else {
        PlaylistRepository::list_tracks(&state.db, id, auth_user.user.id).await?
    };

    Ok(Json(tracks.into_iter().map(Into::into).collect()))
}

pub async fn update_playlist_filter(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(id): Path<Uuid>,
    Garde(Json(body)): Garde<Json<UpdatePlaylistFilterParams>>,
) -> AppResult<Json<PlaylistResponse>> {
    PlaylistRepository::update_filter(&state.db, id, auth_user.user.id, &body.filter_definition)
        .await?
        .ok_or(AppError::NotFound)?;

    let stats = PlaylistRepository::find_by_id_and_user(&state.db, id, auth_user.user.id)
        .await?
        .ok_or(AppError::NotFound)?;

    let pc = playlist_play_count(&state.db, auth_user.user.id, stats.playlist.id).await?;
    Ok(Json(PlaylistResponse::from_stats(stats, pc)))
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
    let mut next_pos = max_pos
        .map(|p| p + POSITION_STEP)
        .unwrap_or(INITIAL_POSITION);

    for track_id in body.track_ids {
        // ON CONFLICT DO NOTHING in insert_track handles duplicates
        PlaylistRepository::insert_track(&state.db, id, track_id, next_pos).await?;
        next_pos += POSITION_STEP;
    }
    PlaylistRepository::touch(&state.db, id).await?;

    Ok(StatusCode::NO_CONTENT)
}

pub async fn remove_track(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path((id, tid)): Path<(Uuid, Uuid)>,
) -> AppResult<StatusCode> {
    let deleted = PlaylistRepository::remove_track(&state.db, id, auth_user.user.id, tid).await?;

    if deleted {
        PlaylistRepository::touch(&state.db, id).await?;
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

    let (prev, next) = PlaylistRepository::get_neighbor_positions(&mut tx, id, body.after_id)
        .await?
        .ok_or(AppError::NotFound)?;

    let new_position = compute_position(prev, next);

    // Check if gap is too small — redistribute then recompute
    let new_position = if let (Some(p), Some(n)) = (prev, next) {
        if (n - p).abs() < MIN_GAP {
            PlaylistRepository::redistribute_positions(&mut *tx, id).await?;
            let (prev2, next2) =
                PlaylistRepository::get_neighbor_positions(&mut tx, id, body.after_id)
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

    if updated {
        PlaylistRepository::touch(&mut *tx, id).await?;
    }

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
