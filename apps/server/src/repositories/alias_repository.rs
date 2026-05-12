use sqlx::PgExecutor;
use uuid::Uuid;

use crate::error::AppError;

pub struct AliasRepository;

impl AliasRepository {
    /// Upsert a (source_name → artist_id) alias. If an alias already exists for
    /// the source_name (case-insensitive), it's rewritten to point at the new
    /// target — covers transitive merges (A→B, then B→C ⇒ A's alias now → C).
    pub async fn upsert_artist_alias(
        executor: impl PgExecutor<'_>,
        source_name: &str,
        artist_id: Uuid,
    ) -> Result<(), AppError> {
        sqlx::query!(
            r#"
            INSERT INTO artist_aliases (source_name, artist_id)
                VALUES ($1, $2)
            ON CONFLICT (LOWER(source_name))
                DO UPDATE SET
                    artist_id = EXCLUDED.artist_id
            "#,
            source_name,
            artist_id,
        )
        .execute(executor)
        .await?;
        Ok(())
    }

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

    /// Upsert a ((source_title, source_album_artist_id) → album_id) alias.
    pub async fn upsert_album_alias(
        tx: &mut sqlx::PgConnection,
        source_title: &str,
        source_album_artist_id: Option<Uuid>,
        album_id: Uuid,
    ) -> Result<(), AppError> {
        // The two partial unique indexes (with-artist / no-artist) need
        // separate ON CONFLICT targets. Easier to delete-then-insert.
        sqlx::query!(
            r#"
            DELETE FROM album_aliases
            WHERE LOWER(source_title) = LOWER($1)
                AND (source_album_artist_id = $2
                    OR (source_album_artist_id IS NULL AND $2 IS NULL))
            "#,
            source_title,
            source_album_artist_id,
        )
        .execute(&mut *tx)
        .await?;
        sqlx::query!(
            r#"
            INSERT INTO album_aliases (source_title, source_album_artist_id, album_id)
                VALUES ($1, $2, $3)
            "#,
            source_title,
            source_album_artist_id,
            album_id,
        )
        .execute(&mut *tx)
        .await?;
        Ok(())
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
}
