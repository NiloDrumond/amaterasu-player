use sqlx::PgPool;

use crate::{
    auth::hash_password,
    db::entities::User,
    error::{AppError, AppResult},
    repositories::UserRepository,
};

pub struct AuthService {
    pool: PgPool,
}

impl AuthService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn register_email(
        &self,
        email: String,
        name: String,
        password: String,
    ) -> AppResult<()> {
        let password_hash =
            hash_password(&password).map_err(|err| AppError::Internal(err.into()))?;

        let user = User {
            name,
            email,
            password_hash,
            ..Default::default()
        };

        let _ = UserRepository::create(&self.pool, &user).await?;

        Ok(())
    }
}
