use sqlx::PgExecutor;
use uuid::Uuid;

use crate::{db::entities::User, error::AppResult};

pub struct UserRepository;

impl UserRepository {
    pub async fn create(executor: impl PgExecutor<'_>, user: &User) -> AppResult<User> {
        let created = sqlx::query_as!(
            User,
            r#"
        INSERT INTO users (id, name, email, password_hash, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING
            *
        "#,
            user.id,
            user.name,
            user.email,
            user.password_hash,
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
}
