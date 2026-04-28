use amaterasu_macros::api_type;
use chrono::NaiveDate;
use serde::Serialize;
use uuid::Uuid;

use crate::{
    db::entities::Artist, dto::response::common_response::PaginatedResponse,
    services::library_service::AlbumWithRefs,
};

#[api_type("response/album")]
#[derive(Debug, Serialize)]
pub struct AlbumArtistResponse {
    pub id: Uuid,
    pub name: String,
}

impl From<Artist> for AlbumArtistResponse {
    fn from(value: Artist) -> Self {
        Self {
            id: value.id,
            name: value.name,
        }
    }
}

#[api_type("response/album")]
#[derive(Debug, Serialize)]
pub struct AlbumResponse {
    pub id: Uuid,
    pub title: String,
    pub artist: Option<AlbumArtistResponse>,
    pub date: Option<NaiveDate>,
    pub cover_url: Option<String>,
    pub track_count: i64,
    pub total_duration_ms: i64,
}

impl From<AlbumWithRefs> for AlbumResponse {
    fn from(value: AlbumWithRefs) -> Self {
        let AlbumWithRefs {
            album,
            artist,
            track_count,
            total_duration_ms,
        } = value;

        Self {
            id: album.id,
            title: album.title,
            artist: artist.map(|a| a.into()),
            date: album.date,
            cover_url: album.cover_path.map(|p| format!("/api/covers/{p}")),
            track_count,
            total_duration_ms,
        }
    }
}

#[api_type("response/album")]
#[derive(Debug, Serialize)]
struct GetAlbumsResponse(PaginatedResponse<AlbumResponse>);
