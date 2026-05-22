use std::str::FromStr;

use sqlx::{PgExecutor, PgPool, Postgres, QueryBuilder};
use uuid::Uuid;

use crate::db::entities::Artist;
use crate::dto::request::SortDir;
use crate::error::AppError;

pub struct ArtistRepository;

#[derive(Debug, Clone, Copy)]
pub enum ArtistSortKey {
    Name,
    AlbumCount,
    TrackCount,
}

impl FromStr for ArtistSortKey {
    type Err = AppError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "name" => Ok(Self::Name),
            "albumCount" => Ok(Self::AlbumCount),
            "trackCount" => Ok(Self::TrackCount),
            other => Err(AppError::BadRequest(format!("invalid sort key: {other}"))),
        }
    }
}

impl ArtistRepository {
    pub async fn create(
        executor: impl PgExecutor<'_>,
        artist: &Artist,
    ) -> Result<Artist, AppError> {
        let created = sqlx::query_as!(
            Artist,
            r#"
            INSERT INTO artists (id, name, sort_name, mbid, locked_at, created_at, updated_at)
                VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING
                *
            "#,
            artist.id,
            artist.name,
            artist.sort_name,
            artist.mbid,
            artist.locked_at,
            artist.created_at,
            artist.updated_at
        )
        .fetch_one(executor)
        .await?;

        Ok(created)
    }

    pub async fn find_by_id(
        executor: impl PgExecutor<'_>,
        id: Uuid,
    ) -> Result<Option<Artist>, AppError> {
        let artist = sqlx::query_as!(
            Artist,
            r#"
            SELECT
                *
            FROM
                artists
            WHERE
                id = $1
            "#,
            id
        )
        .fetch_optional(executor)
        .await?;

        Ok(artist)
    }

    pub async fn find_by_ids(
        executor: impl PgExecutor<'_>,
        ids: &[Uuid],
    ) -> Result<Vec<Artist>, AppError> {
        if ids.is_empty() {
            return Ok(Vec::new());
        }

        let artists = sqlx::query_as!(
            Artist,
            r#"
            SELECT
                *
            FROM
                artists
            WHERE
                id = ANY ($1)
            "#,
            ids
        )
        .fetch_all(executor)
        .await?;

        Ok(artists)
    }

    pub async fn search(
        executor: impl PgExecutor<'_>,
        query: &str,
        limit: i32,
    ) -> Result<Vec<Artist>, AppError> {
        let pattern = format!("%{}%", query.replace('%', "\\%").replace('_', "\\_"));
        let artists = sqlx::query_as!(
            Artist,
            r#"
            SELECT
                *
            FROM
                artists
            WHERE
                name ILIKE $1
            ORDER BY
                sort_name,
                name
            LIMIT $2
            "#,
            pattern,
            limit as i64,
        )
        .fetch_all(executor)
        .await?;

        Ok(artists)
    }

    pub async fn find_all_with_query(
        pool: &PgPool,
        query: Option<&str>,
        limit: i32,
        offset: i32,
        sort: Option<ArtistSortKey>,
        dir: Option<SortDir>,
    ) -> Result<Vec<Artist>, AppError> {
        let pattern = query.map(|q| {
            format!(
                "%{}%",
                q.replace('\\', "\\\\")
                    .replace('%', "\\%")
                    .replace('_', "\\_")
            )
        });
        let mut qb: QueryBuilder<Postgres> = QueryBuilder::new(
            "SELECT artists.* FROM artists \
             LEFT JOIN ( \
               SELECT artist_id, COUNT(DISTINCT album_id) AS album_count, COUNT(*) AS track_count \
               FROM tracks WHERE deleted_at IS NULL AND artist_id IS NOT NULL \
               GROUP BY artist_id \
             ) agg ON agg.artist_id = artists.id \
             WHERE (",
        );
        qb.push_bind(pattern.clone());
        qb.push("::text IS NULL OR artists.name ILIKE ");
        qb.push_bind(pattern);
        qb.push(")");

        let d = dir.unwrap_or(SortDir::Asc).as_sql();
        match sort {
            None => {
                qb.push(" ORDER BY artists.sort_name, artists.name, artists.id");
            }
            Some(ArtistSortKey::Name) => {
                qb.push(format!(
                    " ORDER BY artists.sort_name {d}, artists.name {d}, artists.id"
                ));
            }
            Some(ArtistSortKey::AlbumCount) => {
                qb.push(format!(
                    " ORDER BY agg.album_count {d} NULLS LAST, artists.sort_name, artists.id"
                ));
            }
            Some(ArtistSortKey::TrackCount) => {
                qb.push(format!(
                    " ORDER BY agg.track_count {d} NULLS LAST, artists.sort_name, artists.id"
                ));
            }
        };
        qb.push(" LIMIT ").push_bind(limit as i64);
        qb.push(" OFFSET ").push_bind(offset as i64);

        let artists = qb.build_query_as::<Artist>().fetch_all(pool).await?;
        Ok(artists)
    }

    pub async fn count_with_query(
        executor: impl PgExecutor<'_>,
        query: Option<&str>,
    ) -> Result<i64, AppError> {
        let pattern = query.map(|q| {
            format!(
                "%{}%",
                q.replace('\\', "\\\\")
                    .replace('%', "\\%")
                    .replace('_', "\\_")
            )
        });
        let record = sqlx::query!(
            r#"
            SELECT
                COUNT(*) AS count
            FROM
                artists
            WHERE
                $1::text IS NULL
                OR name ILIKE $1
            "#,
            pattern,
        )
        .fetch_one(executor)
        .await?;
        Ok(record.count.unwrap_or(0))
    }

    pub async fn update(
        executor: impl PgExecutor<'_>,
        id: Uuid,
        name: &str,
        sort_name: &str,
    ) -> Result<Artist, AppError> {
        let updated = sqlx::query_as!(
            Artist,
            r#"
            UPDATE
                artists
            SET
                name = $2,
                sort_name = $3,
                locked_at = NOW(),
                updated_at = NOW()
            WHERE
                id = $1
            RETURNING
                *
            "#,
            id,
            name,
            sort_name,
        )
        .fetch_one(executor)
        .await?;

        Ok(updated)
    }

    pub async fn find_by_mbid(
        executor: impl PgExecutor<'_>,
        mbid: &str,
    ) -> Result<Option<Artist>, AppError> {
        let artist = sqlx::query_as!(Artist, r#"SELECT * FROM artists WHERE mbid = $1"#, mbid,)
            .fetch_optional(executor)
            .await?;
        Ok(artist)
    }

    /// Stamps the MusicBrainz artist MBID without touching anything else.
    /// Used after the admin accepts a MB suggestion.
    pub async fn set_mbid(
        executor: impl PgExecutor<'_>,
        id: Uuid,
        mbid: &str,
    ) -> Result<(), AppError> {
        sqlx::query!(
            r#"UPDATE artists SET mbid = $2, updated_at = NOW() WHERE id = $1"#,
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
    artists
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

    /// Repoint `albums.artist_id` from `from_id` to `to_id`. Scan-side identity
    /// lives in `album_aliases` now and is repointed separately by
    /// [`AliasRepository::repoint_album_alias_artist`].
    pub async fn reassign_albums_artist(
        executor: impl PgExecutor<'_>,
        from_id: Uuid,
        to_id: Uuid,
    ) -> Result<(), AppError> {
        sqlx::query!(
            r#"
            UPDATE albums
            SET artist_id = $2,
                updated_at = NOW()
            WHERE artist_id = $1
            "#,
            from_id,
            to_id,
        )
        .execute(executor)
        .await?;
        Ok(())
    }

    /// Unconditional hard-delete. Caller must ensure no FK referrers remain.
    pub async fn delete(executor: impl PgExecutor<'_>, id: Uuid) -> Result<(), AppError> {
        sqlx::query!(r#"DELETE FROM artists WHERE id = $1"#, id)
            .execute(executor)
            .await?;
        Ok(())
    }

    /// Hard-deletes the artist only if no album or non-deleted track references
    /// it via `artist_id`. Aliases CASCADE; the scan-once invariant in
    /// `scanner/persist.rs` keeps existing tracks from being pulled back onto
    /// a recreated artist on the next scan.
    pub async fn delete_if_empty(
        executor: impl PgExecutor<'_>,
        id: Uuid,
    ) -> Result<bool, AppError> {
        let result = sqlx::query!(
            r#"
            DELETE FROM artists
            WHERE id = $1
                AND NOT EXISTS (
                    SELECT 1 FROM albums WHERE artist_id = $1)
                AND NOT EXISTS (
                    SELECT 1 FROM tracks
                    WHERE artist_id = $1 AND deleted_at IS NULL)
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
    ) -> Result<Option<Artist>, AppError> {
        let updated = sqlx::query_as!(
            Artist,
            r#"
            UPDATE artists
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
        let row = sqlx::query!(r#"SELECT COUNT(*) AS "n!" FROM artists WHERE approved = FALSE"#)
            .fetch_one(executor)
            .await?;
        Ok(row.n)
    }

    /// Pending artists that aren't already represented by an album in the
    /// caller-provided list of album IDs (the review queue's current page).
    /// Capped at a reasonable upper bound; no pagination.
    pub async fn find_pending_excluding_album_artists(
        executor: impl PgExecutor<'_>,
        excluded_album_ids: &[Uuid],
        limit: i64,
    ) -> Result<Vec<Artist>, AppError> {
        let artists = sqlx::query_as!(
            Artist,
            r#"
            SELECT *
            FROM artists ar
            WHERE ar.approved = FALSE
              AND NOT EXISTS (
                  SELECT 1 FROM albums al
                  WHERE al.id = ANY($1)
                    AND al.artist_id = ar.id
              )
            ORDER BY ar.created_at DESC
            LIMIT $2
            "#,
            excluded_album_ids,
            limit,
        )
        .fetch_all(executor)
        .await?;
        Ok(artists)
    }
}
