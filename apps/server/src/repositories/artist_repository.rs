use sqlx::PgExecutor;
use uuid::Uuid;

use crate::db::entities::Artist;
use crate::error::AppError;

pub struct ArtistRepository;

impl ArtistRepository {
    pub async fn create(
        executor: impl PgExecutor<'_>,
        artist: &Artist,
    ) -> Result<Artist, AppError> {
        let created = sqlx::query_as!(
            Artist,
            r#"
            INSERT INTO artists (id, name, sort_name, mbid, source_name, locked_at, created_at, updated_at)
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            RETURNING
                *
            "#,
            artist.id,
            artist.name,
            artist.sort_name,
            artist.mbid,
            artist.source_name,
            artist.locked_at,
            artist.created_at,
            artist.updated_at
        )
        .fetch_one(executor)
        .await?;

        Ok(created)
    }

    pub async fn find_by_id(
        executor: impl PgExecutor<'_>,
        id: Uuid,
    ) -> Result<Option<Artist>, AppError> {
        let artist = sqlx::query_as!(
            Artist,
            r#"
            SELECT
                *
            FROM
                artists
            WHERE
                id = $1
            "#,
            id
        )
        .fetch_optional(executor)
        .await?;

        Ok(artist)
    }

    pub async fn find_by_ids(
        executor: impl PgExecutor<'_>,
        ids: &[Uuid],
    ) -> Result<Vec<Artist>, AppError> {
        if ids.is_empty() {
            return Ok(Vec::new());
        }

        let artists = sqlx::query_as!(
            Artist,
            r#"
            SELECT
                *
            FROM
                artists
            WHERE
                id = ANY($1)
            "#,
            ids
        )
        .fetch_all(executor)
        .await?;

        Ok(artists)
    }

    pub async fn find_by_source_name(
        executor: impl PgExecutor<'_>,
        source_name: &str,
    ) -> Result<Option<Artist>, AppError> {
        let artist = sqlx::query_as!(
            Artist,
            r#"
            SELECT
                *
            FROM
                artists
            WHERE
                LOWER(source_name) = LOWER($1)
            "#,
            source_name
        )
        .fetch_optional(executor)
        .await?;

        Ok(artist)
    }

    pub async fn find_all(
        executor: impl PgExecutor<'_>,
        limit: i32,
        offset: i32,
    ) -> Result<Vec<Artist>, AppError> {
        let artists = sqlx::query_as!(
            Artist,
            r#"
            SELECT
                *
            FROM
                artists
            ORDER BY
                sort_name,
                name
            LIMIT $1 OFFSET $2
            "#,
            limit as i64,
            offset as i64,
        )
        .fetch_all(executor)
        .await?;

        Ok(artists)
    }

    pub async fn search(
        executor: impl PgExecutor<'_>,
        query: &str,
        limit: i32,
    ) -> Result<Vec<Artist>, AppError> {
        let pattern = format!("%{}%", query.replace('%', "\\%").replace('_', "\\_"));
        let artists = sqlx::query_as!(
            Artist,
            r#"
            SELECT
                *
            FROM
                artists
            WHERE
                name ILIKE $1
            ORDER BY
                sort_name,
                name
            LIMIT $2
            "#,
            pattern,
            limit as i64,
        )
        .fetch_all(executor)
        .await?;

        Ok(artists)
    }

    pub async fn count(executor: impl PgExecutor<'_>) -> Result<i64, AppError> {
        let record = sqlx::query!(
            r#"
            SELECT
                COUNT(*) AS count
            FROM
                artists
            "#
        )
        .fetch_one(executor)
        .await?;

        Ok(record.count.unwrap_or(0))
    }

    pub async fn update(
        executor: impl PgExecutor<'_>,
        id: Uuid,
        name: &str,
        sort_name: &str,
    ) -> Result<Artist, AppError> {
        let updated = sqlx::query_as!(
            Artist,
            r#"
            UPDATE artists
            SET
                name = $2,
                sort_name = $3,
                locked_at = NOW(),
                updated_at = NOW()
            WHERE id = $1
            RETURNING *
            "#,
            id,
            name,
            sort_name,
        )
        .fetch_one(executor)
        .await?;

        Ok(updated)
    }

    pub async fn clear_lock(executor: impl PgExecutor<'_>, id: Uuid) -> Result<(), AppError> {
        sqlx::query!(r#"UPDATE artists SET locked_at = NULL WHERE id = $1"#, id)
            .execute(executor)
            .await?;
        Ok(())
    }

    /// Hard-deletes the artist only if no album references it (as `artist_id` or
    /// `source_album_artist_id`) and no track references it. Returns `true` if
    /// deleted, `false` if a reference still exists.
    pub async fn delete_if_empty(
        executor: impl PgExecutor<'_>,
        id: Uuid,
    ) -> Result<bool, AppError> {
        let result = sqlx::query!(
            r#"
            DELETE FROM artists
            WHERE id = $1
              AND NOT EXISTS (
                  SELECT 1 FROM albums
                  WHERE artist_id = $1 OR source_album_artist_id = $1
              )
              AND NOT EXISTS (
                  SELECT 1 FROM tracks
                  WHERE artist_id = $1 AND deleted_at IS NULL
              )
            "#,
            id
        )
        .execute(executor)
        .await?;
        Ok(result.rows_affected() > 0)
    }
}
