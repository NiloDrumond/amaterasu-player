use std::str::FromStr;

use chrono::NaiveDate;
use sqlx::{PgExecutor, PgPool, Postgres, QueryBuilder};
use uuid::Uuid;

use crate::db::entities::Track;
use crate::dto::request::SortDir;
use crate::error::{AppError, AppResult};
use crate::filters::{compile_tracks_filter, FilterNode};

pub struct TrackRepository;

#[derive(Debug, Clone, Copy)]
pub enum TrackSortKey {
    Title,
    TrackNo,
    Album,
    Artist,
    DurationMs,
    PlayCount,
    Random,
}

impl FromStr for TrackSortKey {
    type Err = AppError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "title" => Ok(Self::Title),
            "trackNo" => Ok(Self::TrackNo),
            "album" => Ok(Self::Album),
            "artist" => Ok(Self::Artist),
            "durationMs" => Ok(Self::DurationMs),
            "playCount" => Ok(Self::PlayCount),
            "random" => Ok(Self::Random),
            other => Err(AppError::BadRequest(format!("invalid sort key: {other}"))),
        }
    }
}

#[derive(Debug, Default)]
pub struct TrackUpdate {
    pub title: Option<String>,
    pub sort_title: Option<String>,
    pub artist_id: Option<Option<Uuid>>,
    pub album_id: Option<Option<Uuid>>,
    pub disc: Option<Option<i32>>,
    pub track_no: Option<Option<i32>>,
    pub date: Option<Option<NaiveDate>>,
    pub composer: Option<Option<String>>,
    pub comment: Option<Option<String>>,
}

impl TrackRepository {
    pub async fn create(executor: impl PgExecutor<'_>, track: &Track) -> AppResult<Track> {
        let created = sqlx::query_as!(
            Track,
            r#"
            INSERT INTO tracks (id, audio_hash, album_id, file_path, title, sort_title, artist_id, disc, track_no, date, composer, comment, original_title, original_artist, original_album, format, codec, duration_ms, bitrate, sample_rate, channels, file_size_bytes, file_modified_at, replaygain_track_gain, replaygain_track_peak, metadata_modified_at, deleted_at, locked_at, created_at, updated_at)
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22, $23, $24, $25, $26, $27, $28, $29, $30)
            RETURNING
                *
            "#,
            track.id,
            track.audio_hash,
            track.album_id,
            track.file_path,
            track.title,
            track.sort_title,
            track.artist_id,
            track.disc,
            track.track_no,
            track.date,
            track.composer,
            track.comment,
            track.original_title,
            track.original_artist,
            track.original_album,
            track.format,
            track.codec,
            track.duration_ms,
            track.bitrate,
            track.sample_rate,
            track.channels,
            track.file_size_bytes,
            track.file_modified_at,
            track.replaygain_track_gain,
            track.replaygain_track_peak,
            track.metadata_modified_at,
            track.deleted_at,
            track.locked_at,
            track.created_at,
            track.updated_at
        )
        .fetch_one(executor)
        .await?;

        Ok(created)
    }

    pub async fn find_by_id(executor: impl PgExecutor<'_>, id: Uuid) -> AppResult<Option<Track>> {
        let track = sqlx::query_as!(
            Track,
            r#"
            SELECT
                *
            FROM
                tracks
            WHERE
                id = $1
            "#,
            id
        )
        .fetch_optional(executor)
        .await?;

        Ok(track)
    }

    pub async fn find_by_ids(executor: impl PgExecutor<'_>, ids: &[Uuid]) -> AppResult<Vec<Track>> {
        let tracks = sqlx::query_as!(Track, r#"SELECT * FROM tracks WHERE id = ANY($1)"#, ids)
            .fetch_all(executor)
            .await?;

        Ok(tracks)
    }

    pub async fn find_by_audio_hash(
        executor: impl PgExecutor<'_>,
        audio_hash: &[u8],
    ) -> AppResult<Option<Track>> {
        let track = sqlx::query_as!(
            Track,
            r#"
            SELECT
                *
            FROM
                tracks
            WHERE
                audio_hash = $1
            "#,
            audio_hash
        )
        .fetch_optional(executor)
        .await?;

        Ok(track)
    }

    pub async fn find_by_album_and_title(
        executor: impl PgExecutor<'_>,
        album_id: Uuid,
        title: &str,
    ) -> AppResult<Option<Track>> {
        let track = sqlx::query_as!(
            Track,
            r#"
            SELECT
                *
            FROM
                tracks
            WHERE
                album_id = $1 AND lower(title) = lower($2)
            LIMIT 1
            "#,
            album_id,
            title
        )
        .fetch_optional(executor)
        .await?;

        Ok(track)
    }

    /// Full-row update used by the scanner when refreshing tag-derived fields.
    pub async fn update(executor: impl PgExecutor<'_>, track: &Track) -> AppResult<Track> {
        let updated = sqlx::query_as!(
            Track,
            r#"
            UPDATE
                tracks
            SET
                audio_hash = $2,
                album_id = $3,
                file_path = $4,
                title = $5,
                sort_title = $6,
                artist_id = $7,
                disc = $8,
                track_no = $9,
                date = $10,
                composer = $11,
                comment = $12,
                original_title = $13,
                original_artist = $14,
                original_album = $15,
                format = $16,
                codec = $17,
                duration_ms = $18,
                bitrate = $19,
                sample_rate = $20,
                channels = $21,
                file_size_bytes = $22,
                file_modified_at = $23,
                replaygain_track_gain = $24,
                replaygain_track_peak = $25,
                updated_at = NOW()
            WHERE
                id = $1
            RETURNING
                *
            "#,
            track.id,
            track.audio_hash,
            track.album_id,
            track.file_path,
            track.title,
            track.sort_title,
            track.artist_id,
            track.disc,
            track.track_no,
            track.date,
            track.composer,
            track.comment,
            track.original_title,
            track.original_artist,
            track.original_album,
            track.format,
            track.codec,
            track.duration_ms,
            track.bitrate,
            track.sample_rate,
            track.channels,
            track.file_size_bytes,
            track.file_modified_at,
            track.replaygain_track_gain,
            track.replaygain_track_peak
        )
        .fetch_one(executor)
        .await?;

        Ok(updated)
    }

    /// Re-points every track currently on `from_artist_id` to `to_artist_id`.
    /// Returns the number of rows updated. Used by the admin merge flow.
    pub async fn reassign_artist(
        executor: impl PgExecutor<'_>,
        from_artist_id: Uuid,
        to_artist_id: Uuid,
    ) -> AppResult<u64> {
        let result = sqlx::query!(
            r#"UPDATE tracks SET artist_id = $2 WHERE artist_id = $1"#,
            from_artist_id,
            to_artist_id,
        )
        .execute(executor)
        .await?;
        Ok(result.rows_affected())
    }

    /// Re-points every track currently on `from_album_id` to `to_album_id`.
    /// Returns the number of rows updated. Used by the admin merge flow.
    pub async fn reassign_album(
        executor: impl PgExecutor<'_>,
        from_album_id: Uuid,
        to_album_id: Uuid,
    ) -> AppResult<u64> {
        let result = sqlx::query!(
            r#"UPDATE tracks SET album_id = $2 WHERE album_id = $1"#,
            from_album_id,
            to_album_id,
        )
        .execute(executor)
        .await?;
        Ok(result.rows_affected())
    }

    /// Refreshes only `file_path` (used by scanner when a locked or
    /// soft-deleted track has been moved on disk).
    pub async fn update_file_path(
        executor: impl PgExecutor<'_>,
        id: Uuid,
        file_path: &str,
    ) -> AppResult<()> {
        sqlx::query!(
            r#"UPDATE
    tracks
SET
    file_path = $2
WHERE
    id = $1"#,
            id,
            file_path
        )
        .execute(executor)
        .await?;
        Ok(())
    }

    /// Admin partial update: only writes columns that were provided. Sets
    /// `locked_at = NOW()`. Each field uses double-Option semantics: outer
    /// `None` means "don't change", inner `None` means "set to NULL".
    pub async fn admin_update(
        executor: impl PgExecutor<'_>,
        id: Uuid,
        patch: &TrackUpdate,
    ) -> AppResult<Track> {
        let updated = sqlx::query_as!(
            Track,
            r#"
            UPDATE
                tracks
            SET
                title = COALESCE($2, title),
                sort_title = COALESCE($3, sort_title),
                artist_id = CASE WHEN $4::bool THEN
                    $5
                ELSE
                    artist_id
                END,
                album_id = CASE WHEN $6::bool THEN
                    $7
                ELSE
                    album_id
                END,
                disc = CASE WHEN $8::bool THEN
                    $9
                ELSE
                    disc
                END,
                track_no = CASE WHEN $10::bool THEN
                    $11
                ELSE
                    track_no
                END,
                date = CASE WHEN $12::bool THEN
                    $13
                ELSE
                    date
                END,
                composer = CASE WHEN $14::bool THEN
                    $15
                ELSE
                    composer
                END,
                comment = CASE WHEN $16::bool THEN
                    $17
                ELSE
                    comment
                END,
                locked_at = NOW(),
                updated_at = NOW()
            WHERE
                id = $1
            RETURNING
                *
            "#,
            id,
            patch.title.as_deref(),
            patch.sort_title.as_deref(),
            patch.artist_id.is_some(),
            patch.artist_id.flatten(),
            patch.album_id.is_some(),
            patch.album_id.flatten(),
            patch.disc.is_some(),
            patch.disc.flatten(),
            patch.track_no.is_some(),
            patch.track_no.flatten(),
            patch.date.is_some(),
            patch.date.flatten(),
            patch.composer.is_some(),
            patch.composer.as_ref().and_then(|o| o.as_deref()),
            patch.comment.is_some(),
            patch.comment.as_ref().and_then(|o| o.as_deref()),
        )
        .fetch_one(executor)
        .await?;
        Ok(updated)
    }

    /// Batch admin update — applies the same patch to every id in `ids`.
    /// Sets `locked_at = NOW()` on each matched row. Returns the rows-affected count.
    pub async fn admin_update_many(
        executor: impl PgExecutor<'_>,
        ids: &[Uuid],
        patch: &TrackUpdate,
    ) -> AppResult<u64> {
        let result = sqlx::query!(
            r#"
            UPDATE
                tracks
            SET
                title = COALESCE($2, title),
                sort_title = COALESCE($3, sort_title),
                artist_id = CASE WHEN $4::bool THEN
                    $5
                ELSE
                    artist_id
                END,
                album_id = CASE WHEN $6::bool THEN
                    $7
                ELSE
                    album_id
                END,
                disc = CASE WHEN $8::bool THEN
                    $9
                ELSE
                    disc
                END,
                track_no = CASE WHEN $10::bool THEN
                    $11
                ELSE
                    track_no
                END,
                date = CASE WHEN $12::bool THEN
                    $13
                ELSE
                    date
                END,
                composer = CASE WHEN $14::bool THEN
                    $15
                ELSE
                    composer
                END,
                comment = CASE WHEN $16::bool THEN
                    $17
                ELSE
                    comment
                END,
                locked_at = NOW(),
                updated_at = NOW()
            WHERE
                id = ANY ($1)
            "#,
            ids,
            patch.title.as_deref(),
            patch.sort_title.as_deref(),
            patch.artist_id.is_some(),
            patch.artist_id.flatten(),
            patch.album_id.is_some(),
            patch.album_id.flatten(),
            patch.disc.is_some(),
            patch.disc.flatten(),
            patch.track_no.is_some(),
            patch.track_no.flatten(),
            patch.date.is_some(),
            patch.date.flatten(),
            patch.composer.is_some(),
            patch.composer.as_ref().and_then(|o| o.as_deref()),
            patch.comment.is_some(),
            patch.comment.as_ref().and_then(|o| o.as_deref()),
        )
        .execute(executor)
        .await?;
        Ok(result.rows_affected())
    }

    pub async fn soft_delete(executor: impl PgExecutor<'_>, id: Uuid) -> AppResult<()> {
        sqlx::query!(
            r#"UPDATE
    tracks
SET
    deleted_at = NOW()
WHERE
    id = $1
    AND deleted_at IS NULL"#,
            id
        )
        .execute(executor)
        .await?;
        Ok(())
    }

    pub async fn restore(executor: impl PgExecutor<'_>, id: Uuid) -> AppResult<()> {
        sqlx::query!(
            r#"UPDATE
    tracks
SET
    deleted_at = NULL
WHERE
    id = $1"#,
            id
        )
        .execute(executor)
        .await?;
        Ok(())
    }

    pub async fn force_rescan(executor: impl PgExecutor<'_>, id: Uuid) -> AppResult<()> {
        sqlx::query!(
            r#"UPDATE
    tracks
SET
    deleted_at = NULL,
    locked_at = NULL
WHERE
    id = $1"#,
            id
        )
        .execute(executor)
        .await?;
        Ok(())
    }

    /// Permanently deletes the row only if it's already soft-deleted.
    /// Returns true if a row was deleted.
    pub async fn hard_delete_if_soft_deleted(
        executor: impl PgExecutor<'_>,
        id: Uuid,
    ) -> AppResult<bool> {
        let result = sqlx::query!(
            r#"DELETE FROM tracks
WHERE id = $1
    AND deleted_at IS NOT NULL"#,
            id
        )
        .execute(executor)
        .await?;
        Ok(result.rows_affected() > 0)
    }

    pub async fn find_by_album_id(
        executor: impl PgExecutor<'_>,
        album_id: Uuid,
    ) -> AppResult<Vec<Track>> {
        let tracks = sqlx::query_as!(
            Track,
            r#"
            SELECT
                *
            FROM
                tracks
            WHERE
                album_id = $1
                AND deleted_at IS NULL
            ORDER BY
                disc NULLS LAST,
                track_no NULLS LAST
            "#,
            album_id
        )
        .fetch_all(executor)
        .await?;

        Ok(tracks)
    }

    pub async fn find_by_album_ids(
        executor: impl PgExecutor<'_>,
        album_ids: &[Uuid],
    ) -> AppResult<Vec<Track>> {
        let tracks = sqlx::query_as!(
            Track,
            r#"
            SELECT *
            FROM tracks
            WHERE album_id = ANY($1)
              AND deleted_at IS NULL
            ORDER BY album_id, disc NULLS LAST, track_no NULLS LAST
            "#,
            album_ids
        )
        .fetch_all(executor)
        .await?;

        Ok(tracks)
    }

    pub async fn find_by_artist_ids(
        executor: impl PgExecutor<'_>,
        artist_ids: &[Uuid],
    ) -> AppResult<Vec<Track>> {
        let tracks = sqlx::query_as!(
            Track,
            r#"
            SELECT *
            FROM tracks
            WHERE artist_id = ANY($1)
              AND deleted_at IS NULL
            ORDER BY artist_id, disc NULLS LAST, track_no NULLS LAST, title
            "#,
            artist_ids
        )
        .fetch_all(executor)
        .await?;

        Ok(tracks)
    }

    pub async fn find_by_artist_id(
        executor: impl PgExecutor<'_>,
        artist_id: Uuid,
    ) -> AppResult<Vec<Track>> {
        let tracks = sqlx::query_as!(
            Track,
            r#"
            SELECT
                *
            FROM
                tracks
            WHERE
                artist_id = $1
                AND deleted_at IS NULL
            ORDER BY
                disc NULLS LAST,
                track_no NULLS LAST,
                title
            "#,
            artist_id
        )
        .fetch_all(executor)
        .await?;

        Ok(tracks)
    }

    pub async fn find_deleted(executor: impl PgExecutor<'_>) -> AppResult<Vec<Track>> {
        let tracks = sqlx::query_as!(
            Track,
            r#"
            SELECT
                *
            FROM
                tracks
            WHERE
                deleted_at IS NOT NULL
            ORDER BY
                deleted_at DESC
            "#
        )
        .fetch_all(executor)
        .await?;

        Ok(tracks)
    }

    pub async fn get_track_count_for_artist_ids(
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
                COUNT(*) AS "track_count!"
            FROM
                tracks
            WHERE
                artist_id = ANY ($1)
                AND deleted_at IS NULL
            GROUP BY
                artist_id
            "#,
            ids
        )
        .fetch_all(executor)
        .await?;

        Ok(rows
            .into_iter()
            .map(|r| (r.artist_id, r.track_count))
            .collect())
    }

    pub async fn find(
        pool: &PgPool,
        filter: Option<&FilterNode>,
        params: &super::FindParams<TrackSortKey>,
    ) -> AppResult<Vec<Track>> {
        let super::FindParams {
            limit,
            offset,
            sort,
            dir,
            seed,
            user_id,
        } = params;
        let mut qb: QueryBuilder<Postgres> = QueryBuilder::new(
            "SELECT tracks.* FROM tracks \
             LEFT JOIN albums ON albums.id = tracks.album_id \
             LEFT JOIN artists ON artists.id = tracks.artist_id",
        );
        if matches!(sort, Some(TrackSortKey::PlayCount)) {
            qb.push(" LEFT JOIN (SELECT track_id, COUNT(*) AS play_count FROM track_plays WHERE user_id = ");
            qb.push_bind(user_id.unwrap_or(Uuid::nil()));
            qb.push(" GROUP BY track_id) tp ON tp.track_id = tracks.id");
        }
        qb.push(" WHERE tracks.deleted_at IS NULL");
        if let Some(filter) = filter {
            qb.push(" AND ");
            compile_tracks_filter(&mut qb, filter, user_id.unwrap_or(Uuid::nil()))
                .map_err(|e| crate::error::AppError::BadRequest(e.to_string()))?;
        }
        let d = dir.unwrap_or(SortDir::Asc).as_sql();
        match sort {
            None => {
                qb.push(" ORDER BY tracks.sort_title, tracks.title, tracks.id");
            }
            Some(TrackSortKey::Title) => {
                qb.push(format!(
                    " ORDER BY tracks.sort_title {d}, tracks.title {d}, tracks.id"
                ));
            }
            Some(TrackSortKey::TrackNo) => {
                qb.push(format!(
                    " ORDER BY tracks.disc {d} NULLS LAST, tracks.track_no {d} NULLS LAST, tracks.id"
                ));
            }
            Some(TrackSortKey::Album) => {
                qb.push(format!(
                    " ORDER BY albums.sort_title {d} NULLS LAST, tracks.disc, tracks.track_no, tracks.id"
                ));
            }
            Some(TrackSortKey::Artist) => {
                qb.push(format!(
                    " ORDER BY artists.sort_name {d} NULLS LAST, tracks.id"
                ));
            }
            Some(TrackSortKey::DurationMs) => {
                qb.push(format!(
                    " ORDER BY tracks.duration_ms {d} NULLS LAST, tracks.id"
                ));
            }
            Some(TrackSortKey::PlayCount) => {
                qb.push(format!(
                    " ORDER BY COALESCE(tp.play_count, 0) {d}, tracks.id"
                ));
            }
            Some(TrackSortKey::Random) => {
                qb.push(" ORDER BY md5(");
                qb.push_bind(seed.unwrap_or(0).to_string());
                qb.push(" || tracks.id::text), tracks.id");
            }
        };
        qb.push(" LIMIT ").push_bind(*limit as i64);
        qb.push(" OFFSET ").push_bind(*offset as i64);

        let tracks = qb.build_query_as::<Track>().fetch_all(pool).await?;
        Ok(tracks)
    }

    pub async fn count(
        pool: &PgPool,
        filter: Option<&FilterNode>,
        user_id: Uuid,
    ) -> AppResult<i64> {
        let mut qb: QueryBuilder<Postgres> =
            QueryBuilder::new("SELECT COUNT(*) FROM tracks WHERE tracks.deleted_at IS NULL");
        if let Some(filter) = filter {
            qb.push(" AND ");
            compile_tracks_filter(&mut qb, filter, user_id)
                .map_err(|e| crate::error::AppError::BadRequest(e.to_string()))?;
        }
        let row: (i64,) = qb.build_query_as().fetch_one(pool).await?;
        Ok(row.0)
    }

    pub async fn set_approved(
        executor: impl PgExecutor<'_>,
        id: Uuid,
        approved: bool,
    ) -> AppResult<Option<Track>> {
        let updated = sqlx::query_as!(
            Track,
            r#"
            UPDATE tracks
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

    /// Approves every non-deleted track on the given album. Returns rows affected.
    pub async fn approve_all_for_album(
        executor: impl PgExecutor<'_>,
        album_id: Uuid,
    ) -> AppResult<u64> {
        let result = sqlx::query!(
            r#"
            UPDATE tracks
            SET approved = TRUE, updated_at = NOW()
            WHERE album_id = $1 AND deleted_at IS NULL AND approved = FALSE
            "#,
            album_id
        )
        .execute(executor)
        .await?;
        Ok(result.rows_affected())
    }

    pub async fn count_pending(executor: impl PgExecutor<'_>) -> AppResult<i64> {
        let row = sqlx::query!(
            r#"SELECT COUNT(*) AS "n!" FROM tracks WHERE approved = FALSE AND deleted_at IS NULL"#
        )
        .fetch_one(executor)
        .await?;
        Ok(row.n)
    }
}
