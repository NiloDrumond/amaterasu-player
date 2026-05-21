use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Artist {
    pub id: Uuid,
    pub name: String,
    pub sort_name: String,
    pub mbid: Option<String>, // MusicBrainz ID
    pub locked_at: Option<DateTime<Utc>>,
    pub approved: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Artist {
    pub fn new(name: String, sort_name: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            sort_name,
            mbid: None,
            locked_at: None,
            approved: false,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}
