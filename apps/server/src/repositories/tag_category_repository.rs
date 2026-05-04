use sqlx::PgExecutor;
use uuid::Uuid;

use crate::db::entities::TagCategory;
use crate::error::AppResult;

pub struct TagCategoryRepository;

pub struct TagCategoryWithCount {
    pub category: TagCategory,
    pub tag_count: i64,
}

impl TagCategoryRepository {
    pub async fn list_by_user(
        executor: impl PgExecutor<'_>,
        user_id: Uuid,
    ) -> AppResult<Vec<TagCategoryWithCount>> {
        let rows = sqlx::query!(
            r#"
            SELECT
                c.id,
                c.user_id,
                c.name,
                c.color,
                c.position,
                c.created_at,
                c.updated_at,
                COUNT(t.id) AS "tag_count!: i64"
            FROM
                tag_categories c
            LEFT JOIN tags t ON t.category_id = c.id
            WHERE
                c.user_id = $1
            GROUP BY
                c.id
            ORDER BY
                c.position ASC,
                c.name ASC
            "#,
            user_id
        )
        .fetch_all(executor)
        .await?;

        Ok(rows
            .into_iter()
            .map(|r| TagCategoryWithCount {
                category: TagCategory {
                    id: r.id,
                    user_id: r.user_id,
                    name: r.name,
                    color: r.color,
                    position: r.position,
                    created_at: r.created_at,
                    updated_at: r.updated_at,
                },
                tag_count: r.tag_count,
            })
            .collect())
    }

    pub async fn find_by_id_and_user(
        executor: impl PgExecutor<'_>,
        id: Uuid,
        user_id: Uuid,
    ) -> AppResult<Option<TagCategoryWithCount>> {
        let row = sqlx::query!(
            r#"
            SELECT
                c.id,
                c.user_id,
                c.name,
                c.color,
                c.position,
                c.created_at,
                c.updated_at,
                COUNT(t.id) AS "tag_count!: i64"
            FROM
                tag_categories c
            LEFT JOIN tags t ON t.category_id = c.id
            WHERE
                c.id = $1
                AND c.user_id = $2
            GROUP BY
                c.id
            "#,
            id,
            user_id
        )
        .fetch_optional(executor)
        .await?;

        Ok(row.map(|r| TagCategoryWithCount {
            category: TagCategory {
                id: r.id,
                user_id: r.user_id,
                name: r.name,
                color: r.color,
                position: r.position,
                created_at: r.created_at,
                updated_at: r.updated_at,
            },
            tag_count: r.tag_count,
        }))
    }

    pub async fn create(
        executor: impl PgExecutor<'_>,
        user_id: Uuid,
        name: &str,
        color: Option<&str>,
    ) -> AppResult<TagCategory> {
        let category = sqlx::query_as!(
            TagCategory,
            r#"
            INSERT INTO tag_categories (user_id, name, color, position)
                VALUES (
                    $1,
                    $2,
                    $3,
                    COALESCE(
                        (SELECT MAX(position) + 1 FROM tag_categories WHERE user_id = $1),
                        0
                    )
                )
            RETURNING
                id,
                user_id,
                name,
                color,
                position,
                created_at,
                updated_at
            "#,
            user_id,
            name,
            color
        )
        .fetch_one(executor)
        .await?;

        Ok(category)
    }

    pub async fn update(
        executor: impl PgExecutor<'_>,
        id: Uuid,
        user_id: Uuid,
        name: Option<&str>,
        color: Option<&str>,
    ) -> AppResult<Option<TagCategory>> {
        let category = sqlx::query_as!(
            TagCategory,
            r#"
            UPDATE tag_categories
            SET
                name  = COALESCE($3, name),
                color = COALESCE($4, color)
            WHERE
                id = $1
                AND user_id = $2
            RETURNING
                id,
                user_id,
                name,
                color,
                position,
                created_at,
                updated_at
            "#,
            id,
            user_id,
            name,
            color
        )
        .fetch_optional(executor)
        .await?;

        Ok(category)
    }

    pub async fn delete(executor: impl PgExecutor<'_>, id: Uuid, user_id: Uuid) -> AppResult<bool> {
        let result = sqlx::query!(
            r#"
            DELETE FROM tag_categories
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

    /// Reassigns `position` to match the order of `ordered_ids`. Categories not
    /// in the list are left untouched. Scoped by `user_id` so callers can't
    /// reorder another user's categories.
    pub async fn reorder(
        executor: impl PgExecutor<'_>,
        user_id: Uuid,
        ordered_ids: &[Uuid],
    ) -> AppResult<()> {
        sqlx::query!(
            r#"
            UPDATE tag_categories AS c
            SET position = ord.pos - 1
            FROM (
                SELECT id, ordinality AS pos
                FROM unnest($2::uuid[]) WITH ORDINALITY AS u(id, ordinality)
            ) AS ord
            WHERE
                c.id = ord.id
                AND c.user_id = $1
            "#,
            user_id,
            ordered_ids
        )
        .execute(executor)
        .await?;

        Ok(())
    }
}
