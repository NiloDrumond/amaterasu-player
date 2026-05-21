use chrono::{DateTime, Utc};
use sqlx::PgExecutor;
use uuid::Uuid;

use crate::error::AppError;

pub struct AliasRepository;

#[derive(Debug, Clone)]
pub struct ArtistAliasRow {
    pub id: Uuid,
    pub source_name: String,
    pub artist_id: Uuid,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct AlbumAliasRow {
    pub id: Uuid,
    pub source_title: String,
    pub source_album_artist_id: Option<Uuid>,
    pub album_id: Uuid,
    pub created_at: DateTime<Utc>,
}

impl AliasRepository {
    /// Repoint every existing artist_aliases row from `from_id` to `to_id`.
    pub async fn repoint_artist_aliases(
        executor: impl PgExecutor<'_>,
        from_id: Uuid,
        to_id: Uuid,
    ) -> Result<(), AppError> {
        sqlx::query!(
            r#"UPDATE artist_aliases SET artist_id = $2 WHERE artist_id = $1"#,
            from_id,
            to_id,
        )
        .execute(executor)
        .await?;
        Ok(())
    }

    /// Look up the artist_id an absorbed source_name points to.
    pub async fn find_artist_id_by_source_name(
        executor: impl PgExecutor<'_>,
        source_name: &str,
    ) -> Result<Option<Uuid>, AppError> {
        let row = sqlx::query!(
            r#"
            SELECT artist_id
            FROM artist_aliases
            WHERE LOWER(source_name) = LOWER($1)
            "#,
            source_name,
        )
        .fetch_optional(executor)
        .await?;
        Ok(row.map(|r| r.artist_id))
    }

    /// Repoint every existing album_aliases row from `from_id` to `to_id`.
    pub async fn repoint_album_aliases(
        executor: impl PgExecutor<'_>,
        from_id: Uuid,
        to_id: Uuid,
    ) -> Result<(), AppError> {
        sqlx::query!(
            r#"UPDATE album_aliases SET album_id = $2 WHERE album_id = $1"#,
            from_id,
            to_id,
        )
        .execute(executor)
        .await?;
        Ok(())
    }

    /// Rewrite source_album_artist_id on existing aliases when an artist is
    /// merged away (so album lookups via alias continue to find the right row).
    pub async fn repoint_album_alias_artist(
        executor: impl PgExecutor<'_>,
        from_id: Uuid,
        to_id: Uuid,
    ) -> Result<(), AppError> {
        sqlx::query!(
            r#"UPDATE album_aliases SET source_album_artist_id = $2 WHERE source_album_artist_id = $1"#,
            from_id,
            to_id,
        )
        .execute(executor)
        .await?;
        Ok(())
    }

    pub async fn find_album_id_by_source_keys(
        executor: impl PgExecutor<'_>,
        source_title: &str,
        source_album_artist_id: Option<Uuid>,
    ) -> Result<Option<Uuid>, AppError> {
        let row = sqlx::query!(
            r#"
            SELECT album_id
            FROM album_aliases
            WHERE LOWER(source_title) = LOWER($1)
                AND (source_album_artist_id = $2
                    OR (source_album_artist_id IS NULL AND $2 IS NULL))
            "#,
            source_title,
            source_album_artist_id,
        )
        .fetch_optional(executor)
        .await?;
        Ok(row.map(|r| r.album_id))
    }

    /// Plain INSERT — caller has already confirmed no existing alias matches.
    pub async fn insert_artist_alias(
        executor: impl PgExecutor<'_>,
        source_name: &str,
        artist_id: Uuid,
    ) -> Result<(), AppError> {
        sqlx::query!(
            r#"INSERT INTO artist_aliases (source_name, artist_id) VALUES ($1, $2)"#,
            source_name,
            artist_id,
        )
        .execute(executor)
        .await?;
        Ok(())
    }

    /// Plain INSERT — caller has already confirmed no existing alias matches.
    pub async fn insert_album_alias(
        executor: impl PgExecutor<'_>,
        source_title: &str,
        source_album_artist_id: Option<Uuid>,
        album_id: Uuid,
    ) -> Result<(), AppError> {
        sqlx::query!(
            r#"INSERT INTO album_aliases (source_title, source_album_artist_id, album_id) VALUES ($1, $2, $3)"#,
            source_title,
            source_album_artist_id,
            album_id,
        )
        .execute(executor)
        .await?;
        Ok(())
    }

    pub async fn find_artist_aliases_by_artist_id(
        executor: impl PgExecutor<'_>,
        artist_id: Uuid,
    ) -> Result<Vec<ArtistAliasRow>, AppError> {
        let rows = sqlx::query_as!(
            ArtistAliasRow,
            r#"
            SELECT id, source_name, artist_id, created_at
            FROM artist_aliases
            WHERE artist_id = $1
            ORDER BY created_at
            "#,
            artist_id,
        )
        .fetch_all(executor)
        .await?;
        Ok(rows)
    }

    pub async fn find_artist_aliases_by_artist_ids(
        executor: impl PgExecutor<'_>,
        artist_ids: &[Uuid],
    ) -> Result<Vec<ArtistAliasRow>, AppError> {
        if artist_ids.is_empty() {
            return Ok(Vec::new());
        }
        let rows = sqlx::query_as!(
            ArtistAliasRow,
            r#"
            SELECT id, source_name, artist_id, created_at
            FROM artist_aliases
            WHERE artist_id = ANY($1)
            ORDER BY artist_id, created_at
            "#,
            artist_ids,
        )
        .fetch_all(executor)
        .await?;
        Ok(rows)
    }

    pub async fn find_album_aliases_by_album_id(
        executor: impl PgExecutor<'_>,
        album_id: Uuid,
    ) -> Result<Vec<AlbumAliasRow>, AppError> {
        let rows = sqlx::query_as!(
            AlbumAliasRow,
            r#"
            SELECT id, source_title, source_album_artist_id, album_id, created_at
            FROM album_aliases
            WHERE album_id = $1
            ORDER BY created_at
            "#,
            album_id,
        )
        .fetch_all(executor)
        .await?;
        Ok(rows)
    }

    pub async fn find_album_aliases_by_album_ids(
        executor: impl PgExecutor<'_>,
        album_ids: &[Uuid],
    ) -> Result<Vec<AlbumAliasRow>, AppError> {
        if album_ids.is_empty() {
            return Ok(Vec::new());
        }
        let rows = sqlx::query_as!(
            AlbumAliasRow,
            r#"
            SELECT id, source_title, source_album_artist_id, album_id, created_at
            FROM album_aliases
            WHERE album_id = ANY($1)
            ORDER BY album_id, created_at
            "#,
            album_ids,
        )
        .fetch_all(executor)
        .await?;
        Ok(rows)
    }

    /// Source titles that exist under both `from_artist_id` and `to_artist_id`
    /// in `album_aliases`. Repointing `source_album_artist_id` from `from` to
    /// `to` would violate the `(source_album_artist_id, lower(source_title))`
    /// partial unique index for these titles, so `merge_artist` rejects with a
    /// clear message and the admin must merge the colliding albums first.
    pub async fn find_album_alias_artist_collisions(
        executor: impl PgExecutor<'_>,
        from_artist_id: Uuid,
        to_artist_id: Uuid,
    ) -> Result<Vec<String>, AppError> {
        let rows = sqlx::query!(
            r#"
            SELECT a.source_title
            FROM album_aliases a
            WHERE a.source_album_artist_id = $1
              AND EXISTS (
                  SELECT 1 FROM album_aliases b
                  WHERE b.source_album_artist_id = $2
                    AND LOWER(b.source_title) = LOWER(a.source_title)
              )
            "#,
            from_artist_id,
            to_artist_id,
        )
        .fetch_all(executor)
        .await?;
        Ok(rows.into_iter().map(|r| r.source_title).collect())
    }
}
