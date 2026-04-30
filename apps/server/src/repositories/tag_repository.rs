use sqlx::PgExecutor;
use uuid::Uuid;

use crate::db::entities::Tag;
use crate::error::AppResult;

pub struct TagRepository;

pub struct TagWithUsage {
    pub tag: Tag,
    pub track_count: i64,
    pub album_count: i64,
}

impl TagRepository {
    pub async fn create(
        executor: impl PgExecutor<'_>,
        user_id: Uuid,
        name: &str,
        category: Option<&str>,
        color: Option<&str>,
    ) -> AppResult<Tag> {
        let tag = sqlx::query_as!(
            Tag,
            r#"
            INSERT INTO tags (user_id, name, category, color)
                VALUES ($1, $2, $3, $4)
            RETURNING
                id,
                user_id,
                name,
                category,
                color,
                created_at,
                updated_at
            "#,
            user_id,
            name,
            category,
            color
        )
        .fetch_one(executor)
        .await?;

        Ok(tag)
    }

    pub async fn list_by_user(
        executor: impl PgExecutor<'_>,
        user_id: Uuid,
    ) -> AppResult<Vec<TagWithUsage>> {
        let rows = sqlx::query!(
            r#"
            SELECT
                t.id,
                t.user_id,
                t.name,
                t.category,
                t.color,
                t.created_at,
                t.updated_at,
                COUNT(DISTINCT tt.track_id) AS "track_count!: i64",
                COUNT(DISTINCT at.album_id) AS "album_count!: i64"
            FROM
                tags t
            LEFT JOIN track_tags tt ON tt.tag_id = t.id
            LEFT JOIN album_tags at ON at.tag_id = t.id
            WHERE
                t.user_id = $1
            GROUP BY
                t.id
            ORDER BY
                t.category NULLS LAST,
                t.name ASC
            "#,
            user_id
        )
        .fetch_all(executor)
        .await?;

        Ok(rows
            .into_iter()
            .map(|r| TagWithUsage {
                tag: Tag {
                    id: r.id,
                    user_id: r.user_id,
                    name: r.name,
                    category: r.category,
                    color: r.color,
                    created_at: r.created_at,
                    updated_at: r.updated_at,
                },
                track_count: r.track_count,
                album_count: r.album_count,
            })
            .collect())
    }

    pub async fn find_by_id_and_user(
        executor: impl PgExecutor<'_>,
        id: Uuid,
        user_id: Uuid,
    ) -> AppResult<Option<TagWithUsage>> {
        let row = sqlx::query!(
            r#"
            SELECT
                t.id,
                t.user_id,
                t.name,
                t.category,
                t.color,
                t.created_at,
                t.updated_at,
                COUNT(DISTINCT tt.track_id) AS "track_count!: i64",
                COUNT(DISTINCT at.album_id) AS "album_count!: i64"
            FROM
                tags t
            LEFT JOIN track_tags tt ON tt.tag_id = t.id
            LEFT JOIN album_tags at ON at.tag_id = t.id
            WHERE
                t.id = $1
                AND t.user_id = $2
            GROUP BY
                t.id
            "#,
            id,
            user_id
        )
        .fetch_optional(executor)
        .await?;

        Ok(row.map(|r| TagWithUsage {
            tag: Tag {
                id: r.id,
                user_id: r.user_id,
                name: r.name,
                category: r.category,
                color: r.color,
                created_at: r.created_at,
                updated_at: r.updated_at,
            },
            track_count: r.track_count,
            album_count: r.album_count,
        }))
    }

    /// Updates only the fields provided. `None` leaves the column unchanged.
    pub async fn update(
        executor: impl PgExecutor<'_>,
        id: Uuid,
        user_id: Uuid,
        name: Option<&str>,
        category: Option<&str>,
        color: Option<&str>,
    ) -> AppResult<Option<Tag>> {
        let tag = sqlx::query_as!(
            Tag,
            r#"
            UPDATE tags
            SET
                name     = COALESCE($3, name),
                category = COALESCE($4, category),
                color    = COALESCE($5, color)
            WHERE
                id = $1
                AND user_id = $2
            RETURNING
                id,
                user_id,
                name,
                category,
                color,
                created_at,
                updated_at
            "#,
            id,
            user_id,
            name,
            category,
            color
        )
        .fetch_optional(executor)
        .await?;

        Ok(tag)
    }

    pub async fn delete(executor: impl PgExecutor<'_>, id: Uuid, user_id: Uuid) -> AppResult<bool> {
        let result = sqlx::query!(
            r#"
            DELETE FROM tags
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

    /// Idempotent: inserts only if (track_id, tag_id) doesn't already exist
    /// AND the tag belongs to the user. Returns true if the tag is owned by
    /// the user (and thus is now attached); false otherwise.
    pub async fn attach_to_track(
        executor: impl PgExecutor<'_>,
        tag_id: Uuid,
        track_id: Uuid,
        user_id: Uuid,
    ) -> AppResult<bool> {
        let row = sqlx::query!(
            r#"
            WITH owned AS (
                SELECT id FROM tags WHERE id = $1 AND user_id = $3
            ),
            upsert AS (
                INSERT INTO track_tags (track_id, tag_id)
                SELECT $2, id FROM owned
                ON CONFLICT (track_id, tag_id) DO NOTHING
                RETURNING tag_id
            )
            SELECT EXISTS(SELECT 1 FROM owned) AS "owned!"
            "#,
            tag_id,
            track_id,
            user_id
        )
        .fetch_one(executor)
        .await?;

        Ok(row.owned)
    }

    pub async fn detach_from_track(
        executor: impl PgExecutor<'_>,
        tag_id: Uuid,
        track_id: Uuid,
        user_id: Uuid,
    ) -> AppResult<bool> {
        let result = sqlx::query!(
            r#"
            DELETE FROM track_tags tt
            USING tags t
            WHERE
                tt.tag_id = t.id
                AND t.user_id = $3
                AND tt.track_id = $1
                AND tt.tag_id = $2
            "#,
            track_id,
            tag_id,
            user_id
        )
        .execute(executor)
        .await?;

        Ok(result.rows_affected() > 0)
    }

    pub async fn attach_to_album(
        executor: impl PgExecutor<'_>,
        tag_id: Uuid,
        album_id: Uuid,
        user_id: Uuid,
    ) -> AppResult<bool> {
        let row = sqlx::query!(
            r#"
            WITH owned AS (
                SELECT id FROM tags WHERE id = $1 AND user_id = $3
            ),
            upsert AS (
                INSERT INTO album_tags (album_id, tag_id)
                SELECT $2, id FROM owned
                ON CONFLICT (album_id, tag_id) DO NOTHING
                RETURNING tag_id
            )
            SELECT EXISTS(SELECT 1 FROM owned) AS "owned!"
            "#,
            tag_id,
            album_id,
            user_id
        )
        .fetch_one(executor)
        .await?;

        Ok(row.owned)
    }

    pub async fn detach_from_album(
        executor: impl PgExecutor<'_>,
        tag_id: Uuid,
        album_id: Uuid,
        user_id: Uuid,
    ) -> AppResult<bool> {
        let result = sqlx::query!(
            r#"
            DELETE FROM album_tags at
            USING tags t
            WHERE
                at.tag_id = t.id
                AND t.user_id = $3
                AND at.album_id = $1
                AND at.tag_id = $2
            "#,
            album_id,
            tag_id,
            user_id
        )
        .execute(executor)
        .await?;

        Ok(result.rows_affected() > 0)
    }

    pub async fn list_track_tags(
        executor: impl PgExecutor<'_>,
        track_id: Uuid,
        user_id: Uuid,
    ) -> AppResult<Vec<Tag>> {
        let tags = sqlx::query_as!(
            Tag,
            r#"
            SELECT
                t.id,
                t.user_id,
                t.name,
                t.category,
                t.color,
                t.created_at,
                t.updated_at
            FROM
                tags t
            JOIN track_tags tt ON tt.tag_id = t.id
            WHERE
                tt.track_id = $1
                AND t.user_id = $2
            ORDER BY
                t.category NULLS LAST,
                t.name ASC
            "#,
            track_id,
            user_id
        )
        .fetch_all(executor)
        .await?;

        Ok(tags)
    }

    pub async fn list_album_tags(
        executor: impl PgExecutor<'_>,
        album_id: Uuid,
        user_id: Uuid,
    ) -> AppResult<Vec<Tag>> {
        let tags = sqlx::query_as!(
            Tag,
            r#"
            SELECT
                t.id,
                t.user_id,
                t.name,
                t.category,
                t.color,
                t.created_at,
                t.updated_at
            FROM
                tags t
            JOIN album_tags at ON at.tag_id = t.id
            WHERE
                at.album_id = $1
                AND t.user_id = $2
            ORDER BY
                t.category NULLS LAST,
                t.name ASC
            "#,
            album_id,
            user_id
        )
        .fetch_all(executor)
        .await?;

        Ok(tags)
    }
}
