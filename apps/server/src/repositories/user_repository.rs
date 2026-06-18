use sqlx::PgExecutor;
use uuid::Uuid;

use crate::{db::entities::User, error::AppResult};

pub struct UserRepository;

impl UserRepository {
    pub async fn create(executor: impl PgExecutor<'_>, user: &User) -> AppResult<User> {
        let created = sqlx::query_as!(
            User,
            r#"
        INSERT INTO users (id, name, email, password_hash, role, preferences, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        RETURNING
            *
        "#,
            user.id,
            user.name,
            user.email,
            user.password_hash,
            user.role,
            user.preferences,
            user.created_at,
            user.updated_at
        )
        .fetch_one(executor)
        .await?;

        Ok(created)
    }

    pub async fn find_by_id(executor: impl PgExecutor<'_>, id: Uuid) -> AppResult<Option<User>> {
        let user = sqlx::query_as!(
            User,
            r#"
        SELECT
            *
        FROM
            users
        WHERE
            id = $1
        "#,
            id
        )
        .fetch_optional(executor)
        .await?;

        Ok(user)
    }

    pub async fn find_by_email(
        executor: impl PgExecutor<'_>,
        email: &str,
    ) -> AppResult<Option<User>> {
        let user = sqlx::query_as!(
            User,
            r#"
         SELECT
             *
         FROM
             users
         WHERE
             email = $1
        "#,
            email
        )
        .fetch_optional(executor)
        .await?;

        Ok(user)
    }

    pub async fn list_paginated(
        executor: impl PgExecutor<'_>,
        limit: i32,
        offset: i32,
    ) -> AppResult<Vec<User>> {
        let users = sqlx::query_as!(
            User,
            r#"
        SELECT
            *
        FROM
            users
        ORDER BY
            created_at DESC
        LIMIT $1 OFFSET $2
        "#,
            limit as i64,
            offset as i64
        )
        .fetch_all(executor)
        .await?;

        Ok(users)
    }

    pub async fn count(executor: impl PgExecutor<'_>) -> AppResult<i64> {
        let count = sqlx::query_scalar!(r#"SELECT COUNT(*) AS "count!" FROM users"#)
            .fetch_one(executor)
            .await?;

        Ok(count)
    }

    pub async fn update_password(
        executor: impl PgExecutor<'_>,
        id: Uuid,
        password_hash: &str,
    ) -> AppResult<bool> {
        let result = sqlx::query!(
            r#"UPDATE users SET password_hash = $1, updated_at = NOW() WHERE id = $2"#,
            password_hash,
            id
        )
        .execute(executor)
        .await?;

        Ok(result.rows_affected() > 0)
    }

    pub async fn delete(executor: impl PgExecutor<'_>, id: Uuid) -> AppResult<bool> {
        let result = sqlx::query!(r#"DELETE FROM users WHERE id = $1"#, id)
            .execute(executor)
            .await?;

        Ok(result.rows_affected() > 0)
    }

    pub async fn count_admins(executor: impl PgExecutor<'_>) -> AppResult<i64> {
        let count =
            sqlx::query_scalar!(r#"SELECT COUNT(*) AS "count!" FROM users WHERE role = 'admin'"#)
                .fetch_one(executor)
                .await?;

        Ok(count)
    }

    pub async fn update_preferences(
        executor: impl PgExecutor<'_>,
        user_id: Uuid,
        preferences: &serde_json::Value,
    ) -> AppResult<()> {
        sqlx::query!(
            r#"UPDATE users SET preferences = $1 WHERE id = $2"#,
            preferences,
            user_id
        )
        .execute(executor)
        .await?;

        Ok(())
    }
}
