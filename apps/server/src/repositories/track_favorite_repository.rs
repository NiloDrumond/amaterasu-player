use sqlx::PgExecutor;
use uuid::Uuid;

use crate::error::AppResult;

pub struct TrackFavoriteRepository;

impl TrackFavoriteRepository {
    /// Marks a track as favorited by the user. Idempotent: a second call is a
    /// no-op. Returns true if a new row was inserted.
    pub async fn favorite(
        executor: impl PgExecutor<'_>,
        user_id: Uuid,
        track_id: Uuid,
    ) -> AppResult<bool> {
        let result = sqlx::query!(
            r#"
            INSERT INTO track_favorites (user_id, track_id)
            VALUES ($1, $2)
            ON CONFLICT (user_id, track_id) DO NOTHING
            "#,
            user_id,
            track_id,
        )
        .execute(executor)
        .await?;
        Ok(result.rows_affected() > 0)
    }

    /// Removes a track from the user's favorites. Returns true if a row was
    /// deleted.
    pub async fn unfavorite(
        executor: impl PgExecutor<'_>,
        user_id: Uuid,
        track_id: Uuid,
    ) -> AppResult<bool> {
        let result = sqlx::query!(
            r#"
            DELETE FROM track_favorites
            WHERE user_id = $1 AND track_id = $2
            "#,
            user_id,
            track_id,
        )
        .execute(executor)
        .await?;
        Ok(result.rows_affected() > 0)
    }

    /// Returns the subset of `track_ids` that the user has favorited.
    pub async fn favorited_track_ids(
        executor: impl PgExecutor<'_>,
        user_id: Uuid,
        track_ids: &[Uuid],
    ) -> AppResult<Vec<Uuid>> {
        if track_ids.is_empty() {
            return Ok(Vec::new());
        }
        let rows = sqlx::query!(
            r#"
            SELECT track_id
            FROM track_favorites
            WHERE user_id = $1 AND track_id = ANY($2)
            "#,
            user_id,
            track_ids,
        )
        .fetch_all(executor)
        .await?;
        Ok(rows.into_iter().map(|r| r.track_id).collect())
    }
}
