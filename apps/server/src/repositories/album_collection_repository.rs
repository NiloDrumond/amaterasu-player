use sqlx::types::Json;
use sqlx::PgExecutor;
use uuid::Uuid;

use crate::db::entities::AlbumCollection;
use crate::error::AppResult;
use crate::filters::FilterNode;

pub struct AlbumCollectionRepository;

impl AlbumCollectionRepository {
    pub async fn create(
        executor: impl PgExecutor<'_>,
        user_id: Uuid,
        name: &str,
        filter_definition: &FilterNode,
    ) -> AppResult<AlbumCollection> {
        let filter_json = serde_json::to_value(filter_definition)
            .map_err(|e| crate::error::AppError::BadRequest(format!("invalid filter: {e}")))?;
        let row = sqlx::query_as!(
            AlbumCollection,
            r#"
            INSERT INTO album_collections (user_id, name, filter_definition)
                VALUES ($1, $2, $3)
            RETURNING
                id,
                user_id,
                name,
                filter_definition AS "filter_definition: Json<FilterNode>",
                created_at,
                updated_at
            "#,
            user_id,
            name,
            filter_json,
        )
        .fetch_one(executor)
        .await?;
        Ok(row)
    }

    pub async fn list_by_user(
        executor: impl PgExecutor<'_>,
        user_id: Uuid,
    ) -> AppResult<Vec<AlbumCollection>> {
        let rows = sqlx::query_as!(
            AlbumCollection,
            r#"
            SELECT
                id,
                user_id,
                name,
                filter_definition AS "filter_definition: Json<FilterNode>",
                created_at,
                updated_at
            FROM album_collections
            WHERE user_id = $1
            ORDER BY created_at DESC
            "#,
            user_id,
        )
        .fetch_all(executor)
        .await?;
        Ok(rows)
    }

    pub async fn find_by_id_and_user(
        executor: impl PgExecutor<'_>,
        id: Uuid,
        user_id: Uuid,
    ) -> AppResult<Option<AlbumCollection>> {
        let row = sqlx::query_as!(
            AlbumCollection,
            r#"
            SELECT
                id,
                user_id,
                name,
                filter_definition AS "filter_definition: Json<FilterNode>",
                created_at,
                updated_at
            FROM album_collections
            WHERE id = $1 AND user_id = $2
            "#,
            id,
            user_id,
        )
        .fetch_optional(executor)
        .await?;
        Ok(row)
    }

    pub async fn update_filter(
        executor: impl PgExecutor<'_>,
        id: Uuid,
        user_id: Uuid,
        filter_definition: &FilterNode,
    ) -> AppResult<Option<AlbumCollection>> {
        let filter_json = serde_json::to_value(filter_definition)
            .map_err(|e| crate::error::AppError::BadRequest(format!("invalid filter: {e}")))?;
        let row = sqlx::query_as!(
            AlbumCollection,
            r#"
            UPDATE album_collections
            SET filter_definition = $3, updated_at = NOW()
            WHERE id = $1 AND user_id = $2
            RETURNING
                id,
                user_id,
                name,
                filter_definition AS "filter_definition: Json<FilterNode>",
                created_at,
                updated_at
            "#,
            id,
            user_id,
            filter_json,
        )
        .fetch_optional(executor)
        .await?;
        Ok(row)
    }

    pub async fn rename(
        executor: impl PgExecutor<'_>,
        id: Uuid,
        user_id: Uuid,
        name: &str,
    ) -> AppResult<Option<AlbumCollection>> {
        let row = sqlx::query_as!(
            AlbumCollection,
            r#"
            UPDATE album_collections
            SET name = $3, updated_at = NOW()
            WHERE id = $1 AND user_id = $2
            RETURNING
                id,
                user_id,
                name,
                filter_definition AS "filter_definition: Json<FilterNode>",
                created_at,
                updated_at
            "#,
            id,
            user_id,
            name,
        )
        .fetch_optional(executor)
        .await?;
        Ok(row)
    }

    pub async fn delete(executor: impl PgExecutor<'_>, id: Uuid, user_id: Uuid) -> AppResult<bool> {
        let res = sqlx::query!(
            r#"DELETE FROM album_collections WHERE id = $1 AND user_id = $2"#,
            id,
            user_id,
        )
        .execute(executor)
        .await?;
        Ok(res.rows_affected() > 0)
    }
}
