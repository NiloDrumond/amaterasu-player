use amaterasu_macros::api_type;
use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

use crate::db::entities::TagCategory;
use crate::repositories::tag_category_repository::TagCategoryWithCount;

#[api_type("response/tag-category")]
#[derive(Debug, Serialize, Clone)]
pub struct TagCategoryResponse {
    pub id: Uuid,
    pub name: String,
    pub color: Option<String>,
    pub position: i32,
    pub tag_count: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<TagCategoryWithCount> for TagCategoryResponse {
    fn from(value: TagCategoryWithCount) -> Self {
        Self {
            id: value.category.id,
            name: value.category.name,
            color: value.category.color,
            position: value.category.position,
            tag_count: value.tag_count,
            created_at: value.category.created_at,
            updated_at: value.category.updated_at,
        }
    }
}

#[api_type("response/tag-category")]
#[derive(Debug, Serialize, Clone)]
pub struct TagCategorySummaryResponse {
    pub id: Uuid,
    pub name: String,
    pub color: Option<String>,
    pub position: i32,
}

impl From<TagCategory> for TagCategorySummaryResponse {
    fn from(value: TagCategory) -> Self {
        Self {
            id: value.id,
            name: value.name,
            color: value.color,
            position: value.position,
        }
    }
}
