use std::str::FromStr;

use chrono::NaiveDate;
use sqlx::{PgExecutor, PgPool, Postgres, QueryBuilder};
use uuid::Uuid;

use crate::db::entities::Album;
use crate::dto::request::SortDir;
use crate::error::{AppError, AppResult};
use crate::filters::{compile_albums_filter, FilterNode};

pub struct AlbumRepository;

#[derive(Debug, Clone, Copy)]
pub enum AlbumSortKey {
    Title,
    Artist,
    Year,
    TrackCount,
    Time,
    PlayCount,
    Recent,
    Random,
}

impl FromStr for AlbumSortKey {
    type Err = AppError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "title" => Ok(Self::Title),
            "artist" => Ok(Self::Artist),
            "year" => Ok(Self::Year),
            "trackCount" => Ok(Self::TrackCount),
            "time" => Ok(Self::Time),
            "playCount" => Ok(Self::PlayCount),
            "recent" => Ok(Self::Recent),
            "random" => Ok(Self::Random),
            other => Err(AppError::BadRequest(format!("invalid sort key: {other}"))),
        }
    }
}

impl AlbumRepository {
    pub async fn create(executor: impl PgExecutor<'_>, album: &Album) -> Result<Album, AppError> {
        let created = sqlx::query_as!(
            Album,
            r#"
            INSERT INTO albums (id, artist_id, title, sort_title, date, mbid, cover_path, locked_at, replaygain_album_gain, replaygain_album_peak, created_at, updated_at)
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
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
        sort: Option<AlbumSortKey>,
        dir: Option<SortDir>,
        seed: Option<i64>,
        user_id: Option<Uuid>,
    ) -> AppResult<Vec<Album>> {
        let mut qb: QueryBuilder<Postgres> = QueryBuilder::new(
            "SELECT albums.* FROM albums \
             LEFT JOIN artists ON artists.id = albums.artist_id \
             LEFT JOIN ( \
               SELECT album_id, COUNT(*) AS track_count, \
                      COALESCE(SUM(duration_ms), 0)::bigint AS total_duration_ms \
               FROM tracks WHERE deleted_at IS NULL AND album_id IS NOT NULL \
               GROUP BY album_id \
             ) agg ON agg.album_id = albums.id",
        );
        if matches!(sort, Some(AlbumSortKey::PlayCount)) {
            qb.push(" LEFT JOIN (SELECT context_album_id, COUNT(*) AS play_count FROM track_plays WHERE user_id = ");
            qb.push_bind(user_id.unwrap_or(Uuid::nil()));
            qb.push(" GROUP BY context_album_id) tp ON tp.context_album_id = albums.id");
        }
        if let Some(filter) = filter {
            qb.push(" WHERE ");
            compile_albums_filter(&mut qb, filter)
                .map_err(|e| crate::error::AppError::BadRequest(e.to_string()))?;
        }
        let d = dir.unwrap_or(SortDir::Asc).as_sql();
        match sort {
            None => {
                qb.push(" ORDER BY albums.sort_title, albums.title, albums.id");
            }
            Some(AlbumSortKey::Title) => {
                qb.push(format!(
                    " ORDER BY albums.sort_title {d}, albums.title {d}, albums.id"
                ));
            }
            Some(AlbumSortKey::Artist) => {
                qb.push(format!(
                    " ORDER BY artists.sort_name {d} NULLS LAST, albums.sort_title, albums.id"
                ));
            }
            Some(AlbumSortKey::Year) => {
                qb.push(format!(
                    " ORDER BY albums.date {d} NULLS LAST, albums.sort_title, albums.id"
                ));
            }
            Some(AlbumSortKey::TrackCount) => {
                qb.push(format!(
                    " ORDER BY agg.track_count {d} NULLS LAST, albums.sort_title, albums.id"
                ));
            }
            Some(AlbumSortKey::Time) => {
                qb.push(format!(
                    " ORDER BY agg.total_duration_ms {d} NULLS LAST, albums.sort_title, albums.id"
                ));
            }
            Some(AlbumSortKey::PlayCount) => {
                qb.push(format!(
                    " ORDER BY COALESCE(tp.play_count, 0) {d}, albums.sort_title, albums.id"
                ));
            }
            Some(AlbumSortKey::Recent) => {
                qb.push(format!(" ORDER BY albums.created_at {d}, albums.id"));
            }
            Some(AlbumSortKey::Random) => {
                qb.push(" ORDER BY md5(");
                qb.push_bind(seed.unwrap_or(0).to_string());
                qb.push(" || albums.id::text), albums.id");
            }
        };
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

    /// Overwrite `cover_path` unconditionally. Used by the admin merge flow
    /// when the admin picks which side's cover to keep.
    pub async fn set_cover_path(
        executor: impl PgExecutor<'_>,
        id: Uuid,
        cover_path: Option<&str>,
    ) -> Result<(), AppError> {
        sqlx::query!(
            r#"UPDATE albums SET cover_path = $2, updated_at = NOW() WHERE id = $1"#,
            id,
            cover_path,
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

    /// Stamps the MusicBrainz release-group MBID without touching anything
    /// else. Used after the admin accepts a MB suggestion.
    pub async fn set_mbid(
        executor: impl PgExecutor<'_>,
        id: Uuid,
        mbid: &str,
    ) -> Result<(), AppError> {
        sqlx::query!(
            r#"UPDATE albums SET mbid = $2, updated_at = NOW() WHERE id = $1"#,
            id,
            mbid,
        )
        .execute(executor)
        .await?;
        Ok(())
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

    pub async fn delete(executor: impl PgExecutor<'_>, id: Uuid) -> Result<(), AppError> {
        sqlx::query!(r#"DELETE FROM albums WHERE id = $1"#, id)
            .execute(executor)
            .await?;
        Ok(())
    }

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

    pub async fn set_approved(
        executor: impl PgExecutor<'_>,
        id: Uuid,
        approved: bool,
    ) -> Result<Option<Album>, AppError> {
        let updated = sqlx::query_as!(
            Album,
            r#"
            UPDATE albums
            SET approved = $2, updated_at = NOW()
            WHERE id = $1
            RETURNING *
            "#,
            id,
            approved
        )
        .fetch_optional(executor)
        .await?;
        Ok(updated)
    }

    pub async fn count_pending(executor: impl PgExecutor<'_>) -> Result<i64, AppError> {
        let row = sqlx::query!(r#"SELECT COUNT(*) AS "n!" FROM albums WHERE approved = FALSE"#)
            .fetch_one(executor)
            .await?;
        Ok(row.n)
    }

    /// IDs of albums that either are pending themselves OR have at least one
    /// pending non-deleted track. Newest-album-first. Used by the review queue.
    pub async fn find_pending_affected_ids(
        executor: impl PgExecutor<'_>,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<Uuid>, AppError> {
        let rows = sqlx::query!(
            r#"
            SELECT a.id AS "id!"
            FROM albums a
            WHERE a.approved = FALSE
               OR EXISTS (
                    SELECT 1 FROM tracks t
                    WHERE t.album_id = a.id
                      AND t.approved = FALSE
                      AND t.deleted_at IS NULL
               )
            ORDER BY a.created_at DESC
            LIMIT $1 OFFSET $2
            "#,
            limit,
            offset,
        )
        .fetch_all(executor)
        .await?;
        Ok(rows.into_iter().map(|r| r.id).collect())
    }
}
