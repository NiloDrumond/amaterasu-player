use chrono::{DateTime, Utc};
use ipnet::IpNet;
use rand::{distr::Alphanumeric, prelude::*};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Session {
    pub id: String,
    pub user_id: Uuid,
    pub ip_address: Option<IpNet>,
    pub metadata: Option<Value>,
    pub expires_at: DateTime<Utc>,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Session {
    pub fn new(
        user_id: Uuid,
        expires_at: DateTime<Utc>,
        ip_address: Option<IpNet>,
        metadata: Option<Value>,
    ) -> Self {
        let mut rng = rand::rng();
        let id = rng.sample(Alphanumeric).to_string();
        Self {
            user_id,
            ip_address,
            metadata,
            expires_at,
            id,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}
