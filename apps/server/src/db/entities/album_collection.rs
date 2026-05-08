use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::types::Json;
use sqlx::FromRow;
use uuid::Uuid;

use crate::filters::FilterNode;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct AlbumCollection {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub filter_definition: Json<FilterNode>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
