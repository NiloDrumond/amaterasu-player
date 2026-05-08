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
            CreateAlbumCollectionParams, FilteredPaginationParams, RenameAlbumCollectionParams,
            UpdateAlbumCollectionFilterParams,
        },
        response::{AlbumCollectionResponse, AlbumResponse, PaginatedResponse},
    },
    error::{AppError, AppResult},
    repositories::{AlbumCollectionRepository, AlbumSortKey},
    services::LibraryService,
    state::AppState,
};

pub async fn list_collections(
    State(state): State<AppState>,
    auth_user: AuthUser,
) -> AppResult<Json<Vec<AlbumCollectionResponse>>> {
    let rows = AlbumCollectionRepository::list_by_user(&state.db, auth_user.user.id).await?;
    Ok(Json(rows.into_iter().map(Into::into).collect()))
}

pub async fn create_collection(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Garde(Json(body)): Garde<Json<CreateAlbumCollectionParams>>,
) -> AppResult<(StatusCode, Json<AlbumCollectionResponse>)> {
    let row = AlbumCollectionRepository::create(
        &state.db,
        auth_user.user.id,
        &body.name,
        &body.filter_definition,
    )
    .await?;
    Ok((StatusCode::CREATED, Json(row.into())))
}

pub async fn get_collection(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(id): Path<Uuid>,
) -> AppResult<Json<AlbumCollectionResponse>> {
    let row = AlbumCollectionRepository::find_by_id_and_user(&state.db, id, auth_user.user.id)
        .await?
        .ok_or(AppError::NotFound)?;
    Ok(Json(row.into()))
}

pub async fn rename_collection(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(id): Path<Uuid>,
    Garde(Json(body)): Garde<Json<RenameAlbumCollectionParams>>,
) -> AppResult<Json<AlbumCollectionResponse>> {
    let row = AlbumCollectionRepository::rename(&state.db, id, auth_user.user.id, &body.name)
        .await?
        .ok_or(AppError::NotFound)?;
    Ok(Json(row.into()))
}

pub async fn update_collection_filter(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(id): Path<Uuid>,
    Garde(Json(body)): Garde<Json<UpdateAlbumCollectionFilterParams>>,
) -> AppResult<Json<AlbumCollectionResponse>> {
    let row = AlbumCollectionRepository::update_filter(
        &state.db,
        id,
        auth_user.user.id,
        &body.filter_definition,
    )
    .await?
    .ok_or(AppError::NotFound)?;
    Ok(Json(row.into()))
}

pub async fn delete_collection(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(id): Path<Uuid>,
) -> AppResult<StatusCode> {
    let deleted = AlbumCollectionRepository::delete(&state.db, id, auth_user.user.id).await?;
    if deleted {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(AppError::NotFound)
    }
}

/// Resolve the collection's filter to a paginated list of albums.
pub async fn list_collection_albums(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(id): Path<Uuid>,
    Query(params): Query<FilteredPaginationParams>,
) -> AppResult<Json<PaginatedResponse<AlbumResponse>>> {
    let collection =
        AlbumCollectionRepository::find_by_id_and_user(&state.db, id, auth_user.user.id)
            .await?
            .ok_or(AppError::NotFound)?;

    let sort = params
        .sort
        .as_deref()
        .map(AlbumSortKey::from_str)
        .transpose()?;
    let service = LibraryService::new(state.db.clone());
    let (albums, total) = service
        .get_albums(
            Some(&collection.filter_definition.0),
            params.limit,
            params.offset,
            sort,
            params.dir,
        )
        .await?;

    Ok(Json(PaginatedResponse {
        data: albums.into_iter().map(Into::into).collect(),
        total,
        limit: params.limit,
        offset: params.offset,
    }))
}
