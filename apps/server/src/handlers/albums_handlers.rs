use std::str::FromStr;

use crate::{
    auth::AuthUser,
    dto::{
        request::{FilteredPaginationParams, SearchQuery},
        response::{
            album_response::AlbumResponse, track_response::TrackResponse, AdminAlbumResponse,
            PaginatedResponse,
        },
    },
    error::{AppError, AppResult},
    repositories::{AlbumRepository, AlbumSortKey},
    services::LibraryService,
    state::AppState,
};
use axum::{
    extract::{Path, Query, State},
    Json,
};
use uuid::Uuid;

pub async fn get_albums(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Query(params): Query<FilteredPaginationParams>,
) -> AppResult<Json<PaginatedResponse<AlbumResponse>>> {
    let filter = params.decode_filter()?;
    let sort = params
        .sort
        .as_deref()
        .map(AlbumSortKey::from_str)
        .transpose()?;
    let service = LibraryService::new(state.db.clone(), auth_user.user.id);
    let (albums, total) = service
        .get_albums(
            filter.as_ref(),
            params.limit,
            params.offset,
            sort,
            params.dir,
        )
        .await?;

    let response = PaginatedResponse {
        data: albums.into_iter().map(Into::into).collect(),
        total,
        limit: params.limit,
        offset: params.offset,
    };

    Ok(Json(response))
}

pub async fn get_album(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(id): Path<Uuid>,
) -> AppResult<Json<AlbumResponse>> {
    let service = LibraryService::new(state.db.clone(), auth_user.user.id);
    let album = service
        .get_album_by_id(id)
        .await?
        .ok_or(AppError::NotFound)?;

    Ok(Json(album.into()))
}

pub async fn search_albums(
    State(state): State<AppState>,
    Query(params): Query<SearchQuery>,
) -> AppResult<Json<Vec<AdminAlbumResponse>>> {
    let limit = params.limit.clamp(1, 100);
    let albums = AlbumRepository::search(&state.db, &params.q, params.artist_id, limit).await?;
    Ok(Json(albums.into_iter().map(Into::into).collect()))
}

pub async fn get_album_tracks(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(id): Path<Uuid>,
) -> AppResult<Json<Vec<TrackResponse>>> {
    let service = LibraryService::new(state.db.clone(), auth_user.user.id);
    let tracks = service.get_tracks_by_album_id(id).await?;

    Ok(Json(tracks.into_iter().map(Into::into).collect()))
}
