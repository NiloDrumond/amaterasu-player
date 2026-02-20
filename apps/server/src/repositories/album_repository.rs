use chrono::NaiveDate;
use sqlx::PgExecutor;
use uuid::Uuid;

use crate::db::entities::Album;
use crate::error::AppError;

pub struct AlbumRepository;

impl AlbumRepository {
    pub async fn create(
        executor: impl PgExecutor<'_>,
        album: &Album,
    ) -> Result<Album, AppError> {
        let created = sqlx::query_as!(
            Album,
            r#"
            INSERT INTO albums (id, artist_id, title, sort_title, date, mbid, replaygain_album_gain, replaygain_album_peak, created_at, updated_at)
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            RETURNING
                *
            "#,
            album.id,
            album.artist_id,
            album.title,
            album.sort_title,
            album.date,
            album.mbid,
            album.replaygain_album_gain,
            album.replaygain_album_peak,
            album.created_at,
            album.updated_at
        )
        .fetch_one(executor)
        .await?;

        Ok(created)
    }

    pub async fn get_by_id(
        executor: impl PgExecutor<'_>,
        id: Uuid,
    ) -> Result<Option<Album>, AppError> {
        let album = sqlx::query_as!(
            Album,
            r#"
            SELECT
                *
            FROM
                albums
            WHERE
                id = $1
            "#,
            id
        )
        .fetch_optional(executor)
        .await?;

        Ok(album)
    }

    pub async fn get_by_artist(
        executor: impl PgExecutor<'_>,
        artist_id: Uuid,
    ) -> Result<Vec<Album>, AppError> {
        let albums = sqlx::query_as!(
            Album,
            r#"
            SELECT
                *
            FROM
                albums
            WHERE
                artist_id = $1
            ORDER BY
                date DESC NULLS LAST,
                title
            "#,
            artist_id
        )
        .fetch_all(executor)
        .await?;

        Ok(albums)
    }

    pub async fn find_by_title_and_artist(
        executor: impl PgExecutor<'_>,
        title: &str,
        artist_id: Option<Uuid>,
    ) -> Result<Option<Album>, AppError> {
        let album = sqlx::query_as!(
            Album,
            r#"
            SELECT
                *
            FROM
                albums
            WHERE
                LOWER(title) = LOWER($1)
                AND (artist_id = $2
                    OR (artist_id IS NULL
                        AND $2 IS NULL))
            "#,
            title,
            artist_id
        )
        .fetch_optional(executor)
        .await?;

        Ok(album)
    }

    pub async fn update(
        executor: impl PgExecutor<'_>,
        album: &Album,
    ) -> Result<Album, AppError> {
        let updated = sqlx::query_as!(
            Album,
            r#"
            UPDATE
                albums
            SET
                artist_id = $2,
                title = $3,
                sort_title = $4,
                date = $5,
                mbid = $6,
                replaygain_album_gain = $7,
                replaygain_album_peak = $8
            WHERE
                id = $1
            RETURNING
                *
            "#,
            album.id,
            album.artist_id,
            album.title,
            album.sort_title,
            album.date,
            album.mbid,
            album.replaygain_album_gain,
            album.replaygain_album_peak
        )
        .fetch_one(executor)
        .await?;

        Ok(updated)
    }

    pub async fn delete(executor: impl PgExecutor<'_>, id: Uuid) -> Result<bool, AppError> {
        let result = sqlx::query!(
            r#"
            DELETE FROM albums
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
    ) -> Result<Vec<Album>, AppError> {
        let albums = sqlx::query_as!(
            Album,
            r#"
            SELECT
                *
            FROM
                albums
            ORDER BY
                title OFFSET $1
            LIMIT $2
            "#,
            offset,
            limit
        )
        .fetch_all(executor)
        .await?;

        Ok(albums)
    }

    pub async fn get_compilations(
        executor: impl PgExecutor<'_>,
    ) -> Result<Vec<Album>, AppError> {
        let albums = sqlx::query_as!(
            Album,
            r#"
            SELECT
                *
            FROM
                albums
            WHERE
                artist_id IS NULL
            ORDER BY
                title
            "#
        )
        .fetch_all(executor)
        .await?;

        Ok(albums)
    }

    pub async fn search(
        executor: impl PgExecutor<'_>,
        query: &str,
    ) -> Result<Vec<Album>, AppError> {
        let pattern = format!("%{}%", query);
        let albums = sqlx::query_as!(
            Album,
            r#"
            SELECT
                *
            FROM
                albums
            WHERE
                title ILIKE $1
            ORDER BY
                title
            LIMIT 50
            "#,
            pattern
        )
        .fetch_all(executor)
        .await?;

        Ok(albums)
    }

    pub async fn count(executor: impl PgExecutor<'_>) -> Result<i64, AppError> {
        let record = sqlx::query!(
            r#"
            SELECT
                COUNT(*) AS count
            FROM
                albums
            "#
        )
        .fetch_one(executor)
        .await?;

        Ok(record.count.unwrap_or(0))
    }

    pub async fn get_by_date(
        executor: impl PgExecutor<'_>,
        date: NaiveDate,
    ) -> Result<Vec<Album>, AppError> {
        let albums = sqlx::query_as!(
            Album,
            r#"
            SELECT
                *
            FROM
                albums
            WHERE
                date = $1
            ORDER BY
                title
            "#,
            date
        )
        .fetch_all(executor)
        .await?;

        Ok(albums)
    }
}
