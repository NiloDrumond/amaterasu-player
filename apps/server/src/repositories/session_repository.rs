use sqlx::PgExecutor;

use crate::{db::entities::Session, error::AppResult};

pub struct SessionRepository;

impl SessionRepository {
    pub async fn create(executor: impl PgExecutor<'_>, session: &Session) -> AppResult<Session> {
        let created = sqlx::query_as!(
            Session,
            r#"
            INSERT INTO sessions (id, user_id, ip_address, metadata, expires_at, created_at, updated_at)
                VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING
                *
"#,
            session.id,
            session.user_id,
            session.ip_address,
            session.metadata,
            session.expires_at,
            session.created_at,
            session.updated_at
        )
        .fetch_one(executor)
        .await?;

        Ok(created)
    }

    pub async fn find_by_id(executor: impl PgExecutor<'_>, id: &str) -> AppResult<Option<Session>> {
        let session = sqlx::query_as!(
            Session,
            r#"
            SELECT
                *
            FROM
                sessions
            WHERE
                id = $1
"#,
            id
        )
        .fetch_optional(executor)
        .await?;

        Ok(session)
    }

    pub async fn delete_by_id(executor: impl PgExecutor<'_>, id: &str) -> AppResult<Session> {
        let session = sqlx::query_as!(
            Session,
            r#"
            DELETE FROM sessions
            WHERE id = $1
            RETURNING
                *
            "#,
            id
        )
        .fetch_one(executor)
        .await?;

        Ok(session)
    }
}
