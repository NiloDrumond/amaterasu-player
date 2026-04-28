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
            INSERT INTO tracks (id, audio_hash, album_id, file_path, title, sort_title, artist_id, disc, track_no, date, composer, comment, original_title, original_artist, original_album, format, codec, duration_ms, bitrate, sample_rate, channels, file_size_bytes, file_modified_at, replaygain_track_gain, replaygain_track_peak, metadata_modified_at, created_at, updated_at)
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22, $23, $24, $25, $26, $27, $28)
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
                original_title = $12,
                original_artist = $13,
                original_album = $14,
                format = $15,
                codec = $16,
                duration_ms = $17,
                bitrate = $18,
                sample_rate = $19,
                channels = $20,
                file_size_bytes = $21,
                file_modified_at = $22,
                replaygain_track_gain = $23,
                replaygain_track_peak = $24,
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

    pub async fn find_all(
        executor: impl PgExecutor<'_>,
        limit: i32,
        offset: i32,
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
            limit as i64,
            offset as i64,
        )
        .fetch_all(executor)
        .await?;

        Ok(tracks)
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
}
