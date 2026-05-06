use crate::{
    dto::{
        request::{PaginationParams, SearchQuery},
        response::{AdminArtistResponse, AlbumResponse, ArtistResponse, PaginatedResponse},
    },
    error::{AppError, AppResult},
    repositories::ArtistRepository,
    services::LibraryService,
    state::AppState,
};
use axum::{
    extract::{Path, Query, State},
    Json,
};
use uuid::Uuid;

pub async fn get_artists(
    State(state): State<AppState>,
    Query(params): Query<PaginationParams>,
) -> AppResult<Json<PaginatedResponse<ArtistResponse>>> {
    let service = LibraryService::new(state.db.clone());
    let (artists, total) = service.get_artists(params.limit, params.offset).await?;

    let response = PaginatedResponse {
        data: artists.into_iter().map(Into::into).collect(),
        total,
        limit: params.limit,
        offset: params.offset,
    };

    Ok(Json(response))
}

pub async fn get_artist(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> AppResult<Json<ArtistResponse>> {
    let service = LibraryService::new(state.db.clone());
    let artist = service
        .get_artist_by_id(id)
        .await?
        .ok_or(AppError::NotFound)?;

    Ok(Json(artist.into()))
}

pub async fn search_artists(
    State(state): State<AppState>,
    Query(params): Query<SearchQuery>,
) -> AppResult<Json<Vec<AdminArtistResponse>>> {
    let limit = params.limit.clamp(1, 100);
    let artists = ArtistRepository::search(&state.db, &params.q, limit).await?;
    Ok(Json(artists.into_iter().map(Into::into).collect()))
}

pub async fn get_artist_albums(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> AppResult<Json<Vec<AlbumResponse>>> {
    let service = LibraryService::new(state.db.clone());
    let albums = service.get_albums_by_artist_id(id).await?;

    Ok(Json(albums.into_iter().map(Into::into).collect()))
}
