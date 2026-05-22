use sqlx::types::Json;
use sqlx::{PgExecutor, PgPool};
use uuid::Uuid;

use crate::db::entities::Playlist;
use crate::error::{AppError, AppResult};
use crate::filters::FilterNode;
use crate::repositories::playlist_repository::PlaylistStats;

pub struct PinnedPlaylistRepository;

/// Maximum number of playlists a single user can pin to their home screen.
pub const MAX_PINNED_PLAYLISTS: i64 = 6;

impl PinnedPlaylistRepository {
    /// Returns the pinned playlists for `user_id` joined with playlist
    /// metadata + aggregate stats, ordered by user-defined position.
    pub async fn list_for_user(
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
                p.type AS "playlist_type!",
                p.filter_definition AS "filter_definition: Json<FilterNode>",
                p.cached_track_count,
                p.cached_total_duration_ms,
                CASE WHEN p.type = 'dynamic' THEN
                    p.cached_track_count::bigint
                ELSE
                    COUNT(pt.id)
                END AS "track_count!: i64",
                CASE WHEN p.type = 'dynamic' THEN
                    p.cached_total_duration_ms
                ELSE
                    COALESCE(SUM(t.duration_ms), 0)::bigint
                END AS "total_duration_ms!: i64",
                pp.position AS "position!"
            FROM pinned_playlists pp
            JOIN playlists p ON p.id = pp.playlist_id
            LEFT JOIN playlist_tracks pt ON pt.playlist_id = p.id
            LEFT JOIN tracks t ON t.id = pt.track_id
            WHERE pp.user_id = $1
            GROUP BY p.id, pp.position
            ORDER BY pp.position ASC
            "#,
            user_id,
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
                    playlist_type: r.playlist_type,
                    filter_definition: r.filter_definition,
                    cached_track_count: r.cached_track_count,
                    cached_total_duration_ms: r.cached_total_duration_ms,
                },
                track_count: r.track_count,
                total_duration_ms: r.total_duration_ms,
            })
            .collect())
    }

    pub async fn count_for_user(executor: impl PgExecutor<'_>, user_id: Uuid) -> AppResult<i64> {
        let row = sqlx::query!(
            r#"SELECT COUNT(*) AS "n!" FROM pinned_playlists WHERE user_id = $1"#,
            user_id,
        )
        .fetch_one(executor)
        .await?;
        Ok(row.n)
    }

    /// Pins `playlist_id` for `user_id` at the end (max(position) + 1).
    /// Returns `false` if the playlist is already pinned (no-op).
    pub async fn insert(
        executor: impl PgExecutor<'_>,
        user_id: Uuid,
        playlist_id: Uuid,
    ) -> AppResult<bool> {
        let result = sqlx::query!(
            r#"
            INSERT INTO pinned_playlists (user_id, playlist_id, position)
            VALUES (
                $1,
                $2,
                COALESCE((SELECT MAX(position) FROM pinned_playlists WHERE user_id = $1), -1) + 1
            )
            ON CONFLICT (user_id, playlist_id) DO NOTHING
            "#,
            user_id,
            playlist_id,
        )
        .execute(executor)
        .await?;
        Ok(result.rows_affected() > 0)
    }

    /// Unpins `playlist_id` for `user_id`. Returns `true` if a row was deleted.
    /// Does not compact the remaining positions — reorder handles that.
    pub async fn delete(
        executor: impl PgExecutor<'_>,
        user_id: Uuid,
        playlist_id: Uuid,
    ) -> AppResult<bool> {
        let result = sqlx::query!(
            r#"DELETE FROM pinned_playlists WHERE user_id = $1 AND playlist_id = $2"#,
            user_id,
            playlist_id,
        )
        .execute(executor)
        .await?;
        Ok(result.rows_affected() > 0)
    }

    /// Replaces the position of every pinned playlist for `user_id` according
    /// to `ordered_ids` (index in the slice becomes the new position). The
    /// provided list must be exactly the current set of pinned playlist ids —
    /// any mismatch returns `BadRequest`.
    pub async fn reorder(pool: &PgPool, user_id: Uuid, ordered_ids: &[Uuid]) -> AppResult<()> {
        let mut tx = pool.begin().await?;

        let current: Vec<Uuid> = sqlx::query_scalar!(
            r#"SELECT playlist_id FROM pinned_playlists WHERE user_id = $1"#,
            user_id,
        )
        .fetch_all(&mut *tx)
        .await?;

        if current.len() != ordered_ids.len() || !current.iter().all(|id| ordered_ids.contains(id))
        {
            return Err(AppError::BadRequest(
                "reorder list must contain exactly the currently pinned playlists".into(),
            ));
        }

        for (idx, playlist_id) in ordered_ids.iter().enumerate() {
            sqlx::query!(
                r#"UPDATE pinned_playlists SET position = $3 WHERE user_id = $1 AND playlist_id = $2"#,
                user_id,
                playlist_id,
                idx as i32,
            )
            .execute(&mut *tx)
            .await?;
        }

        tx.commit().await?;
        Ok(())
    }
}
