use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String,

    #[serde(skip_serializing)]
    pub password_hash: String,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl User {
    pub fn new(name: String, email: String, password_hash: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            email,
            password_hash,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}
