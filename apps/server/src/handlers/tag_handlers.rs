use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use axum_valid::Garde;
use uuid::Uuid;

use crate::{
    auth::AuthUser,
    dto::{
        request::{CreateTagParams, UpdateTagParams},
        response::{TagResponse, TagSummaryResponse},
    },
    error::{AppError, AppResult},
    repositories::TagRepository,
    state::AppState,
};

pub async fn list_tags(
    State(state): State<AppState>,
    auth_user: AuthUser,
) -> AppResult<Json<Vec<TagResponse>>> {
    let tags = TagRepository::list_by_user(&state.db, auth_user.user.id).await?;

    Ok(Json(tags.into_iter().map(Into::into).collect()))
}

pub async fn create_tag(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Garde(Json(body)): Garde<Json<CreateTagParams>>,
) -> AppResult<(StatusCode, Json<TagResponse>)> {
    let tag = TagRepository::create(
        &state.db,
        auth_user.user.id,
        &body.name,
        body.category_id,
        body.color.as_deref(),
    )
    .await?;

    let with_usage = TagRepository::find_by_id_and_user(&state.db, tag.id, auth_user.user.id)
        .await?
        .ok_or(AppError::NotFound)?;

    Ok((StatusCode::CREATED, Json(with_usage.into())))
}

pub async fn update_tag(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(id): Path<Uuid>,
    Garde(Json(body)): Garde<Json<UpdateTagParams>>,
) -> AppResult<Json<TagResponse>> {
    TagRepository::update(
        &state.db,
        id,
        auth_user.user.id,
        body.name.as_deref(),
        body.category_id,
        body.clear_category,
        body.color.as_deref(),
    )
    .await?
    .ok_or(AppError::NotFound)?;

    let with_usage = TagRepository::find_by_id_and_user(&state.db, id, auth_user.user.id)
        .await?
        .ok_or(AppError::NotFound)?;

    Ok(Json(with_usage.into()))
}

pub async fn delete_tag(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(id): Path<Uuid>,
) -> AppResult<StatusCode> {
    let deleted = TagRepository::delete(&state.db, id, auth_user.user.id).await?;

    if deleted {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(AppError::NotFound)
    }
}

pub async fn list_track_tags(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(track_id): Path<Uuid>,
) -> AppResult<Json<Vec<TagSummaryResponse>>> {
    let tags = TagRepository::list_track_tags(&state.db, track_id, auth_user.user.id).await?;

    Ok(Json(tags.into_iter().map(Into::into).collect()))
}

pub async fn attach_track_tag(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path((track_id, tag_id)): Path<(Uuid, Uuid)>,
) -> AppResult<StatusCode> {
    let owned =
        TagRepository::attach_to_track(&state.db, tag_id, track_id, auth_user.user.id).await?;

    if owned {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(AppError::NotFound)
    }
}

pub async fn detach_track_tag(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path((track_id, tag_id)): Path<(Uuid, Uuid)>,
) -> AppResult<StatusCode> {
    let removed =
        TagRepository::detach_from_track(&state.db, tag_id, track_id, auth_user.user.id).await?;

    if removed {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(AppError::NotFound)
    }
}

pub async fn list_album_tags(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(album_id): Path<Uuid>,
) -> AppResult<Json<Vec<TagSummaryResponse>>> {
    let tags = TagRepository::list_album_tags(&state.db, album_id, auth_user.user.id).await?;

    Ok(Json(tags.into_iter().map(Into::into).collect()))
}

pub async fn attach_album_tag(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path((album_id, tag_id)): Path<(Uuid, Uuid)>,
) -> AppResult<StatusCode> {
    let owned =
        TagRepository::attach_to_album(&state.db, tag_id, album_id, auth_user.user.id).await?;

    if owned {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(AppError::NotFound)
    }
}

pub async fn detach_album_tag(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path((album_id, tag_id)): Path<(Uuid, Uuid)>,
) -> AppResult<StatusCode> {
    let removed =
        TagRepository::detach_from_album(&state.db, tag_id, album_id, auth_user.user.id).await?;

    if removed {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(AppError::NotFound)
    }
}
