use sqlx::PgPool;

use crate::{
    auth::hash_password,
    auth::error::AuthError,
    db::entities::User,
    error::AppResult,
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
        let previous_user = UserRepository::find_by_email(&self.pool, &email).await?;

        if previous_user.is_some() {
            return Err(AuthError::EmailAlreadyTaken.into());
        }

        let password_hash = hash_password(&password)?;

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
