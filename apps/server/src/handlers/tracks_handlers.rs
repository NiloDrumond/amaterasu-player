use std::str::FromStr;

use crate::{
    auth::AuthUser,
    dto::{
        request::{FilteredPaginationParams, ScrobbleParams},
        response::{PaginatedResponse, TrackResponse},
    },
    error::{AppError, AppResult},
    repositories::{TrackPlayRepository, TrackSortKey},
    services::LibraryService,
    state::AppState,
};
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use axum_valid::Garde;
use uuid::Uuid;

pub async fn get_tracks(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Query(params): Query<FilteredPaginationParams>,
) -> AppResult<Json<PaginatedResponse<TrackResponse>>> {
    let filter = params.decode_filter()?;
    let sort = params
        .sort
        .as_deref()
        .map(TrackSortKey::from_str)
        .transpose()?;
    let service = LibraryService::new(state.db.clone(), auth_user.user.id);
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
    auth_user: AuthUser,
    Path(id): Path<Uuid>,
) -> AppResult<Json<TrackResponse>> {
    let service = LibraryService::new(state.db.clone(), auth_user.user.id);
    let track = service
        .get_track_by_id(id)
        .await?
        .ok_or(AppError::NotFound)?;

    Ok(Json(track.into()))
}

pub async fn scrobble_track(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(id): Path<Uuid>,
    Garde(Json(body)): Garde<Json<ScrobbleParams>>,
) -> AppResult<StatusCode> {
    TrackPlayRepository::record(
        &state.db,
        auth_user.user.id,
        id,
        body.context_album_id,
        body.context_playlist_id,
    )
    .await?;
    Ok(StatusCode::NO_CONTENT)
}
