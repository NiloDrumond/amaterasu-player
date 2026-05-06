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
        request::{CreateTagCategoryParams, ReorderTagCategoriesParams, UpdateTagCategoryParams},
        response::TagCategoryResponse,
    },
    error::{AppError, AppResult},
    repositories::TagCategoryRepository,
    state::AppState,
};

pub async fn list_tag_categories(
    State(state): State<AppState>,
    auth_user: AuthUser,
) -> AppResult<Json<Vec<TagCategoryResponse>>> {
    let categories = TagCategoryRepository::list_by_user(&state.db, auth_user.user.id).await?;

    Ok(Json(categories.into_iter().map(Into::into).collect()))
}

pub async fn create_tag_category(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Garde(Json(body)): Garde<Json<CreateTagCategoryParams>>,
) -> AppResult<(StatusCode, Json<TagCategoryResponse>)> {
    let category = TagCategoryRepository::create(
        &state.db,
        auth_user.user.id,
        &body.name,
        body.color.as_deref(),
    )
    .await?;

    let with_count =
        TagCategoryRepository::find_by_id_and_user(&state.db, category.id, auth_user.user.id)
            .await?
            .ok_or(AppError::NotFound)?;

    Ok((StatusCode::CREATED, Json(with_count.into())))
}

pub async fn update_tag_category(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(id): Path<Uuid>,
    Garde(Json(body)): Garde<Json<UpdateTagCategoryParams>>,
) -> AppResult<Json<TagCategoryResponse>> {
    TagCategoryRepository::update(
        &state.db,
        id,
        auth_user.user.id,
        body.name.as_deref(),
        body.color.as_deref(),
    )
    .await?
    .ok_or(AppError::NotFound)?;

    let with_count = TagCategoryRepository::find_by_id_and_user(&state.db, id, auth_user.user.id)
        .await?
        .ok_or(AppError::NotFound)?;

    Ok(Json(with_count.into()))
}

pub async fn delete_tag_category(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(id): Path<Uuid>,
) -> AppResult<StatusCode> {
    let deleted = TagCategoryRepository::delete(&state.db, id, auth_user.user.id).await?;

    if deleted {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(AppError::NotFound)
    }
}

pub async fn reorder_tag_categories(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Garde(Json(body)): Garde<Json<ReorderTagCategoriesParams>>,
) -> AppResult<StatusCode> {
    TagCategoryRepository::reorder(&state.db, auth_user.user.id, &body.ordered_ids).await?;

    Ok(StatusCode::NO_CONTENT)
}
