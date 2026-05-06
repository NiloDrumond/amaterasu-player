use amaterasu_macros::api_type;
use chrono::{DateTime, NaiveDate, Utc};
use serde::Serialize;
use uuid::Uuid;

use crate::db::entities::{Album, Artist, Track};

#[api_type("response/admin")]
#[derive(Debug, Serialize)]
pub struct AdminTrackResponse {
    pub id: Uuid,
    pub title: String,
    pub sort_title: String,
    pub artist_id: Option<Uuid>,
    pub album_id: Option<Uuid>,
    pub disc: Option<i32>,
    pub track_no: Option<i32>,
    pub date: Option<NaiveDate>,
    pub composer: Option<String>,
    pub comment: Option<String>,
    pub original_title: Option<String>,
    pub original_artist: Option<String>,
    pub original_album: Option<String>,
    pub file_path: String,
    pub duration_ms: i32,
    pub locked_at: Option<DateTime<Utc>>,
    pub deleted_at: Option<DateTime<Utc>>,
}

impl From<Track> for AdminTrackResponse {
    fn from(t: Track) -> Self {
        Self {
            id: t.id,
            title: t.title,
            sort_title: t.sort_title,
            artist_id: t.artist_id,
            album_id: t.album_id,
            disc: t.disc,
            track_no: t.track_no,
            date: t.date,
            composer: t.composer,
            comment: t.comment,
            original_title: t.original_title,
            original_artist: t.original_artist,
            original_album: t.original_album,
            file_path: t.file_path,
            duration_ms: t.duration_ms,
            locked_at: t.locked_at,
            deleted_at: t.deleted_at,
        }
    }
}

#[api_type("response/admin")]
#[derive(Debug, Serialize)]
pub struct AdminAlbumResponse {
    pub id: Uuid,
    pub title: String,
    pub sort_title: String,
    pub artist_id: Option<Uuid>,
    pub source_title: String,
    pub source_album_artist_id: Option<Uuid>,
    pub date: Option<NaiveDate>,
    pub cover_url: Option<String>,
    pub locked_at: Option<DateTime<Utc>>,
}

impl From<Album> for AdminAlbumResponse {
    fn from(a: Album) -> Self {
        Self {
            id: a.id,
            title: a.title,
            sort_title: a.sort_title,
            artist_id: a.artist_id,
            source_title: a.source_title,
            source_album_artist_id: a.source_album_artist_id,
            date: a.date,
            cover_url: a.cover_path.map(|p| format!("/api/covers/{p}")),
            locked_at: a.locked_at,
        }
    }
}

#[api_type("response/admin")]
#[derive(Debug, Serialize)]
pub struct AdminArtistResponse {
    pub id: Uuid,
    pub name: String,
    pub sort_name: String,
    pub source_name: String,
    pub locked_at: Option<DateTime<Utc>>,
}

impl From<Artist> for AdminArtistResponse {
    fn from(a: Artist) -> Self {
        Self {
            id: a.id,
            name: a.name,
            sort_name: a.sort_name,
            source_name: a.source_name,
            locked_at: a.locked_at,
        }
    }
}

#[api_type("response/admin")]
#[derive(Debug, Serialize)]
pub struct AdminDeletedTrackResponse {
    #[serde(flatten)]
    pub track: AdminTrackResponse,
    pub file_missing: bool,
}
