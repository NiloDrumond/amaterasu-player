use amaterasu_macros::api_type;
use serde::Serialize;
use uuid::Uuid;

use crate::{
    dto::response::common_response::PaginatedResponse, services::library_service::ArtistWithRefs,
};

#[api_type("response/artist")]
#[derive(Debug, Serialize)]
pub struct ArtistResponse {
    pub id: Uuid,
    pub name: String,
    pub album_count: i64,
    pub track_count: i64,
    pub play_count: i64,
}

impl From<ArtistWithRefs> for ArtistResponse {
    fn from(value: ArtistWithRefs) -> Self {
        Self {
            id: value.artist.id,
            name: value.artist.name,
            album_count: value.album_count,
            track_count: value.track_count,
            play_count: value.play_count,
        }
    }
}

#[api_type("response/artist")]
#[derive(Debug, Serialize)]
struct GetArtistsResponse(PaginatedResponse<ArtistResponse>);
