use amaterasu_macros::api_type;
use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

use crate::db::entities::Tag;
use crate::repositories::tag_repository::TagWithUsage;

#[api_type("response/tag")]
#[derive(Debug, Serialize)]
pub struct TagResponse {
    pub id: Uuid,
    pub name: String,
    pub category: Option<String>,
    pub color: Option<String>,
    pub track_count: i64,
    pub album_count: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<TagWithUsage> for TagResponse {
    fn from(value: TagWithUsage) -> Self {
        Self {
            id: value.tag.id,
            name: value.tag.name,
            category: value.tag.category,
            color: value.tag.color,
            track_count: value.track_count,
            album_count: value.album_count,
            created_at: value.tag.created_at,
            updated_at: value.tag.updated_at,
        }
    }
}

#[api_type("response/tag")]
#[derive(Debug, Serialize)]
pub struct TagSummaryResponse {
    pub id: Uuid,
    pub name: String,
    pub category: Option<String>,
    pub color: Option<String>,
}

impl From<Tag> for TagSummaryResponse {
    fn from(value: Tag) -> Self {
        Self {
            id: value.id,
            name: value.name,
            category: value.category,
            color: value.color,
        }
    }
}
