use std::collections::HashMap;
use std::sync::Arc;

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
        request::{PinPlaylistParams, ReorderPinnedPlaylistsParams},
        response::{AlbumResponse, HomeItemResponse, PlaylistResponse},
    },
    error::{AppError, AppResult},
    repositories::{
        PinnedPlaylistRepository, PlaylistRepository, TrackPlayRepository, MAX_PINNED_PLAYLISTS,
    },
    services::{LibraryService, RecommendationKind},
    state::AppState,
};

#[derive(serde::Deserialize)]
pub struct RecommendationQuery {
    #[serde(default = "default_recommendation_limit")]
    pub limit: i64,
}

fn default_recommendation_limit() -> i64 {
    20
}

/// Hydrates a list of album IDs into a HashMap of `id -> AlbumResponse` so the
/// caller can reorder by their own sort key without losing items.
async fn hydrate_albums(
    service: &LibraryService,
    ids: &[Uuid],
) -> AppResult<HashMap<Uuid, AlbumResponse>> {
    let bundled = service.get_albums_by_ids(ids).await?;
    Ok(bundled
        .into_iter()
        .map(|a| (a.album.id, a.into()))
        .collect())
}

/// Hydrates a list of playlist IDs into a HashMap of `id -> PlaylistResponse`.
/// Restricted to playlists owned by `user_id`.
async fn hydrate_playlists(
    state: &AppState,
    user_id: Uuid,
    ids: &[Uuid],
) -> AppResult<HashMap<Uuid, PlaylistResponse>> {
    let stats = PlaylistRepository::find_by_ids_for_user(&state.db, user_id, ids).await?;
    let plays = TrackPlayRepository::play_counts_for_playlists(&state.db, user_id, ids).await?;
    let plays_by_id: HashMap<Uuid, i64> = plays.into_iter().collect();
    Ok(stats
        .into_iter()
        .map(|s| {
            let pc = plays_by_id.get(&s.playlist.id).copied().unwrap_or(0);
            let id = s.playlist.id;
            (id, PlaylistResponse::from_stats(s, pc))
        })
        .collect())
}

pub async fn listen_again(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Query(params): Query<RecommendationQuery>,
) -> AppResult<Json<Arc<Vec<HomeItemResponse>>>> {
    let limit = params.limit.clamp(1, 100);
    let user_id = auth_user.user.id;

    if let Some(cached) = state
        .recommendation_cache
        .get(user_id, RecommendationKind::ListenAgain, limit)
        .await
    {
        return Ok(Json(cached));
    }

    let items = compute_listen_again(&state, user_id, limit).await?;
    let arc = state
        .recommendation_cache
        .put(user_id, RecommendationKind::ListenAgain, limit, items)
        .await;
    Ok(Json(arc))
}

pub async fn forgotten_favorites(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Query(params): Query<RecommendationQuery>,
) -> AppResult<Json<Arc<Vec<HomeItemResponse>>>> {
    let limit = params.limit.clamp(1, 100);
    let user_id = auth_user.user.id;

    if let Some(cached) = state
        .recommendation_cache
        .get(user_id, RecommendationKind::ForgottenFavorites, limit)
        .await
    {
        return Ok(Json(cached));
    }

    let items = compute_forgotten_favorites(&state, user_id, limit).await?;
    let arc = state
        .recommendation_cache
        .put(
            user_id,
            RecommendationKind::ForgottenFavorites,
            limit,
            items,
        )
        .await;
    Ok(Json(arc))
}

async fn compute_listen_again(
    state: &AppState,
    user_id: Uuid,
    limit: i64,
) -> AppResult<Vec<HomeItemResponse>> {
    let album_rows =
        TrackPlayRepository::recent_album_ids_with_time(&state.db, user_id, limit).await?;
    let playlist_rows =
        TrackPlayRepository::recent_playlist_ids_with_time(&state.db, user_id, limit).await?;

    let service = LibraryService::new(state.db.clone(), user_id);
    let album_ids: Vec<Uuid> = album_rows.iter().map(|(id, _)| *id).collect();
    let playlist_ids: Vec<Uuid> = playlist_rows.iter().map(|(id, _)| *id).collect();
    let mut albums = hydrate_albums(&service, &album_ids).await?;
    let mut playlists = hydrate_playlists(state, user_id, &playlist_ids).await?;

    let mut combined = Vec::with_capacity(album_rows.len() + playlist_rows.len());
    for (id, ts) in album_rows {
        if let Some(a) = albums.remove(&id) {
            combined.push((ts, HomeItemResponse::Album(a)));
        }
    }
    for (id, ts) in playlist_rows {
        if let Some(p) = playlists.remove(&id) {
            combined.push((ts, HomeItemResponse::Playlist(p)));
        }
    }
    combined.sort_by(|a, b| b.0.cmp(&a.0));
    combined.truncate(limit as usize);
    Ok(combined.into_iter().map(|(_, item)| item).collect())
}

async fn compute_forgotten_favorites(
    state: &AppState,
    user_id: Uuid,
    limit: i64,
) -> AppResult<Vec<HomeItemResponse>> {
    let album_rows =
        TrackPlayRepository::forgotten_album_ids_with_plays(&state.db, user_id, limit).await?;
    let playlist_rows =
        TrackPlayRepository::forgotten_playlist_ids_with_plays(&state.db, user_id, limit).await?;

    let service = LibraryService::new(state.db.clone(), user_id);
    let album_ids: Vec<Uuid> = album_rows.iter().map(|(id, _)| *id).collect();
    let playlist_ids: Vec<Uuid> = playlist_rows.iter().map(|(id, _)| *id).collect();
    let mut albums = hydrate_albums(&service, &album_ids).await?;
    let mut playlists = hydrate_playlists(state, user_id, &playlist_ids).await?;

    let mut combined = Vec::with_capacity(album_rows.len() + playlist_rows.len());
    for (id, plays) in album_rows {
        if let Some(a) = albums.remove(&id) {
            combined.push((plays, HomeItemResponse::Album(a)));
        }
    }
    for (id, plays) in playlist_rows {
        if let Some(p) = playlists.remove(&id) {
            combined.push((plays, HomeItemResponse::Playlist(p)));
        }
    }
    combined.sort_by(|a, b| b.0.cmp(&a.0));
    combined.truncate(limit as usize);
    Ok(combined.into_iter().map(|(_, item)| item).collect())
}

pub async fn list_pinned_playlists(
    State(state): State<AppState>,
    auth_user: AuthUser,
) -> AppResult<Json<Vec<PlaylistResponse>>> {
    let stats = PinnedPlaylistRepository::list_for_user(&state.db, auth_user.user.id).await?;

    let ids: Vec<Uuid> = stats.iter().map(|s| s.playlist.id).collect();
    let plays =
        TrackPlayRepository::play_counts_for_playlists(&state.db, auth_user.user.id, &ids).await?;
    let plays_by_id: HashMap<Uuid, i64> = plays.into_iter().collect();

    Ok(Json(
        stats
            .into_iter()
            .map(|s| {
                let pc = plays_by_id.get(&s.playlist.id).copied().unwrap_or(0);
                PlaylistResponse::from_stats(s, pc)
            })
            .collect(),
    ))
}

pub async fn pin_playlist(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Garde(Json(body)): Garde<Json<PinPlaylistParams>>,
) -> AppResult<StatusCode> {
    if !PlaylistRepository::belongs_to_user(&state.db, body.playlist_id, auth_user.user.id).await? {
        return Err(AppError::NotFound);
    }

    let current = PinnedPlaylistRepository::count_for_user(&state.db, auth_user.user.id).await?;
    if current >= MAX_PINNED_PLAYLISTS {
        return Err(AppError::BadRequest(format!(
            "cannot pin more than {MAX_PINNED_PLAYLISTS} playlists"
        )));
    }

    PinnedPlaylistRepository::insert(&state.db, auth_user.user.id, body.playlist_id).await?;
    Ok(StatusCode::NO_CONTENT)
}

pub async fn unpin_playlist(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(playlist_id): Path<Uuid>,
) -> AppResult<StatusCode> {
    let deleted =
        PinnedPlaylistRepository::delete(&state.db, auth_user.user.id, playlist_id).await?;
    if deleted {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(AppError::NotFound)
    }
}

pub async fn reorder_pinned_playlists(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Garde(Json(body)): Garde<Json<ReorderPinnedPlaylistsParams>>,
) -> AppResult<StatusCode> {
    PinnedPlaylistRepository::reorder(&state.db, auth_user.user.id, &body.ordered_playlist_ids)
        .await?;
    Ok(StatusCode::NO_CONTENT)
}
