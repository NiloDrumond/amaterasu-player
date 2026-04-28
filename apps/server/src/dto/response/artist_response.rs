use amaterasu_macros::api_type;
use serde::Serialize;
use uuid::Uuid;

use crate::{dto::response::common_response::PaginatedResponse, services::library_service::ArtistWithRefs};

#[api_type("response/artist")]
#[derive(Debug, Serialize)]
pub struct ArtistResponse {
    pub id: Uuid,
    pub name: String,
    pub album_count: i64,
}

impl From<ArtistWithRefs> for ArtistResponse {
    fn from(value: ArtistWithRefs) -> Self {
        Self {
            id: value.artist.id,
            name: value.artist.name,
            album_count: value.album_count,
        }
    }
}

#[api_type("response/artist")]
#[derive(Debug, Serialize)]
struct GetArtistsResponse(PaginatedResponse<ArtistResponse>);
