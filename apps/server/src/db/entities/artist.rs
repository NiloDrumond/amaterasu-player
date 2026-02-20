use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, Default)]
pub struct Artist {
    pub id: Uuid,
    pub name: String,
    pub sort_name: String,
    pub mbid: Option<String>, // MusicBrainz ID
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
