use std::str::FromStr;

use crate::{
    dto::{
        request::{SearchPaginationParams, SearchQuery},
        response::{
            AdminArtistResponse, AlbumResponse, ArtistResponse, PaginatedResponse, TrackResponse,
        },
    },
    error::{AppError, AppResult},
    repositories::{ArtistRepository, ArtistSortKey},
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
    Query(params): Query<SearchPaginationParams>,
) -> AppResult<Json<PaginatedResponse<ArtistResponse>>> {
    let sort = params
        .sort
        .as_deref()
        .map(ArtistSortKey::from_str)
        .transpose()?;
    let service = LibraryService::new(state.db.clone());
    let (artists, total) = service
        .get_artists_with_query(
            params.q.as_deref(),
            params.limit,
            params.offset,
            sort,
            params.dir,
        )
        .await?;

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

pub async fn get_artist_tracks(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> AppResult<Json<Vec<TrackResponse>>> {
    let service = LibraryService::new(state.db.clone());
    let tracks = service.get_tracks_by_artist_id(id).await?;

    Ok(Json(tracks.into_iter().map(Into::into).collect()))
}
