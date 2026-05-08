use chrono::NaiveDate;
use sqlx::{PgExecutor, PgPool, Postgres, QueryBuilder};
use uuid::Uuid;

use crate::db::entities::Album;
use crate::error::{AppError, AppResult};
use crate::filters::{compile_albums_filter, FilterNode};

pub struct AlbumRepository;

impl AlbumRepository {
    pub async fn create(executor: impl PgExecutor<'_>, album: &Album) -> Result<Album, AppError> {
        let created = sqlx::query_as!(
            Album,
            r#"
            INSERT INTO albums (id, artist_id, title, sort_title, date, mbid, cover_path, source_title, source_album_artist_id, locked_at, replaygain_album_gain, replaygain_album_peak, created_at, updated_at)
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14)
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
            album.source_title,
            album.source_album_artist_id,
            album.locked_at,
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

    pub async fn find_by_source_keys(
        executor: impl PgExecutor<'_>,
        source_title: &str,
        source_album_artist_id: Option<Uuid>,
    ) -> Result<Option<Album>, AppError> {
        let album = sqlx::query_as!(
            Album,
            r#"
            SELECT
                *
            FROM
                albums
            WHERE
                LOWER(source_title) = LOWER($1)
                AND (source_album_artist_id = $2
                    OR (source_album_artist_id IS NULL
                        AND $2 IS NULL))
            "#,
            source_title,
            source_album_artist_id
        )
        .fetch_optional(executor)
        .await?;

        Ok(album)
    }

    pub async fn search(
        executor: impl PgExecutor<'_>,
        query: &str,
        artist_id: Option<Uuid>,
        limit: i32,
    ) -> AppResult<Vec<Album>> {
        let pattern = format!("%{}%", query.replace('%', "\\%").replace('_', "\\_"));
        let albums = sqlx::query_as!(
            Album,
            r#"
            SELECT
                *
            FROM
                albums
            WHERE
                title ILIKE $1
                AND ($2::uuid IS NULL
                    OR artist_id = $2)
            ORDER BY
                sort_title,
                title
            LIMIT $3
            "#,
            pattern,
            artist_id,
            limit as i64,
        )
        .fetch_all(executor)
        .await?;

        Ok(albums)
    }

    pub async fn find(
        pool: &PgPool,
        filter: Option<&FilterNode>,
        limit: i32,
        offset: i32,
    ) -> AppResult<Vec<Album>> {
        let mut qb: QueryBuilder<Postgres> = QueryBuilder::new("SELECT albums.* FROM albums");
        if let Some(filter) = filter {
            qb.push(" WHERE ");
            compile_albums_filter(&mut qb, filter)
                .map_err(|e| crate::error::AppError::BadRequest(e.to_string()))?;
        }
        qb.push(" ORDER BY albums.sort_title, albums.title");
        qb.push(" LIMIT ").push_bind(limit as i64);
        qb.push(" OFFSET ").push_bind(offset as i64);

        let albums = qb.build_query_as::<Album>().fetch_all(pool).await?;
        Ok(albums)
    }

    pub async fn count(pool: &PgPool, filter: Option<&FilterNode>) -> AppResult<i64> {
        let mut qb: QueryBuilder<Postgres> = QueryBuilder::new("SELECT COUNT(*) FROM albums");
        if let Some(filter) = filter {
            qb.push(" WHERE ");
            compile_albums_filter(&mut qb, filter)
                .map_err(|e| crate::error::AppError::BadRequest(e.to_string()))?;
        }
        let row: (i64,) = qb.build_query_as().fetch_one(pool).await?;
        Ok(row.0)
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
                AND deleted_at IS NULL
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

    pub async fn update(
        executor: impl PgExecutor<'_>,
        id: Uuid,
        title: &str,
        sort_title: &str,
        artist_id: Option<Uuid>,
        date: Option<NaiveDate>,
    ) -> Result<Album, AppError> {
        let updated = sqlx::query_as!(
            Album,
            r#"
            UPDATE
                albums
            SET
                title = $2,
                sort_title = $3,
                artist_id = $4,
                date = $5,
                locked_at = NOW(),
                updated_at = NOW()
            WHERE
                id = $1
            RETURNING
                *
            "#,
            id,
            title,
            sort_title,
            artist_id,
            date,
        )
        .fetch_one(executor)
        .await?;

        Ok(updated)
    }

    pub async fn clear_lock(executor: impl PgExecutor<'_>, id: Uuid) -> Result<(), AppError> {
        sqlx::query!(
            r#"UPDATE
    albums
SET
    locked_at = NULL
WHERE
    id = $1"#,
            id
        )
        .execute(executor)
        .await?;
        Ok(())
    }

    /// Hard-deletes the album only if it has no live tracks. Returns true if deleted.
    pub async fn delete_if_empty(
        executor: impl PgExecutor<'_>,
        id: Uuid,
    ) -> Result<bool, AppError> {
        let result = sqlx::query!(
            r#"
            DELETE FROM albums
            WHERE id = $1
                AND NOT EXISTS (
                    SELECT
                        1
                    FROM
                        tracks
                    WHERE
                        album_id = $1
                        AND deleted_at IS NULL)
            "#,
            id
        )
        .execute(executor)
        .await?;
        Ok(result.rows_affected() > 0)
    }
}
