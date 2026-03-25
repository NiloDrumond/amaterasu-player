use chrono::Duration;
use chrono::Utc;
use sqlx::PgPool;

use crate::auth::SESSION_DURATION_HOURS;
use crate::repositories::SessionRepository;
use crate::{
    auth::{error::AuthError, hash_password, verify_password},
    db::entities::{Session, User},
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

    pub async fn sign_in_email(&self, email: &str, password: &str) -> AppResult<Session> {
        let user = UserRepository::find_by_email(&self.pool, email)
            .await?
            .ok_or(AuthError::UserNotFound)?;
        let correct = verify_password(password, &user.password_hash)?;
        if !correct {
            return Err(AuthError::PasswordDoesntMatch.into());
        }

        let expires_at = Utc::now() + Duration::hours(SESSION_DURATION_HOURS.into());

        let session = Session::new(user.id, expires_at, None, None);
        SessionRepository::create(&self.pool, &session);

        Ok(session)
    }
}
