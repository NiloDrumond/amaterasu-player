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
