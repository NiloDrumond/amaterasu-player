use sqlx::PgPool;
use uuid::Uuid;

use crate::db::entities::Artist;
use crate::error::AppError;

pub struct ArtistRepository {
    pool: PgPool,
}

impl ArtistRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Create a new artist
    pub async fn create(&self, artist: &Artist) -> Result<Artist, AppError> {
        let created = sqlx::query_as!(
            Artist,
            r#"
            INSERT INTO artists (id, name, mbid, created_at, updated_at)
                VALUES ($1, $2, $3, $4, $5)
            RETURNING
                *
            "#,
            artist.id,
            artist.name,
            artist.mbid,
            artist.created_at,
            artist.updated_at
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(created)
    }

    /// Get an artist by ID
    pub async fn get_by_id(&self, id: Uuid) -> Result<Option<Artist>, AppError> {
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
        .fetch_optional(&self.pool)
        .await?;

        Ok(artist)
    }

    /// Get an artist by name (case-insensitive)
    pub async fn get_by_name(&self, name: &str) -> Result<Option<Artist>, AppError> {
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
        .fetch_optional(&self.pool)
        .await?;

        Ok(artist)
    }

    pub async fn find_or_create(&self, name: String) -> Result<Artist, AppError> {
        if let Some(artist) = self.get_by_name(&name).await? {
            return Ok(artist);
        }

        let new_artist = Artist {
            name,
            ..Default::default()
        };
        self.create(&new_artist).await
    }

    /// Update an artist
    pub async fn update(&self, artist: &Artist) -> Result<Artist, AppError> {
        let updated = sqlx::query_as!(
            Artist,
            r#"
            UPDATE
                artists
            SET
                name = $2,
                mbid = $3
            WHERE
                id = $1
            RETURNING
                *
            "#,
            artist.id,
            artist.name,
            artist.mbid
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(updated)
    }

    /// Delete an artist
    pub async fn delete(&self, id: Uuid) -> Result<bool, AppError> {
        let result = sqlx::query!(
            r#"
            DELETE FROM artists
            WHERE id = $1
            "#,
            id
        )
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }

    /// List all artists with pagination
    pub async fn list(&self, offset: i64, limit: i64) -> Result<Vec<Artist>, AppError> {
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
        .fetch_all(&self.pool)
        .await?;

        Ok(artists)
    }

    /// Search artists by name
    pub async fn search(&self, query: &str) -> Result<Vec<Artist>, AppError> {
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
        .fetch_all(&self.pool)
        .await?;

        Ok(artists)
    }

    /// Count total artists
    pub async fn count(&self) -> Result<i64, AppError> {
        let record = sqlx::query!(
            r#"
            SELECT
                COUNT(*) AS count
            FROM
                artists
            "#
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(record.count.unwrap_or(0))
    }
}
