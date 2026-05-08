use std::str::FromStr;

use crate::{
    dto::{
        request::FilteredPaginationParams,
        response::{PaginatedResponse, TrackResponse},
    },
    error::{AppError, AppResult},
    repositories::TrackSortKey,
    services::LibraryService,
    state::AppState,
};
use axum::{
    extract::{Path, Query, State},
    Json,
};
use uuid::Uuid;

pub async fn get_tracks(
    State(state): State<AppState>,
    Query(params): Query<FilteredPaginationParams>,
) -> AppResult<Json<PaginatedResponse<TrackResponse>>> {
    let filter = params.decode_filter()?;
    let sort = params
        .sort
        .as_deref()
        .map(TrackSortKey::from_str)
        .transpose()?;
    let service = LibraryService::new(state.db.clone());
    let (tracks, total) = service
        .get_tracks(
            filter.as_ref(),
            params.limit,
            params.offset,
            sort,
            params.dir,
        )
        .await?;

    let response = PaginatedResponse {
        data: tracks.into_iter().map(Into::into).collect(),
        total,
        limit: params.limit,
        offset: params.offset,
    };

    Ok(Json(response))
}

pub async fn get_track(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> AppResult<Json<TrackResponse>> {
    let service = LibraryService::new(state.db.clone());
    let track = service
        .get_track_by_id(id)
        .await?
        .ok_or(AppError::NotFound)?;

    Ok(Json(track.into()))
}
