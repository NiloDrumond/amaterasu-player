use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Album {
    pub id: Uuid,
    pub artist_id: Option<Uuid>,
    pub title: String,
    pub year: Option<i32>,
    pub mbid: Option<String>, // MusicBrainz ID
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Album {
    /// Create a new album
    pub fn new(title: String, artist_id: Option<Uuid>) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            artist_id,
            title,
            year: None,
            mbid: None,
            created_at: now,
            updated_at: now,
        }
    }

    /// Create an album with year
    pub fn with_year(title: String, artist_id: Option<Uuid>, year: i32) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            artist_id,
            title,
            year: Some(year),
            mbid: None,
            created_at: now,
            updated_at: now,
        }
    }
}

