use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Artist {
    pub id: Uuid,
    pub name: String,
    pub mbid: Option<String>, // MusicBrainz ID
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Artist {
    /// Create a new artist
    pub fn new(name: String) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            name,
            mbid: None,
            created_at: now,
            updated_at: now,
        }
    }
    
    /// Create an artist with a MusicBrainz ID
    pub fn with_mbid(name: String, mbid: String) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            name,
            mbid: Some(mbid),
            created_at: now,
            updated_at: now,
        }
    }
}
