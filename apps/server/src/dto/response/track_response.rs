use amaterasu_macros::api_type;
use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

use crate::{
    db::entities::{Album, Artist},
    dto::response::{common_response::PaginatedResponse, tag_response::TagSummaryResponse},
    services::library_service::TrackWithRefs,
};

#[api_type("response/track")]
#[derive(Debug, Serialize)]
pub struct TrackAlbumResponse {
    pub id: Uuid,
    pub title: String,
    pub cover_url: Option<String>,
}

impl From<Album> for TrackAlbumResponse {
    fn from(value: Album) -> Self {
        Self {
            id: value.id,
            title: value.title,
            cover_url: value.cover_path.map(|p| format!("/api/covers/{p}")),
        }
    }
}

#[api_type("response/track")]
#[derive(Debug, Serialize)]
pub struct TrackArtistResponse {
    pub id: Uuid,
    pub name: String,
}

impl From<Artist> for TrackArtistResponse {
    fn from(value: Artist) -> Self {
        Self {
            id: value.id,
            name: value.name,
        }
    }
}

#[api_type("response/track")]
#[derive(Debug, Serialize)]
pub struct TrackResponse {
    pub id: Uuid,
    pub title: String,
    pub artist: Option<TrackArtistResponse>,
    pub album: Option<TrackAlbumResponse>,
    pub track_no: Option<i32>,
    pub disc: Option<i32>,
    pub duration_ms: i32,
    pub format: String,
    pub codec: String,
    pub bitrate: Option<i32>,
    pub file_path: String,
    pub original_title: Option<String>,
    pub original_artist: Option<String>,
    pub original_album: Option<String>,
    pub play_count: i64,
    pub tags: Vec<TagSummaryResponse>,
    pub favorite: bool,
    pub created_at: DateTime<Utc>,
}

impl From<TrackWithRefs> for TrackResponse {
    fn from(value: TrackWithRefs) -> Self {
        let TrackWithRefs {
            track,
            album,
            artist,
            play_count,
            tags,
            favorite,
        } = value;

        Self {
            id: track.id,
            title: track.title,
            artist: artist.map(|a| a.into()),
            album: album.map(|a| a.into()),
            track_no: track.track_no,
            disc: track.disc,
            duration_ms: track.duration_ms,
            format: track.format,
            codec: track.codec,
            bitrate: track.bitrate,
            file_path: track.file_path,
            original_title: track.original_title,
            original_artist: track.original_artist,
            original_album: track.original_album,
            play_count,
            tags: tags.into_iter().map(Into::into).collect(),
            favorite,
            created_at: track.created_at,
        }
    }
}

#[api_type("response/track")]
#[derive(Debug, Serialize)]
struct GetTracksResponse(PaginatedResponse<TrackResponse>);
