use sqlx::PgExecutor;
use uuid::Uuid;

use crate::db::entities::Album;
use crate::error::{AppError, AppResult};

pub struct AlbumRepository;

impl AlbumRepository {
    pub async fn create(executor: impl PgExecutor<'_>, album: &Album) -> Result<Album, AppError> {
        let created = sqlx::query_as!(
            Album,
            r#"
            INSERT INTO albums (id, artist_id, title, sort_title, date, mbid, cover_path, replaygain_album_gain, replaygain_album_peak, created_at, updated_at)
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
            RETURNING
                *
            "#,
            album.id,
            album.artist_id,
            album.title,
            album.sort_title,
            album.date,
            album.mbid,
            album.cover_path,
            album.replaygain_album_gain,
            album.replaygain_album_peak,
            album.created_at,
            album.updated_at
        )
        .fetch_one(executor)
        .await?;

        Ok(created)
    }

    pub async fn find_by_id(executor: impl PgExecutor<'_>, id: Uuid) -> AppResult<Album> {
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
        .fetch_one(executor)
        .await?;

        Ok(album)
    }

    pub async fn find_by_ids(
        executor: impl PgExecutor<'_>,
        ids: &[Uuid],
    ) -> Result<Vec<Album>, AppError> {
        if ids.is_empty() {
            return Ok(Vec::new());
        }

        let albums = sqlx::query_as!(
            Album,
            r#"
            SELECT
                *
            FROM
                albums
            WHERE
                id = ANY ($1)
            "#,
            ids
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

    pub async fn find_all(
        executor: impl PgExecutor<'_>,
        limit: i32,
        offset: i32,
    ) -> AppResult<Vec<Album>> {
        let albums = sqlx::query_as!(
            Album,
            r#"
            SELECT
                *
            FROM
                albums
            ORDER BY
                sort_title,
                title
            LIMIT $1 OFFSET $2
            "#,
            limit as i64,
            offset as i64,
        )
        .fetch_all(executor)
        .await?;

        Ok(albums)
    }

    pub async fn count(executor: impl PgExecutor<'_>) -> AppResult<i64> {
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

    pub async fn get_track_stats_for_album_ids(
        executor: impl PgExecutor<'_>,
        ids: &[Uuid],
    ) -> AppResult<Vec<(Uuid, i64, i64)>> {
        if ids.is_empty() {
            return Ok(Vec::new());
        }

        let rows = sqlx::query!(
            r#"
            SELECT
                album_id AS "album_id!",
                COUNT(*) AS "track_count!",
                COALESCE(SUM(duration_ms), 0)::bigint AS "total_duration_ms!"
            FROM
                tracks
            WHERE
                album_id = ANY ($1)
            GROUP BY
                album_id
            "#,
            ids
        )
        .fetch_all(executor)
        .await?;

        Ok(rows
            .into_iter()
            .map(|r| (r.album_id, r.track_count, r.total_duration_ms))
            .collect())
    }

    pub async fn find_by_artist_id(
        executor: impl PgExecutor<'_>,
        artist_id: Uuid,
    ) -> AppResult<Vec<Album>> {
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
                date DESC,
                sort_title,
                title
            "#,
            artist_id,
        )
        .fetch_all(executor)
        .await?;

        Ok(albums)
    }

    pub async fn get_album_count_for_artist_ids(
        executor: impl PgExecutor<'_>,
        ids: &[Uuid],
    ) -> AppResult<Vec<(Uuid, i64)>> {
        if ids.is_empty() {
            return Ok(Vec::new());
        }

        let rows = sqlx::query!(
            r#"
            SELECT
                artist_id AS "artist_id!",
                COUNT(*) AS "album_count!"
            FROM
                albums
            WHERE
                artist_id = ANY ($1)
            GROUP BY
                artist_id
            "#,
            ids
        )
        .fetch_all(executor)
        .await?;

        Ok(rows
            .into_iter()
            .map(|r| (r.artist_id, r.album_count))
            .collect())
    }

    pub async fn update_cover_path(
        executor: impl PgExecutor<'_>,
        id: Uuid,
        cover_path: &str,
    ) -> Result<(), AppError> {
        sqlx::query!(
            r#"
            UPDATE
                albums
            SET
                cover_path = $2
            WHERE
                id = $1
                AND cover_path IS NULL
            "#,
            id,
            cover_path
        )
        .execute(executor)
        .await?;

        Ok(())
    }
}
