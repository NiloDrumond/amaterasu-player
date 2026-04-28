use crate::{
    dto::{
        request::PaginationParams,
        response::{track_response::TrackResponse, PaginatedResponse},
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

pub async fn get_tracks(
    State(state): State<AppState>,
    Query(params): Query<PaginationParams>,
) -> AppResult<Json<PaginatedResponse<TrackResponse>>> {
    let service = LibraryService::new(state.db.clone());
    let (tracks, total) = service.get_tracks(params.limit, params.offset).await?;

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
