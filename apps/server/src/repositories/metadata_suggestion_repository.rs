//! Persistence for MusicBrainz suggestion candidates and previously-rejected
//! candidate MBIDs.

use chrono::{DateTime, Utc};
use serde_json::Value as JsonValue;
use sqlx::{FromRow, PgExecutor};
use uuid::Uuid;

use crate::error::{AppError, AppResult};

/// Discriminator for the polymorphic `entity_id` column. Mirrors the DB CHECK
/// constraint on `metadata_suggestions.entity_type` exactly.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SuggestionEntityType {
    Album,
    Artist,
    Track,
}

impl SuggestionEntityType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Album => "album",
            Self::Artist => "artist",
            Self::Track => "track",
        }
    }

    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "album" => Some(Self::Album),
            "artist" => Some(Self::Artist),
            "track" => Some(Self::Track),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, FromRow)]
pub struct MetadataSuggestion {
    pub id: Uuid,
    pub entity_type: String,
    pub entity_id: Uuid,
    pub source: String,
    pub mbid: String,
    pub score: i16,
    pub rank: i16,
    pub proposed: JsonValue,
    pub raw: Option<JsonValue>,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct NewSuggestion {
    pub mbid: String,
    pub score: i16,
    pub proposed: JsonValue,
    pub raw: Option<JsonValue>,
}

pub struct MetadataSuggestionRepository;

impl MetadataSuggestionRepository {
    /// Marks any existing pending suggestions for this entity as `superseded`
    /// then inserts the fresh batch with stable ranks `0..candidates.len()`.
    /// Called whenever a new MB lookup completes. Caller is expected to run
    /// this inside a transaction; takes the connection directly to allow
    /// reusing it across multiple queries.
    pub async fn replace_pending_for_entity(
        conn: &mut sqlx::PgConnection,
        entity_type: SuggestionEntityType,
        entity_id: Uuid,
        candidates: &[NewSuggestion],
    ) -> AppResult<()> {
        sqlx::query!(
            r#"
            UPDATE metadata_suggestions
            SET status = 'superseded', updated_at = NOW()
            WHERE entity_type = $1 AND entity_id = $2 AND status = 'pending'
            "#,
            entity_type.as_str(),
            entity_id,
        )
        .execute(&mut *conn)
        .await?;

        for (rank, c) in candidates.iter().enumerate() {
            sqlx::query!(
                r#"
                INSERT INTO metadata_suggestions
                    (entity_type, entity_id, source, mbid, score, rank, proposed, raw)
                VALUES ($1, $2, 'musicbrainz', $3, $4, $5, $6, $7)
                "#,
                entity_type.as_str(),
                entity_id,
                c.mbid,
                c.score,
                rank as i16,
                c.proposed,
                c.raw,
            )
            .execute(&mut *conn)
            .await?;
        }
        Ok(())
    }

    pub async fn find_by_entity(
        executor: impl PgExecutor<'_>,
        entity_type: SuggestionEntityType,
        entity_id: Uuid,
    ) -> AppResult<Vec<MetadataSuggestion>> {
        let rows = sqlx::query_as!(
            MetadataSuggestion,
            r#"
            SELECT id, entity_type, entity_id, source, mbid, score, rank,
                   proposed, raw, status, created_at, updated_at
            FROM metadata_suggestions
            WHERE entity_type = $1 AND entity_id = $2
            ORDER BY rank ASC
            "#,
            entity_type.as_str(),
            entity_id,
        )
        .fetch_all(executor)
        .await?;
        Ok(rows)
    }

    /// Bulk variant for the review-queue batch endpoint. Returns rows
    /// ordered by `(entity_id, rank)`; caller buckets them.
    pub async fn find_pending_by_entity_ids(
        executor: impl PgExecutor<'_>,
        entity_type: SuggestionEntityType,
        entity_ids: &[Uuid],
    ) -> AppResult<Vec<MetadataSuggestion>> {
        if entity_ids.is_empty() {
            return Ok(Vec::new());
        }
        let rows = sqlx::query_as!(
            MetadataSuggestion,
            r#"
            SELECT id, entity_type, entity_id, source, mbid, score, rank,
                   proposed, raw, status, created_at, updated_at
            FROM metadata_suggestions
            WHERE entity_type = $1
              AND entity_id = ANY($2)
              AND status = 'pending'
            ORDER BY entity_id, rank ASC
            "#,
            entity_type.as_str(),
            entity_ids,
        )
        .fetch_all(executor)
        .await?;
        Ok(rows)
    }

    pub async fn find_by_id(
        executor: impl PgExecutor<'_>,
        id: Uuid,
    ) -> AppResult<Option<MetadataSuggestion>> {
        let row = sqlx::query_as!(
            MetadataSuggestion,
            r#"
            SELECT id, entity_type, entity_id, source, mbid, score, rank,
                   proposed, raw, status, created_at, updated_at
            FROM metadata_suggestions
            WHERE id = $1
            "#,
            id,
        )
        .fetch_optional(executor)
        .await?;
        Ok(row)
    }

    pub async fn set_status(
        executor: impl PgExecutor<'_>,
        id: Uuid,
        status: &str,
    ) -> AppResult<()> {
        sqlx::query!(
            r#"UPDATE metadata_suggestions SET status = $2, updated_at = NOW() WHERE id = $1"#,
            id,
            status,
        )
        .execute(executor)
        .await?;
        Ok(())
    }

    /// On accept: mark this row accepted, all sibling pendings as superseded.
    pub async fn accept_and_supersede_siblings(
        executor: impl PgExecutor<'_>,
        id: Uuid,
    ) -> AppResult<()> {
        sqlx::query!(
            r#"
            WITH target AS (
                SELECT entity_type, entity_id FROM metadata_suggestions WHERE id = $1
            )
            UPDATE metadata_suggestions s
            SET status = CASE WHEN s.id = $1 THEN 'accepted' ELSE 'superseded' END,
                updated_at = NOW()
            FROM target t
            WHERE s.entity_type = t.entity_type
              AND s.entity_id = t.entity_id
              AND (s.id = $1 OR s.status = 'pending')
            "#,
            id,
        )
        .execute(executor)
        .await?;
        Ok(())
    }

    pub async fn record_rejection(
        executor: impl PgExecutor<'_>,
        entity_type: SuggestionEntityType,
        entity_id: Uuid,
        mbid: &str,
    ) -> AppResult<()> {
        sqlx::query!(
            r#"
            INSERT INTO metadata_rejections (entity_type, entity_id, mbid)
            VALUES ($1, $2, $3)
            ON CONFLICT DO NOTHING
            "#,
            entity_type.as_str(),
            entity_id,
            mbid,
        )
        .execute(executor)
        .await?;
        Ok(())
    }

    pub async fn rejected_mbids_for_entity(
        executor: impl PgExecutor<'_>,
        entity_type: SuggestionEntityType,
        entity_id: Uuid,
    ) -> AppResult<Vec<String>> {
        let rows = sqlx::query!(
            r#"
            SELECT mbid
            FROM metadata_rejections
            WHERE entity_type = $1 AND entity_id = $2
            "#,
            entity_type.as_str(),
            entity_id,
        )
        .fetch_all(executor)
        .await?;
        Ok(rows.into_iter().map(|r| r.mbid).collect())
    }

    /// Used by the cleanup hook called from `delete_album` / `delete_artist`
    /// (and the track hard-delete path) when an entity is removed for real.
    pub async fn delete_for_entity(
        conn: &mut sqlx::PgConnection,
        entity_type: SuggestionEntityType,
        entity_id: Uuid,
    ) -> AppResult<()> {
        sqlx::query!(
            r#"DELETE FROM metadata_suggestions WHERE entity_type = $1 AND entity_id = $2"#,
            entity_type.as_str(),
            entity_id,
        )
        .execute(&mut *conn)
        .await?;
        sqlx::query!(
            r#"DELETE FROM metadata_rejections WHERE entity_type = $1 AND entity_id = $2"#,
            entity_type.as_str(),
            entity_id,
        )
        .execute(&mut *conn)
        .await?;
        Ok(())
    }
}

/// Per-entity lookup-status helpers. These flip the `mb_lookup_status` /
/// `mb_lookup_attempted_at` columns on each entity table.
pub struct MbLookupStatusRepository;

impl MbLookupStatusRepository {
    pub async fn mark_album(
        executor: impl PgExecutor<'_>,
        id: Uuid,
        status: &str,
    ) -> AppResult<()> {
        sqlx::query!(
            r#"
            UPDATE albums
            SET mb_lookup_status = $2, mb_lookup_attempted_at = NOW(), updated_at = NOW()
            WHERE id = $1
            "#,
            id,
            status,
        )
        .execute(executor)
        .await?;
        Ok(())
    }

    pub async fn mark_artist(
        executor: impl PgExecutor<'_>,
        id: Uuid,
        status: &str,
    ) -> AppResult<()> {
        sqlx::query!(
            r#"
            UPDATE artists
            SET mb_lookup_status = $2, mb_lookup_attempted_at = NOW(), updated_at = NOW()
            WHERE id = $1
            "#,
            id,
            status,
        )
        .execute(executor)
        .await?;
        Ok(())
    }

    pub async fn mark_track(
        executor: impl PgExecutor<'_>,
        id: Uuid,
        status: &str,
    ) -> AppResult<()> {
        sqlx::query!(
            r#"
            UPDATE tracks
            SET mb_lookup_status = $2, mb_lookup_attempted_at = NOW(), updated_at = NOW()
            WHERE id = $1
            "#,
            id,
            status,
        )
        .execute(executor)
        .await?;
        Ok(())
    }

    /// IDs of albums with no successful lookup yet (NULL or 'failed' status).
    /// Used by the bulk-retry endpoint.
    pub async fn pending_album_ids(
        executor: impl PgExecutor<'_>,
        limit: i64,
    ) -> AppResult<Vec<Uuid>> {
        let rows = sqlx::query!(
            r#"
            SELECT id AS "id!"
            FROM albums
            WHERE mb_lookup_status IS NULL OR mb_lookup_status = 'failed'
            ORDER BY created_at DESC
            LIMIT $1
            "#,
            limit,
        )
        .fetch_all(executor)
        .await?;
        Ok(rows.into_iter().map(|r| r.id).collect())
    }

    pub async fn pending_artist_ids(
        executor: impl PgExecutor<'_>,
        limit: i64,
    ) -> AppResult<Vec<Uuid>> {
        let rows = sqlx::query!(
            r#"
            SELECT id AS "id!"
            FROM artists
            WHERE mb_lookup_status IS NULL OR mb_lookup_status = 'failed'
            ORDER BY created_at DESC
            LIMIT $1
            "#,
            limit,
        )
        .fetch_all(executor)
        .await?;
        Ok(rows.into_iter().map(|r| r.id).collect())
    }
}

// Reference AppError so its use in return types isn't flagged before handlers
// are wired up.
#[allow(dead_code)]
fn _ensure_app_error_is_used(_: AppError) {}
