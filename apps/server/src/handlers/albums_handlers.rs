use crate::{
    dto::{
        request::PaginationParams,
        response::{
            album_response::AlbumResponse,
            track_response::TrackResponse,
            PaginatedResponse,
        },
    },
    error::{AppError, AppResult},
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
    Query(params): Query<PaginationParams>,
) -> AppResult<Json<PaginatedResponse<AlbumResponse>>> {
    let service = LibraryService::new(state.db.clone());
    let (albums, total) = service.get_albums(params.limit, params.offset).await?;

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

pub async fn get_album_tracks(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> AppResult<Json<Vec<TrackResponse>>> {
    let service = LibraryService::new(state.db.clone());
    let tracks = service.get_tracks_by_album_id(id).await?;

    Ok(Json(tracks.into_iter().map(Into::into).collect()))
}
