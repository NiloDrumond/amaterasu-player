use sqlx::PgPool;
use uuid::Uuid;

use crate::db::entities::Album;
use crate::error::AppError;

pub struct AlbumRepository {
    pool: PgPool,
}

impl AlbumRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Create a new album
    pub async fn create(&self, album: &Album) -> Result<Album, AppError> {
        let created = sqlx::query_as!(
            Album,
            r#"
            INSERT INTO albums (id, artist_id, title, year, mbid, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING *
            "#,
            album.id,
            album.artist_id,
            album.title,
            album.year,
            album.mbid,
            album.created_at,
            album.updated_at
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(created)
    }

    /// Get an album by ID
    pub async fn get_by_id(&self, id: Uuid) -> Result<Option<Album>, AppError> {
        let album = sqlx::query_as!(
            Album,
            r#"
            SELECT * FROM albums WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(album)
    }

    /// Get albums by artist
    pub async fn get_by_artist(&self, artist_id: Uuid) -> Result<Vec<Album>, AppError> {
        let albums = sqlx::query_as!(
            Album,
            r#"
            SELECT * FROM albums 
            WHERE artist_id = $1
            ORDER BY year DESC NULLS LAST, title
            "#,
            artist_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(albums)
    }

    /// Find album by title and artist (case-insensitive)
    pub async fn find_by_title_and_artist(
        &self,
        title: &str,
        artist_id: Option<Uuid>,
    ) -> Result<Option<Album>, AppError> {
        let album = sqlx::query_as!(
            Album,
            r#"
            SELECT * FROM albums 
            WHERE LOWER(title) = LOWER($1) 
            AND (artist_id = $2 OR (artist_id IS NULL AND $2 IS NULL))
            "#,
            title,
            artist_id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(album)
    }

    /// Find or create an album
    pub async fn find_or_create(
        &self,
        title: String,
        artist_id: Option<Uuid>,
        year: Option<i32>,
    ) -> Result<Album, AppError> {
        // First try to find existing
        if let Some(album) = self.find_by_title_and_artist(&title, artist_id).await? {
            return Ok(album);
        }

        // Create new if not found
        let new_album = if let Some(year) = year {
            Album::with_year(title, artist_id, year)
        } else {
            Album::new(title, artist_id)
        };
        
        self.create(&new_album).await
    }

    /// Update an album
    pub async fn update(&self, album: &Album) -> Result<Album, AppError> {
        let updated = sqlx::query_as!(
            Album,
            r#"
            UPDATE albums
            SET artist_id = $2, title = $3, year = $4, mbid = $5
            WHERE id = $1
            RETURNING *
            "#,
            album.id,
            album.artist_id,
            album.title,
            album.year,
            album.mbid
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(updated)
    }

    /// Delete an album
    pub async fn delete(&self, id: Uuid) -> Result<bool, AppError> {
        let result = sqlx::query!(
            r#"
            DELETE FROM albums WHERE id = $1
            "#,
            id
        )
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }

    /// List all albums with pagination
    pub async fn list(&self, offset: i64, limit: i64) -> Result<Vec<Album>, AppError> {
        let albums = sqlx::query_as!(
            Album,
            r#"
            SELECT * FROM albums
            ORDER BY title
            OFFSET $1 LIMIT $2
            "#,
            offset,
            limit
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(albums)
    }

    /// Get compilation albums (no artist)
    pub async fn get_compilations(&self) -> Result<Vec<Album>, AppError> {
        let albums = sqlx::query_as!(
            Album,
            r#"
            SELECT * FROM albums
            WHERE artist_id IS NULL
            ORDER BY title
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(albums)
    }

    /// Search albums by title
    pub async fn search(&self, query: &str) -> Result<Vec<Album>, AppError> {
        let pattern = format!("%{}%", query);
        let albums = sqlx::query_as!(
            Album,
            r#"
            SELECT * FROM albums
            WHERE title ILIKE $1
            ORDER BY title
            LIMIT 50
            "#,
            pattern
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(albums)
    }

    /// Count total albums
    pub async fn count(&self) -> Result<i64, AppError> {
        let record = sqlx::query!(
            r#"
            SELECT COUNT(*) as count FROM albums
            "#
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(record.count.unwrap_or(0))
    }

    /// Get albums by year
    pub async fn get_by_year(&self, year: i32) -> Result<Vec<Album>, AppError> {
        let albums = sqlx::query_as!(
            Album,
            r#"
            SELECT * FROM albums
            WHERE year = $1
            ORDER BY title
            "#,
            year
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(albums)
    }
}