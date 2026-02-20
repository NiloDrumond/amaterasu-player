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
            INSERT INTO artists (id, name, sort_name, mbid, created_at, updated_at)
                VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING
                *
            "#,
            artist.id,
            artist.name,
            artist.sort_name,
            artist.mbid,
            artist.created_at,
            artist.updated_at
        )
        .fetch_one(executor)
        .await?;

        Ok(created)
    }

    pub async fn get_by_id(
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

    pub async fn get_by_name(
        executor: impl PgExecutor<'_>,
        name: &str,
    ) -> Result<Option<Artist>, AppError> {
        let artist = sqlx::query_as!(
            Artist,
            r#"
            SELECT
                *
            FROM
                artists
            WHERE
                LOWER(name) = LOWER($1)
            "#,
            name
        )
        .fetch_optional(executor)
        .await?;

        Ok(artist)
    }

    pub async fn update(
        executor: impl PgExecutor<'_>,
        artist: &Artist,
    ) -> Result<Artist, AppError> {
        let updated = sqlx::query_as!(
            Artist,
            r#"
            UPDATE
                artists
            SET
                name = $2,
                sort_name = $3,
                mbid = $4
            WHERE
                id = $1
            RETURNING
                *
            "#,
            artist.id,
            artist.name,
            artist.sort_name,
            artist.mbid
        )
        .fetch_one(executor)
        .await?;

        Ok(updated)
    }

    pub async fn delete(executor: impl PgExecutor<'_>, id: Uuid) -> Result<bool, AppError> {
        let result = sqlx::query!(
            r#"
            DELETE FROM artists
            WHERE id = $1
            "#,
            id
        )
        .execute(executor)
        .await?;

        Ok(result.rows_affected() > 0)
    }

    pub async fn list(
        executor: impl PgExecutor<'_>,
        offset: i64,
        limit: i64,
    ) -> Result<Vec<Artist>, AppError> {
        let artists = sqlx::query_as!(
            Artist,
            r#"
            SELECT
                *
            FROM
                artists
            ORDER BY
                name OFFSET $1
            LIMIT $2
            "#,
            offset,
            limit
        )
        .fetch_all(executor)
        .await?;

        Ok(artists)
    }

    pub async fn search(
        executor: impl PgExecutor<'_>,
        query: &str,
    ) -> Result<Vec<Artist>, AppError> {
        let pattern = format!("%{}%", query);
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
                name
            LIMIT 50
            "#,
            pattern
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
}
