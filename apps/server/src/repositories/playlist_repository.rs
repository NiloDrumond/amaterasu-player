use std::str::FromStr;

use sqlx::types::Json;
use sqlx::{PgExecutor, PgPool, Postgres, QueryBuilder};
use uuid::Uuid;

use crate::db::entities::{Playlist, PlaylistTrack};
use crate::dto::request::SortDir;
use crate::error::{AppError, AppResult};
use crate::filters::{compile_tracks_filter, FilterNode};

pub struct PlaylistRepository;

#[derive(Debug, Clone, Copy)]
pub enum PlaylistSortKey {
    Name,
    Type,
    TrackCount,
    Duration,
    CreatedAt,
}

impl FromStr for PlaylistSortKey {
    type Err = AppError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "name" => Ok(Self::Name),
            "type" => Ok(Self::Type),
            "trackCount" => Ok(Self::TrackCount),
            "duration" => Ok(Self::Duration),
            "createdAt" => Ok(Self::CreatedAt),
            other => Err(AppError::BadRequest(format!("invalid sort key: {other}"))),
        }
    }
}

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
        filter_definition: Option<&FilterNode>,
    ) -> AppResult<Playlist> {
        let playlist_type = if filter_definition.is_some() {
            "dynamic"
        } else {
            "manual"
        };
        let filter_json = filter_definition
            .map(serde_json::to_value)
            .transpose()
            .map_err(|e| crate::error::AppError::BadRequest(format!("invalid filter: {e}")))?;
        let playlist = sqlx::query_as!(
            Playlist,
            r#"
            INSERT INTO playlists (user_id, name, type, filter_definition)
                VALUES ($1, $2, $3, $4)
            RETURNING
                id, user_id, name, created_at, updated_at, type AS "playlist_type", filter_definition AS "filter_definition: Json<FilterNode>", cached_track_count, cached_total_duration_ms
            "#,
            user_id,
            name,
            playlist_type,
            filter_json,
        )
        .fetch_one(executor)
        .await?;

        Ok(playlist)
    }

    pub async fn find_by_ids_for_user(
        executor: impl PgExecutor<'_>,
        user_id: Uuid,
        ids: &[Uuid],
    ) -> AppResult<Vec<PlaylistStats>> {
        if ids.is_empty() {
            return Ok(Vec::new());
        }
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
                END AS "total_duration_ms!: i64"
            FROM
                playlists p
                LEFT JOIN playlist_tracks pt ON pt.playlist_id = p.id
                LEFT JOIN tracks t ON t.id = pt.track_id
            WHERE
                p.id = ANY($1)
                AND p.user_id = $2
            GROUP BY
                p.id
            "#,
            ids,
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
                END AS "total_duration_ms!: i64"
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
                playlist_type: r.playlist_type,
                filter_definition: r.filter_definition,
                cached_track_count: r.cached_track_count,
                cached_total_duration_ms: r.cached_total_duration_ms,
            },
            track_count: r.track_count,
            total_duration_ms: r.total_duration_ms,
        }))
    }

    pub async fn list_by_user_with_query(
        pool: &PgPool,
        user_id: Uuid,
        query: Option<&str>,
        limit: i32,
        offset: i32,
        sort: Option<PlaylistSortKey>,
        dir: Option<SortDir>,
    ) -> AppResult<Vec<PlaylistStats>> {
        use sqlx::Row;

        let pattern = query.map(|q| {
            format!(
                "%{}%",
                q.replace('\\', "\\\\")
                    .replace('%', "\\%")
                    .replace('_', "\\_")
            )
        });

        let mut qb: QueryBuilder<Postgres> = QueryBuilder::new(
            "SELECT \
                p.id, p.user_id, p.name, p.created_at, p.updated_at, \
                p.type AS playlist_type, \
                p.filter_definition, \
                p.cached_track_count, p.cached_total_duration_ms, \
                CASE WHEN p.type = 'dynamic' THEN p.cached_track_count::bigint \
                     ELSE COUNT(pt.id) END AS track_count, \
                CASE WHEN p.type = 'dynamic' THEN p.cached_total_duration_ms \
                     ELSE COALESCE(SUM(t.duration_ms), 0)::bigint END AS total_duration_ms \
             FROM playlists p \
             LEFT JOIN playlist_tracks pt ON pt.playlist_id = p.id \
             LEFT JOIN tracks t ON t.id = pt.track_id \
             WHERE p.user_id = ",
        );
        qb.push_bind(user_id);
        qb.push(" AND (");
        qb.push_bind(pattern.clone());
        qb.push("::text IS NULL OR p.name ILIKE ");
        qb.push_bind(pattern);
        qb.push(") GROUP BY p.id");

        let d = dir.unwrap_or(SortDir::Asc).as_sql();
        match sort {
            None => qb.push(" ORDER BY p.created_at DESC, p.id"),
            Some(PlaylistSortKey::Name) => {
                qb.push(format!(" ORDER BY p.name {d}, p.id"))
            }
            Some(PlaylistSortKey::Type) => {
                qb.push(format!(" ORDER BY p.type {d}, p.name, p.id"))
            }
            Some(PlaylistSortKey::TrackCount) => qb.push(format!(
                " ORDER BY (CASE WHEN p.type = 'dynamic' THEN p.cached_track_count::bigint ELSE COUNT(pt.id) END) {d} NULLS LAST, p.id"
            )),
            Some(PlaylistSortKey::Duration) => qb.push(format!(
                " ORDER BY (CASE WHEN p.type = 'dynamic' THEN p.cached_total_duration_ms ELSE COALESCE(SUM(t.duration_ms), 0)::bigint END) {d} NULLS LAST, p.id"
            )),
            Some(PlaylistSortKey::CreatedAt) => {
                qb.push(format!(" ORDER BY p.created_at {d}, p.id"))
            }
        };
        qb.push(" LIMIT ").push_bind(limit as i64);
        qb.push(" OFFSET ").push_bind(offset as i64);

        let rows = qb.build().fetch_all(pool).await?;

        let mut out = Vec::with_capacity(rows.len());
        for r in rows {
            out.push(PlaylistStats {
                playlist: Playlist {
                    id: r.try_get("id")?,
                    user_id: r.try_get("user_id")?,
                    name: r.try_get("name")?,
                    created_at: r.try_get("created_at")?,
                    updated_at: r.try_get("updated_at")?,
                    playlist_type: r.try_get("playlist_type")?,
                    filter_definition: r
                        .try_get::<Option<Json<FilterNode>>, _>("filter_definition")?,
                    cached_track_count: r.try_get("cached_track_count")?,
                    cached_total_duration_ms: r.try_get("cached_total_duration_ms")?,
                },
                track_count: r.try_get("track_count")?,
                total_duration_ms: r.try_get("total_duration_ms")?,
            });
        }
        Ok(out)
    }

    pub async fn count_by_user_with_query(
        pool: &PgPool,
        user_id: Uuid,
        query: Option<&str>,
    ) -> AppResult<i64> {
        let pattern = query.map(|q| {
            format!(
                "%{}%",
                q.replace('\\', "\\\\")
                    .replace('%', "\\%")
                    .replace('_', "\\_")
            )
        });
        let row = sqlx::query!(
            r#"
            SELECT COUNT(*) AS "count!"
            FROM playlists p
            WHERE p.user_id = $1
              AND ($2::text IS NULL OR p.name ILIKE $2)
            "#,
            user_id,
            pattern,
        )
        .fetch_one(pool)
        .await?;
        Ok(row.count)
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
            UPDATE
                playlists
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
                updated_at,
                type AS "playlist_type",
                filter_definition AS "filter_definition: Json<FilterNode>",
                cached_track_count,
                cached_total_duration_ms
            "#,
            id,
            user_id,
            name
        )
        .fetch_optional(executor)
        .await?;

        Ok(playlist)
    }

    /// Updates the filter definition of a dynamic playlist.
    pub async fn update_filter(
        executor: impl PgExecutor<'_>,
        id: Uuid,
        user_id: Uuid,
        filter_definition: &FilterNode,
    ) -> AppResult<Option<Playlist>> {
        let filter_json = serde_json::to_value(filter_definition)
            .map_err(|e| crate::error::AppError::BadRequest(format!("invalid filter: {e}")))?;
        let playlist = sqlx::query_as!(
            Playlist,
            r#"
            UPDATE
                playlists
            SET
                filter_definition = $3,
                updated_at = NOW()
            WHERE
                id = $1
                AND user_id = $2
                AND type = 'dynamic'
            RETURNING
                id,
                user_id,
                name,
                created_at,
                updated_at,
                type AS "playlist_type",
                filter_definition AS "filter_definition: Json<FilterNode>",
                cached_track_count,
                cached_total_duration_ms
            "#,
            id,
            user_id,
            filter_json,
        )
        .fetch_optional(executor)
        .await?;

        Ok(playlist)
    }

    /// Resolves a dynamic playlist's filter to a list of `PlaylistTrackRow`s.
    /// Position is the row index, playlist_track_id is the synthetic UUID nil
    /// (dynamic playlists have no real `playlist_tracks` rows).
    pub async fn list_dynamic_tracks(
        pool: &PgPool,
        playlist_id: Uuid,
        filter: &FilterNode,
    ) -> AppResult<Vec<PlaylistTrackRow>> {
        let mut qb: QueryBuilder<Postgres> = QueryBuilder::new(
            r#"
            SELECT
                tracks.id           AS track_id,
                tracks.title        AS title,
                tracks.sort_title   AS sort_title,
                tracks.artist_id,
                tracks.album_id,
                tracks.track_no,
                tracks.disc,
                tracks.duration_ms,
                tracks.format,
                tracks.codec,
                tracks.bitrate,
                tracks.file_path,
                tracks.original_title,
                tracks.original_artist,
                tracks.original_album,
                ar.name             AS artist_name,
                al.title            AS album_title,
                al.cover_path       AS album_cover_path
            FROM tracks
            LEFT JOIN artists ar ON ar.id = tracks.artist_id
            LEFT JOIN albums al ON al.id = tracks.album_id
            WHERE tracks.deleted_at IS NULL AND "#,
        );
        compile_tracks_filter(&mut qb, filter)
            .map_err(|e| crate::error::AppError::BadRequest(e.to_string()))?;
        qb.push(" ORDER BY tracks.disc NULLS LAST, tracks.track_no NULLS LAST, tracks.title");

        let rows = qb.build().fetch_all(pool).await?;

        // Write the resolved aggregates back to the playlist so the listing
        // page can show accurate counts without re-running the filter.
        {
            use sqlx::Row;
            let total_duration_ms: i64 = rows
                .iter()
                .map(|r| r.try_get::<i32, _>("duration_ms").map(i64::from))
                .collect::<Result<Vec<_>, _>>()?
                .into_iter()
                .sum();
            sqlx::query!(
                r#"
                UPDATE
                    playlists
                SET
                    cached_track_count = $1,
                    cached_total_duration_ms = $2
                WHERE
                    id = $3
                "#,
                rows.len() as i32,
                total_duration_ms,
                playlist_id,
            )
            .execute(pool)
            .await?;
        }

        let mut out = Vec::with_capacity(rows.len());
        for (idx, r) in rows.iter().enumerate() {
            use sqlx::Row;
            out.push(PlaylistTrackRow {
                playlist_track_id: Uuid::nil(),
                position: (idx + 1) as f64,
                added_at: chrono::Utc::now(),
                track_id: r.try_get("track_id")?,
                title: r.try_get("title")?,
                sort_title: r.try_get("sort_title")?,
                artist_id: r.try_get("artist_id")?,
                album_id: r.try_get("album_id")?,
                track_no: r.try_get("track_no")?,
                disc: r.try_get("disc")?,
                duration_ms: r.try_get("duration_ms")?,
                format: r.try_get("format")?,
                codec: r.try_get("codec")?,
                bitrate: r.try_get("bitrate")?,
                file_path: r.try_get("file_path")?,
                original_title: r.try_get("original_title")?,
                original_artist: r.try_get("original_artist")?,
                original_album: r.try_get("original_album")?,
                artist_name: r.try_get("artist_name")?,
                album_title: r.try_get("album_title")?,
                album_cover_path: r.try_get("album_cover_path")?,
            });
        }
        Ok(out)
    }

    pub async fn delete(executor: impl PgExecutor<'_>, id: Uuid, user_id: Uuid) -> AppResult<bool> {
        let result = sqlx::query!(
            r#"
            DELETE FROM playlists
            WHERE id = $1
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
            SELECT
                MAX(position) AS max_pos
            FROM
                playlist_tracks
            WHERE
                playlist_id = $1
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
            ON CONFLICT (playlist_id, track_id)
                DO NOTHING
            RETURNING
                id, playlist_id, track_id, position, added_at
            "#,
            playlist_id,
            track_id,
            position
        )
        .fetch_optional(executor)
        .await?;

        Ok(())
    }

    /// Bumps `updated_at` on a playlist so it surfaces in "recently active"
    /// lists when tracks are added, removed, or reordered.
    pub async fn touch(executor: impl PgExecutor<'_>, playlist_id: Uuid) -> AppResult<()> {
        sqlx::query!(
            r#"UPDATE playlists SET updated_at = NOW() WHERE id = $1"#,
            playlist_id
        )
        .execute(executor)
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
            DELETE FROM playlist_tracks pt USING playlists p
            WHERE pt.playlist_id = p.id
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
                pt.id AS "playlist_track_id!",
                pt.position AS "position!",
                pt.added_at AS "added_at!",
                t.id AS "track_id!",
                t.title AS "title!",
                t.sort_title AS "sort_title!",
                t.artist_id,
                t.album_id,
                t.track_no,
                t.disc,
                t.duration_ms AS "duration_ms!",
                t.format AS "format!",
                t.codec AS "codec!",
                t.bitrate,
                t.file_path AS "file_path!",
                t.original_title,
                t.original_artist,
                t.original_album,
                ar.name AS "artist_name?",
                al.title AS "album_title?",
                al.cover_path AS "album_cover_path?"
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
                    SELECT
                        MIN(position) AS min_pos
                    FROM
                        playlist_tracks
                    WHERE
                        playlist_id = $1
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
                    SELECT
                        position
                    FROM
                        playlist_tracks
                    WHERE
                        playlist_id = $1
                        AND track_id = $2
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
                    SELECT
                        position
                    FROM
                        playlist_tracks
                    WHERE
                        playlist_id = $1
                        AND position > $2
                    ORDER BY
                        position ASC
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
            UPDATE
                playlist_tracks
            SET
                position = $3
            WHERE
                playlist_id = $1
                AND track_id = $2
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
            UPDATE
                playlist_tracks pt
            SET
                position = sub.new_pos
            FROM (
                SELECT
                    id,
                    ROW_NUMBER() OVER (ORDER BY position ASC) * 1000.0 AS new_pos
                FROM
                    playlist_tracks
                WHERE
                    playlist_id = $1) sub
            WHERE
                pt.id = sub.id
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
            SELECT
                1 AS exists
            FROM
                playlists
            WHERE
                id = $1
                AND user_id = $2
            "#,
            playlist_id,
            user_id
        )
        .fetch_optional(executor)
        .await?;

        Ok(row.is_some())
    }
}
