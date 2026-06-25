use chrono::{DateTime, Utc};
use sqlx::PgExecutor;
use uuid::Uuid;

use crate::error::AppResult;

pub struct TrackPlayRepository;

impl TrackPlayRepository {
    pub async fn record(
        executor: impl PgExecutor<'_>,
        user_id: Uuid,
        track_id: Uuid,
        context_album_id: Option<Uuid>,
        context_playlist_id: Option<Uuid>,
    ) -> AppResult<Uuid> {
        let row = sqlx::query!(
            r#"
            INSERT INTO track_plays (user_id, track_id, context_album_id, context_playlist_id)
            VALUES ($1, $2, $3, $4)
            RETURNING id
            "#,
            user_id,
            track_id,
            context_album_id,
            context_playlist_id,
        )
        .fetch_one(executor)
        .await?;
        Ok(row.id)
    }

    pub async fn play_counts_for_tracks(
        executor: impl PgExecutor<'_>,
        user_id: Uuid,
        track_ids: &[Uuid],
    ) -> AppResult<Vec<(Uuid, i64)>> {
        if track_ids.is_empty() {
            return Ok(Vec::new());
        }
        let rows = sqlx::query!(
            r#"
            SELECT
                track_id AS "track_id!",
                COUNT(*) AS "play_count!"
            FROM track_plays
            WHERE user_id = $1 AND track_id = ANY($2)
            GROUP BY track_id
            "#,
            user_id,
            track_ids,
        )
        .fetch_all(executor)
        .await?;
        Ok(rows
            .into_iter()
            .map(|r| (r.track_id, r.play_count))
            .collect())
    }

    pub async fn play_counts_for_albums(
        executor: impl PgExecutor<'_>,
        user_id: Uuid,
        album_ids: &[Uuid],
    ) -> AppResult<Vec<(Uuid, i64)>> {
        if album_ids.is_empty() {
            return Ok(Vec::new());
        }
        let rows = sqlx::query!(
            r#"
            SELECT
                context_album_id AS "album_id!",
                COUNT(*) AS "play_count!"
            FROM track_plays
            WHERE user_id = $1 AND context_album_id = ANY($2)
            GROUP BY context_album_id
            "#,
            user_id,
            album_ids,
        )
        .fetch_all(executor)
        .await?;
        Ok(rows
            .into_iter()
            .map(|r| (r.album_id, r.play_count))
            .collect())
    }

    pub async fn play_counts_for_artists(
        executor: impl PgExecutor<'_>,
        user_id: Uuid,
        artist_ids: &[Uuid],
    ) -> AppResult<Vec<(Uuid, i64)>> {
        if artist_ids.is_empty() {
            return Ok(Vec::new());
        }
        let rows = sqlx::query!(
            r#"
            SELECT
                tracks.artist_id AS "artist_id!",
                COUNT(*) AS "play_count!"
            FROM track_plays
            JOIN tracks ON tracks.id = track_plays.track_id
            WHERE track_plays.user_id = $1
              AND tracks.artist_id = ANY($2)
            GROUP BY tracks.artist_id
            "#,
            user_id,
            artist_ids,
        )
        .fetch_all(executor)
        .await?;
        Ok(rows
            .into_iter()
            .map(|r| (r.artist_id, r.play_count))
            .collect())
    }

    /// Returns playlists owned by `user_id` ordered by the most recent play
    /// (via the playlist as playback context). Playlists never played by the
    /// user are excluded.
    pub async fn recent_playlists_for_user(
        executor: impl PgExecutor<'_>,
        user_id: Uuid,
        limit: i64,
    ) -> AppResult<Vec<(Uuid, String, String)>> {
        use sqlx::{Postgres, QueryBuilder, Row};

        // Built with QueryBuilder (not the checked `query!` macro) so adding the
        // `p.type` column doesn't require regenerating the offline .sqlx cache.
        let mut qb: QueryBuilder<Postgres> = QueryBuilder::new(
            "SELECT p.id AS id, p.name AS name, p.type AS playlist_type \
             FROM playlists p \
             LEFT JOIN track_plays tp \
                 ON tp.context_playlist_id = p.id AND tp.user_id = ",
        );
        qb.push_bind(user_id);
        qb.push(" WHERE p.user_id = ");
        qb.push_bind(user_id);
        qb.push(
            " GROUP BY p.id, p.name, p.type, p.updated_at \
              ORDER BY GREATEST(COALESCE(MAX(tp.played_at), p.updated_at), p.updated_at) DESC \
              LIMIT ",
        );
        qb.push_bind(limit);

        let rows = qb.build().fetch_all(executor).await?;
        let mut out = Vec::with_capacity(rows.len());
        for r in rows {
            out.push((
                r.try_get("id")?,
                r.try_get("name")?,
                r.try_get("playlist_type")?,
            ));
        }
        Ok(out)
    }

    pub async fn play_counts_for_playlists(
        executor: impl PgExecutor<'_>,
        user_id: Uuid,
        playlist_ids: &[Uuid],
    ) -> AppResult<Vec<(Uuid, i64)>> {
        if playlist_ids.is_empty() {
            return Ok(Vec::new());
        }
        let rows = sqlx::query!(
            r#"
            SELECT
                context_playlist_id AS "playlist_id!",
                COUNT(*) AS "play_count!"
            FROM track_plays
            WHERE user_id = $1 AND context_playlist_id = ANY($2)
            GROUP BY context_playlist_id
            "#,
            user_id,
            playlist_ids,
        )
        .fetch_all(executor)
        .await?;
        Ok(rows
            .into_iter()
            .map(|r| (r.playlist_id, r.play_count))
            .collect())
    }

    /// Album IDs the user has played, ordered by most recent play. Returns the
    /// timestamp of the latest play alongside each id so callers can merge
    /// album and playlist recency lists.
    pub async fn recent_album_ids_with_time(
        executor: impl PgExecutor<'_>,
        user_id: Uuid,
        limit: i64,
    ) -> AppResult<Vec<(Uuid, DateTime<Utc>)>> {
        let rows = sqlx::query!(
            r#"
            SELECT context_album_id AS "album_id!", MAX(played_at) AS "last_played!"
            FROM track_plays
            WHERE user_id = $1 AND context_album_id IS NOT NULL
            GROUP BY context_album_id
            ORDER BY MAX(played_at) DESC
            LIMIT $2
            "#,
            user_id,
            limit,
        )
        .fetch_all(executor)
        .await?;
        Ok(rows
            .into_iter()
            .map(|r| (r.album_id, r.last_played))
            .collect())
    }

    /// Playlist IDs the user has played, ordered by most recent play. Like
    /// `recent_album_ids_with_time`, returns timestamps for cross-source merging.
    pub async fn recent_playlist_ids_with_time(
        executor: impl PgExecutor<'_>,
        user_id: Uuid,
        limit: i64,
    ) -> AppResult<Vec<(Uuid, DateTime<Utc>)>> {
        let rows = sqlx::query!(
            r#"
            SELECT context_playlist_id AS "playlist_id!", MAX(played_at) AS "last_played!"
            FROM track_plays
            WHERE user_id = $1 AND context_playlist_id IS NOT NULL
            GROUP BY context_playlist_id
            ORDER BY MAX(played_at) DESC
            LIMIT $2
            "#,
            user_id,
            limit,
        )
        .fetch_all(executor)
        .await?;
        Ok(rows
            .into_iter()
            .map(|r| (r.playlist_id, r.last_played))
            .collect())
    }

    /// Albums the user used to play heavily (>= 3 lifetime plays from this
    /// context) but hasn't touched in the last 30 days. Returned with total
    /// play count so callers can merge with playlists by historical popularity.
    pub async fn forgotten_album_ids_with_plays(
        executor: impl PgExecutor<'_>,
        user_id: Uuid,
        limit: i64,
    ) -> AppResult<Vec<(Uuid, i64)>> {
        let rows = sqlx::query!(
            r#"
            SELECT context_album_id AS "album_id!", COUNT(*) AS "plays!"
            FROM track_plays
            WHERE user_id = $1 AND context_album_id IS NOT NULL
            GROUP BY context_album_id
            HAVING COUNT(*) >= 3 AND MAX(played_at) < NOW() - INTERVAL '30 days'
            ORDER BY COUNT(*) DESC
            LIMIT $2
            "#,
            user_id,
            limit,
        )
        .fetch_all(executor)
        .await?;
        Ok(rows.into_iter().map(|r| (r.album_id, r.plays)).collect())
    }

    pub async fn forgotten_playlist_ids_with_plays(
        executor: impl PgExecutor<'_>,
        user_id: Uuid,
        limit: i64,
    ) -> AppResult<Vec<(Uuid, i64)>> {
        let rows = sqlx::query!(
            r#"
            SELECT context_playlist_id AS "playlist_id!", COUNT(*) AS "plays!"
            FROM track_plays
            WHERE user_id = $1 AND context_playlist_id IS NOT NULL
            GROUP BY context_playlist_id
            HAVING COUNT(*) >= 3 AND MAX(played_at) < NOW() - INTERVAL '30 days'
            ORDER BY COUNT(*) DESC
            LIMIT $2
            "#,
            user_id,
            limit,
        )
        .fetch_all(executor)
        .await?;
        Ok(rows.into_iter().map(|r| (r.playlist_id, r.plays)).collect())
    }
}
