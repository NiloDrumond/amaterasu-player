use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::types::Json;
use sqlx::FromRow;
use uuid::Uuid;

use crate::filters::FilterNode;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Playlist {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    /// 'manual' or 'dynamic'.
    #[sqlx(rename = "type")]
    pub playlist_type: String,
    pub filter_definition: Option<Json<FilterNode>>,
    pub cached_track_count: i32,
    pub cached_total_duration_ms: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct PlaylistTrack {
    pub id: Uuid,
    pub playlist_id: Uuid,
    pub track_id: Uuid,
    pub position: f64,
    pub added_at: DateTime<Utc>,
}
