use crate::{
    dto::{
        request::{FilteredPaginationParams, SearchQuery},
        response::{
            album_response::AlbumResponse, track_response::TrackResponse, AdminAlbumResponse,
            PaginatedResponse,
        },
    },
    error::{AppError, AppResult},
    repositories::AlbumRepository,
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
    Query(params): Query<FilteredPaginationParams>,
) -> AppResult<Json<PaginatedResponse<AlbumResponse>>> {
    let filter = params.decode_filter()?;
    let service = LibraryService::new(state.db.clone());
    let (albums, total) = service
        .get_albums(filter.as_ref(), params.limit, params.offset)
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
    Path(id): Path<Uuid>,
) -> AppResult<Json<AlbumResponse>> {
    let service = LibraryService::new(state.db.clone());
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
    Path(id): Path<Uuid>,
) -> AppResult<Json<Vec<TrackResponse>>> {
    let service = LibraryService::new(state.db.clone());
    let tracks = service.get_tracks_by_album_id(id).await?;

    Ok(Json(tracks.into_iter().map(Into::into).collect()))
}
