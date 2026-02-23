use sqlx::PgExecutor;

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
}
