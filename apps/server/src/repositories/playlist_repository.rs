use sqlx::PgExecutor;
use uuid::Uuid;

use crate::db::entities::{Playlist, PlaylistTrack};
use crate::error::AppResult;

pub struct PlaylistRepository;

// A row returned when querying playlist details (with aggregate stats)
pub struct PlaylistStats {
    pub playlist: Playlist,
    pub track_count: i64,
    pub total_duration_ms: i64,
}

// A row returned when listing tracks in a playlist (flat join)
pub struct PlaylistTrackRow {
    pub playlist_track_id: Uuid,
    pub position: f64,
    pub added_at: chrono::DateTime<chrono::Utc>,
    pub track_id: Uuid,
    pub title: String,
    #[allow(dead_code)]
    pub sort_title: String,
    pub artist_id: Option<Uuid>,
    pub album_id: Option<Uuid>,
    pub track_no: Option<i32>,
    pub disc: Option<i32>,
    pub duration_ms: i32,
    pub format: String,
    pub codec: String,
    pub bitrate: Option<i32>,
    pub file_path: String,
    pub original_title: Option<String>,
    pub original_artist: Option<String>,
    pub original_album: Option<String>,
    // artist / album names for the response
    pub artist_name: Option<String>,
    pub album_title: Option<String>,
    pub album_cover_path: Option<String>,
}

impl PlaylistRepository {
    pub async fn create(
        executor: impl PgExecutor<'_>,
        user_id: Uuid,
        name: &str,
    ) -> AppResult<Playlist> {
        let playlist = sqlx::query_as!(
            Playlist,
            r#"
            INSERT INTO playlists (user_id, name)
                VALUES ($1, $2)
            RETURNING
                id,
                user_id,
                name,
                created_at,
                updated_at
            "#,
            user_id,
            name
        )
        .fetch_one(executor)
        .await?;

        Ok(playlist)
    }

    pub async fn find_by_id_and_user(
        executor: impl PgExecutor<'_>,
        id: Uuid,
        user_id: Uuid,
    ) -> AppResult<Option<PlaylistStats>> {
        let row = sqlx::query!(
            r#"
            SELECT
                p.id,
                p.user_id,
                p.name,
                p.created_at,
                p.updated_at,
                COUNT(pt.id) AS "track_count!: i64",
                COALESCE(SUM(t.duration_ms), 0)::bigint AS "total_duration_ms!: i64"
            FROM
                playlists p
            LEFT JOIN playlist_tracks pt ON pt.playlist_id = p.id
            LEFT JOIN tracks t ON t.id = pt.track_id
            WHERE
                p.id = $1
                AND p.user_id = $2
            GROUP BY
                p.id
            "#,
            id,
            user_id
        )
        .fetch_optional(executor)
        .await?;

        Ok(row.map(|r| PlaylistStats {
            playlist: Playlist {
                id: r.id,
                user_id: r.user_id,
                name: r.name,
                created_at: r.created_at,
                updated_at: r.updated_at,
            },
            track_count: r.track_count,
            total_duration_ms: r.total_duration_ms,
        }))
    }

    pub async fn list_by_user(
        executor: impl PgExecutor<'_>,
        user_id: Uuid,
    ) -> AppResult<Vec<PlaylistStats>> {
        let rows = sqlx::query!(
            r#"
            SELECT
                p.id,
                p.user_id,
                p.name,
                p.created_at,
                p.updated_at,
                COUNT(pt.id) AS "track_count!: i64",
                COALESCE(SUM(t.duration_ms), 0)::bigint AS "total_duration_ms!: i64"
            FROM
                playlists p
            LEFT JOIN playlist_tracks pt ON pt.playlist_id = p.id
            LEFT JOIN tracks t ON t.id = pt.track_id
            WHERE
                p.user_id = $1
            GROUP BY
                p.id
            ORDER BY
                p.created_at DESC
            "#,
            user_id
        )
        .fetch_all(executor)
        .await?;

        Ok(rows
            .into_iter()
            .map(|r| PlaylistStats {
                playlist: Playlist {
                    id: r.id,
                    user_id: r.user_id,
                    name: r.name,
                    created_at: r.created_at,
                    updated_at: r.updated_at,
                },
                track_count: r.track_count,
                total_duration_ms: r.total_duration_ms,
            })
            .collect())
    }

    pub async fn rename(
        executor: impl PgExecutor<'_>,
        id: Uuid,
        user_id: Uuid,
        name: &str,
    ) -> AppResult<Option<Playlist>> {
        let playlist = sqlx::query_as!(
            Playlist,
            r#"
            UPDATE playlists
            SET
                name = $3
            WHERE
                id = $1
                AND user_id = $2
            RETURNING
                id,
                user_id,
                name,
                created_at,
                updated_at
            "#,
            id,
            user_id,
            name
        )
        .fetch_optional(executor)
        .await?;

        Ok(playlist)
    }

    pub async fn delete(
        executor: impl PgExecutor<'_>,
        id: Uuid,
        user_id: Uuid,
    ) -> AppResult<bool> {
        let result = sqlx::query!(
            r#"
            DELETE FROM playlists
            WHERE
                id = $1
                AND user_id = $2
            "#,
            id,
            user_id
        )
        .execute(executor)
        .await?;

        Ok(result.rows_affected() > 0)
    }

    /// Returns the maximum position in the playlist, or None if empty.
    pub async fn get_max_position(
        executor: impl PgExecutor<'_>,
        playlist_id: Uuid,
    ) -> AppResult<Option<f64>> {
        let row = sqlx::query!(
            r#"
            SELECT MAX(position) AS max_pos
            FROM playlist_tracks
            WHERE playlist_id = $1
            "#,
            playlist_id
        )
        .fetch_one(executor)
        .await?;

        Ok(row.max_pos)
    }

    pub async fn insert_track(
        executor: impl PgExecutor<'_>,
        playlist_id: Uuid,
        track_id: Uuid,
        position: f64,
    ) -> AppResult<()> {
        sqlx::query_as!(
            PlaylistTrack,
            r#"
            INSERT INTO playlist_tracks (playlist_id, track_id, position)
                VALUES ($1, $2, $3)
            ON CONFLICT (playlist_id, track_id) DO NOTHING
            RETURNING
                id,
                playlist_id,
                track_id,
                position,
                added_at
            "#,
            playlist_id,
            track_id,
            position
        )
        .fetch_optional(executor)
        .await?;

        Ok(())
    }

    pub async fn remove_track(
        executor: impl PgExecutor<'_>,
        playlist_id: Uuid,
        user_id: Uuid,
        track_id: Uuid,
    ) -> AppResult<bool> {
        let result = sqlx::query!(
            r#"
            DELETE FROM playlist_tracks pt
            USING playlists p
            WHERE
                pt.playlist_id = p.id
                AND p.user_id = $3
                AND pt.playlist_id = $1
                AND pt.track_id = $2
            "#,
            playlist_id,
            track_id,
            user_id
        )
        .execute(executor)
        .await?;

        Ok(result.rows_affected() > 0)
    }

    pub async fn list_tracks(
        executor: impl PgExecutor<'_>,
        playlist_id: Uuid,
        user_id: Uuid,
    ) -> AppResult<Vec<PlaylistTrackRow>> {
        let rows = sqlx::query!(
            r#"
            SELECT
                pt.id          AS "playlist_track_id!",
                pt.position    AS "position!",
                pt.added_at    AS "added_at!",
                t.id           AS "track_id!",
                t.title        AS "title!",
                t.sort_title   AS "sort_title!",
                t.artist_id,
                t.album_id,
                t.track_no,
                t.disc,
                t.duration_ms  AS "duration_ms!",
                t.format       AS "format!",
                t.codec        AS "codec!",
                t.bitrate,
                t.file_path    AS "file_path!",
                t.original_title,
                t.original_artist,
                t.original_album,
                ar.name        AS "artist_name?",
                al.title       AS "album_title?",
                al.cover_path  AS "album_cover_path?"
            FROM
                playlist_tracks pt
            JOIN playlists p ON p.id = pt.playlist_id
            JOIN tracks t ON t.id = pt.track_id
            LEFT JOIN artists ar ON ar.id = t.artist_id
            LEFT JOIN albums al ON al.id = t.album_id
            WHERE
                pt.playlist_id = $1
                AND p.user_id = $2
            ORDER BY
                pt.position ASC
            "#,
            playlist_id,
            user_id
        )
        .fetch_all(executor)
        .await?;

        Ok(rows
            .into_iter()
            .map(|r| PlaylistTrackRow {
                playlist_track_id: r.playlist_track_id,
                position: r.position,
                added_at: r.added_at,
                track_id: r.track_id,
                title: r.title,
                sort_title: r.sort_title,
                artist_id: r.artist_id,
                album_id: r.album_id,
                track_no: r.track_no,
                disc: r.disc,
                duration_ms: r.duration_ms,
                format: r.format,
                codec: r.codec,
                bitrate: r.bitrate,
                file_path: r.file_path,
                original_title: r.original_title,
                original_artist: r.original_artist,
                original_album: r.original_album,
                artist_name: r.artist_name,
                album_title: r.album_title,
                album_cover_path: r.album_cover_path,
            })
            .collect())
    }

    /// Returns the neighbor positions for the insertion point.
    ///
    /// Returns:
    /// - `Ok(None)` if `after_id` was provided but not found in the playlist.
    /// - `Ok(Some((prev, next)))` otherwise (`after_id = None` means insert at front).
    ///
    /// Accepts `&mut PgConnection` so it can participate in a transaction
    /// (pass `&mut *tx` from a `Transaction`).
    pub async fn get_neighbor_positions(
        conn: &mut sqlx::PgConnection,
        playlist_id: Uuid,
        after_id: Option<Uuid>,
    ) -> AppResult<Option<(Option<f64>, Option<f64>)>> {
        match after_id {
            None => {
                // Moving to front: prev = None, next = current minimum position
                let row = sqlx::query!(
                    r#"
                    SELECT MIN(position) AS min_pos
                    FROM playlist_tracks
                    WHERE playlist_id = $1
                    "#,
                    playlist_id
                )
                .fetch_one(&mut *conn)
                .await?;
                Ok(Some((None, row.min_pos)))
            }
            Some(after_track_id) => {
                // Get the position of `after_track_id`
                let after_row = sqlx::query!(
                    r#"
                    SELECT position
                    FROM playlist_tracks
                    WHERE playlist_id = $1 AND track_id = $2
                    "#,
                    playlist_id,
                    after_track_id
                )
                .fetch_optional(&mut *conn)
                .await?;

                let Some(after_row) = after_row else {
                    // after_id not found in this playlist
                    return Ok(None);
                };
                let after_pos = after_row.position;

                // Find the next position after after_pos
                let next_row = sqlx::query!(
                    r#"
                    SELECT position
                    FROM playlist_tracks
                    WHERE playlist_id = $1 AND position > $2
                    ORDER BY position ASC
                    LIMIT 1
                    "#,
                    playlist_id,
                    after_pos
                )
                .fetch_optional(&mut *conn)
                .await?;

                Ok(Some((Some(after_pos), next_row.map(|r| r.position))))
            }
        }
    }

    pub async fn update_track_position(
        executor: impl PgExecutor<'_>,
        playlist_id: Uuid,
        track_id: Uuid,
        new_position: f64,
    ) -> AppResult<bool> {
        let result = sqlx::query!(
            r#"
            UPDATE playlist_tracks
            SET position = $3
            WHERE playlist_id = $1 AND track_id = $2
            "#,
            playlist_id,
            track_id,
            new_position
        )
        .execute(executor)
        .await?;

        Ok(result.rows_affected() > 0)
    }

    /// Redistributes all positions in a playlist as 1000, 2000, 3000, ...
    pub async fn redistribute_positions(
        executor: impl PgExecutor<'_>,
        playlist_id: Uuid,
    ) -> AppResult<()> {
        sqlx::query!(
            r#"
            UPDATE playlist_tracks pt
            SET position = sub.new_pos
            FROM (
                SELECT id, ROW_NUMBER() OVER (ORDER BY position ASC) * 1000.0 AS new_pos
                FROM playlist_tracks
                WHERE playlist_id = $1
            ) sub
            WHERE pt.id = sub.id
            "#,
            playlist_id
        )
        .execute(executor)
        .await?;

        Ok(())
    }

    /// Verifies that a playlist belongs to a user (for ownership checks).
    pub async fn belongs_to_user(
        executor: impl PgExecutor<'_>,
        playlist_id: Uuid,
        user_id: Uuid,
    ) -> AppResult<bool> {
        let row = sqlx::query!(
            r#"
            SELECT 1 AS exists
            FROM playlists
            WHERE id = $1 AND user_id = $2
            "#,
            playlist_id,
            user_id
        )
        .fetch_optional(executor)
        .await?;

        Ok(row.is_some())
    }
}
