use amaterasu_macros::api_type;
use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

use crate::db::entities::AlbumCollection;
use crate::filters::FilterNode;

#[api_type("response/album-collection")]
#[derive(Debug, Serialize)]
pub struct AlbumCollectionResponse {
    pub id: Uuid,
    pub name: String,
    pub filter_definition: FilterNode,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<AlbumCollection> for AlbumCollectionResponse {
    fn from(value: AlbumCollection) -> Self {
        Self {
            id: value.id,
            name: value.name,
            filter_definition: value.filter_definition.0,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}
