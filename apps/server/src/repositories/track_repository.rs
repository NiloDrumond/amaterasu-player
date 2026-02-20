use crate::db::entities::Track;
use crate::error::AppResult;
use sqlx::PgExecutor;
use uuid::Uuid;

pub struct TrackRepository;

impl TrackRepository {
    pub async fn create(executor: impl PgExecutor<'_>, track: &Track) -> AppResult<Track> {
        let created = sqlx::query_as!(
            Track,
            r#"
            INSERT INTO tracks (id, audio_hash, album_id, file_path, title, sort_title, artist_id, disc, track_no, date, composer, comment, duration_ms, bitrate, sample_rate, channels, file_size_bytes, file_modified_at, replaygain_track_gain, replaygain_track_peak, metadata_modified_at, created_at, updated_at)
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22, $23)
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
            track.duration_ms,
            track.bitrate,
            track.sample_rate,
            track.channels,
            track.file_size_bytes,
            track.file_modified_at,
            track.replaygain_track_gain,
            track.replaygain_track_peak,
            track.metadata_modified_at,
            track.created_at,
            track.updated_at
        )
        .fetch_one(executor)
        .await?;

        Ok(created)
    }

    pub async fn find_by_id(
        executor: impl PgExecutor<'_>,
        id: Uuid,
    ) -> AppResult<Option<Track>> {
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

    pub async fn find_by_file_path(
        executor: impl PgExecutor<'_>,
        file_path: &str,
    ) -> AppResult<Option<Track>> {
        let track = sqlx::query_as!(
            Track,
            r#"
            SELECT
                *
            FROM
                tracks
            WHERE
                file_path = $1
            "#,
            file_path
        )
        .fetch_optional(executor)
        .await?;

        Ok(track)
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

    pub async fn find_by_album(
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
            ORDER BY
                disc,
                track_no,
                title
            "#,
            album_id
        )
        .fetch_all(executor)
        .await?;

        Ok(tracks)
    }

    pub async fn update(executor: impl PgExecutor<'_>, track: &Track) -> AppResult<Track> {
        let updated = sqlx::query_as!(
            Track,
            r#"
            UPDATE
                tracks
            SET
                audio_hash = $2,
                album_id = $3,
                title = $4,
                sort_title = $5,
                artist_id = $6,
                disc = $7,
                track_no = $8,
                date = $9,
                composer = $10,
                comment = $11,
                duration_ms = $12,
                bitrate = $13,
                sample_rate = $14,
                channels = $15,
                file_size_bytes = $16,
                file_modified_at = $17,
                replaygain_track_gain = $18,
                replaygain_track_peak = $19,
                updated_at = NOW()
            WHERE
                id = $1
            RETURNING
                *
            "#,
            track.id,
            track.audio_hash,
            track.album_id,
            track.title,
            track.sort_title,
            track.artist_id,
            track.disc,
            track.track_no,
            track.date,
            track.composer,
            track.comment,
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

    pub async fn delete(executor: impl PgExecutor<'_>, id: Uuid) -> AppResult<bool> {
        let result = sqlx::query!(
            r#"
            DELETE FROM tracks
            WHERE id = $1
            "#,
            id
        )
        .execute(executor)
        .await?;

        Ok(result.rows_affected() > 0)
    }

    pub async fn find_all(
        executor: impl PgExecutor<'_>,
        limit: i64,
        offset: i64,
    ) -> AppResult<Vec<Track>> {
        let tracks = sqlx::query_as!(
            Track,
            r#"
            SELECT
                *
            FROM
                tracks
            ORDER BY
                disc,
                track_no,
                title
            LIMIT $1 OFFSET $2
            "#,
            limit,
            offset
        )
        .fetch_all(executor)
        .await?;

        Ok(tracks)
    }

    pub async fn count(executor: impl PgExecutor<'_>) -> AppResult<i64> {
        let record = sqlx::query!(
            r#"
            SELECT
                COUNT(*) AS count
            FROM
                tracks
            "#
        )
        .fetch_one(executor)
        .await?;

        Ok(record.count.unwrap_or(0))
    }

    pub async fn find_user_edited(executor: impl PgExecutor<'_>) -> AppResult<Vec<Track>> {
        let tracks = sqlx::query_as!(
            Track,
            r#"
            SELECT
                *
            FROM
                tracks
            WHERE
                metadata_modified_at IS NOT NULL
            ORDER BY
                metadata_modified_at DESC
            "#
        )
        .fetch_all(executor)
        .await?;

        Ok(tracks)
    }
}
