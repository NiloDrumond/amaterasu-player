use crate::db::entities::Track;
use crate::error::AppResult;
use sqlx::PgPool;
use uuid::Uuid;

pub struct TrackRepository {
    db: PgPool,
}

impl TrackRepository {
    pub fn new(db: PgPool) -> Self {
        Self { db }
    }

    /// Create a new track
    pub async fn create(&self, track: &Track) -> AppResult<Track> {
        let created = sqlx::query_as!(
            Track,
            r#"
            INSERT INTO tracks (id, audio_hash, album_id, file_path, title, artist_id, disc, track_no, date, composer, comment, duration_ms, bitrate, sample_rate, channels, file_size_bytes, file_modified_at, replaygain_track_gain, replaygain_track_peak, metadata_modified_at, created_at, updated_at)
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22)
            RETURNING
                *
            "#,
            track.id,
            track.audio_hash,
            track.album_id,
            track.file_path,
            track.title,
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
        .fetch_one(&self.db)
        .await?;

        Ok(created)
    }

    /// Find track by ID
    pub async fn find_by_id(&self, id: Uuid) -> AppResult<Option<Track>> {
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
        .fetch_optional(&self.db)
        .await?;

        Ok(track)
    }

    /// Find track by file path
    pub async fn find_by_file_path(&self, file_path: &str) -> AppResult<Option<Track>> {
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
        .fetch_optional(&self.db)
        .await?;

        Ok(track)
    }

    /// Find tracks by album
    pub async fn find_by_album(&self, album_id: Uuid) -> AppResult<Vec<Track>> {
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
        .fetch_all(&self.db)
        .await?;

        Ok(tracks)
    }

    /// Update technical metadata (from file rescan)
    pub async fn update_technical_metadata(&self, track: &Track) -> AppResult<Track> {
        let updated = sqlx::query_as!(
            Track,
            r#"
            UPDATE
                tracks
            SET
                duration_ms = $2,
                bitrate = $3,
                sample_rate = $4,
                channels = $5,
                file_size_bytes = $6,
                file_modified_at = $7,
                replaygain_track_gain = $8,
                replaygain_track_peak = $9
            WHERE
                id = $1
            RETURNING
                *
            "#,
            track.id,
            track.duration_ms,
            track.bitrate,
            track.sample_rate,
            track.channels,
            track.file_size_bytes,
            track.file_modified_at,
            track.replaygain_track_gain,
            track.replaygain_track_peak
        )
        .fetch_one(&self.db)
        .await?;

        Ok(updated)
    }

    /// Delete a track
    pub async fn delete(&self, id: Uuid) -> AppResult<bool> {
        let result = sqlx::query!(
            r#"
            DELETE FROM tracks
            WHERE id = $1
            "#,
            id
        )
        .execute(&self.db)
        .await?;

        Ok(result.rows_affected() > 0)
    }

    pub async fn find_all(&self, limit: i64, offset: i64) -> AppResult<Vec<Track>> {
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
        .fetch_all(&self.db)
        .await?;

        Ok(tracks)
    }

    /// Count total tracks
    pub async fn count(&self) -> AppResult<i64> {
        let record = sqlx::query!(
            r#"
            SELECT
                COUNT(*) AS count
            FROM
                tracks
            "#
        )
        .fetch_one(&self.db)
        .await?;

        Ok(record.count.unwrap_or(0))
    }

    /// Get tracks that have been edited by user
    pub async fn find_user_edited(&self) -> AppResult<Vec<Track>> {
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
        .fetch_all(&self.db)
        .await?;

        Ok(tracks)
    }
}
