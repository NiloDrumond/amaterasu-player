use std::net::IpAddr;

use chrono::Duration;
use chrono::Utc;
use ipnet::IpNet;
use sqlx::PgPool;

use crate::auth::SESSION_DURATION_HOURS;
use crate::repositories::SessionRepository;
use crate::{
    auth::{hash_password, verify_password, AuthError},
    db::entities::{Session, User},
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
        let email = email.trim().to_lowercase();
        let password_hash = hash_password(&password)?;

        let user = User::new(name, email, password_hash);

        match UserRepository::create(&self.pool, &user).await {
            Ok(_) => Ok(()),
            Err(AppError::Database(sqlx::Error::Database(e))) if e.is_unique_violation() => {
                Err(AuthError::EmailAlreadyTaken.into())
            }
            Err(e) => Err(e),
        }
    }

    pub async fn sign_in_email(
        &self,
        email: &str,
        password: &str,
        ip_address: Option<IpAddr>,
    ) -> AppResult<Session> {
        let email = email.trim().to_lowercase();
        let user = UserRepository::find_by_email(&self.pool, &email)
            .await?
            .ok_or(AuthError::UserNotFound)?;
        let correct = verify_password(password, &user.password_hash)?;
        if !correct {
            tracing::info!(email = %email, "Failed login attempt: incorrect password");
            return Err(AuthError::PasswordDoesntMatch.into());
        }

        let expires_at = Utc::now() + Duration::hours(SESSION_DURATION_HOURS.into());

        let ip_net = ip_address.map(IpNet::from);
        let session = Session::new(user.id, expires_at, ip_net, None);

        let pool = self.pool.clone();
        let mut transaction = pool.begin().await?;
        let session = SessionRepository::create(&mut *transaction, &session).await?;
        SessionRepository::cap_user_sessions(&mut *transaction, user.id).await?;
        transaction.commit().await?;

        tracing::info!(session_id = %session.id, user_id = %user.id, "Session created");

        Ok(session)
    }

    pub async fn validate_session(&self, session_id: &str) -> AppResult<(Session, User)> {
        let session = SessionRepository::find_by_id(&self.pool, session_id)
            .await?
            .ok_or(AuthError::SessionNotFound(session_id.to_string()))?;

        let valid = session.is_valid();

        if !valid {
            SessionRepository::delete_by_id(&self.pool, &session.id).await?;
            return Err(AuthError::ExpiredSession.into());
        }

        let user = UserRepository::find_by_id(&self.pool, session.user_id)
            .await?
            .ok_or(AuthError::UserNotFoundForSession(session.id.clone()))?;

        Ok((session, user))
    }

    pub async fn delete_session(&self, session_id: &str) -> Option<Session> {
        match SessionRepository::delete_by_id(&self.pool, session_id).await {
            Ok(session) => {
                tracing::info!(session_id, user_id = %session.user_id, "Session deleted");
                Some(session)
            }
            Err(e) => {
                tracing::error!(session_id, error = %e, "Failed to delete session");
                None
            }
        }
    }

    pub async fn delete_expired(&self) -> AppResult<usize> {
        let count = SessionRepository::delete_expired(&self.pool).await?;
        if count > 0 {
            tracing::info!(count, "Expired sessions cleaned up");
        }
        Ok(count)
    }

    pub async fn bootstrap_admin(&self, email: &str, password: &str, name: &str) -> AppResult<()> {
        let email = email.trim().to_lowercase();

        let admin_count = UserRepository::count_admins(&self.pool).await?;
        if admin_count > 0 {
            tracing::info!("Admin user already exists, skipping bootstrap");
            return Ok(());
        }

        if UserRepository::find_by_email(&self.pool, &email)
            .await?
            .is_some()
        {
            return Err(AppError::Internal(anyhow::anyhow!(
                "ADMIN_EMAIL {email} is already taken by a non-admin user; remove the user or change ADMIN_EMAIL"
            )));
        }

        let password_hash = hash_password(password)?;
        let user = User::new_admin(name.to_string(), email.clone(), password_hash);
        UserRepository::create(&self.pool, &user).await?;

        tracing::info!(email = %email, "Admin user bootstrapped");
        Ok(())
    }
}
