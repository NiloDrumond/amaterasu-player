use amaterasu_macros::api_type;
use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

use crate::filters::FilterNode;
use crate::repositories::playlist_repository::{PlaylistStats, PlaylistTrackRow};

#[api_type("response/playlist")]
#[derive(Debug, Serialize)]
pub struct PlaylistResponse {
    pub id: Uuid,
    pub name: String,
    pub track_count: i64,
    pub total_duration_ms: i64,
    pub created_at: DateTime<Utc>,
    /// 'manual' or 'dynamic'.
    pub playlist_type: String,
    pub filter_definition: Option<FilterNode>,
}

impl From<PlaylistStats> for PlaylistResponse {
    fn from(value: PlaylistStats) -> Self {
        Self {
            id: value.playlist.id,
            name: value.playlist.name,
            track_count: value.track_count,
            total_duration_ms: value.total_duration_ms,
            created_at: value.playlist.created_at,
            playlist_type: value.playlist.playlist_type,
            filter_definition: value.playlist.filter_definition.map(|j| j.0),
        }
    }
}

#[api_type("response/playlist")]
#[derive(Debug, Serialize)]
pub struct PlaylistTrackArtistResponse {
    pub id: Uuid,
    pub name: String,
}

#[api_type("response/playlist")]
#[derive(Debug, Serialize)]
pub struct PlaylistTrackAlbumResponse {
    pub id: Uuid,
    pub title: String,
    pub cover_url: Option<String>,
}

#[api_type("response/playlist")]
#[derive(Debug, Serialize)]
pub struct PlaylistTrackResponse {
    pub id: Uuid,
    pub title: String,
    pub artist: Option<PlaylistTrackArtistResponse>,
    pub album: Option<PlaylistTrackAlbumResponse>,
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
    // Playlist-specific fields
    pub playlist_track_id: Uuid,
    pub position: f64,
    pub added_at: DateTime<Utc>,
}

impl From<PlaylistTrackRow> for PlaylistTrackResponse {
    fn from(r: PlaylistTrackRow) -> Self {
        let artist = r
            .artist_id
            .zip(r.artist_name)
            .map(|(id, name)| PlaylistTrackArtistResponse { id, name });

        let album = r
            .album_id
            .zip(r.album_title)
            .map(|(id, title)| PlaylistTrackAlbumResponse {
                id,
                title,
                cover_url: r.album_cover_path.map(|p| format!("/api/covers/{p}")),
            });

        Self {
            id: r.track_id,
            title: r.title,
            artist,
            album,
            track_no: r.track_no,
            disc: r.disc,
            duration_ms: r.duration_ms,
            format: r.format,
            codec: r.codec,
            bitrate: r.bitrate,
            file_path: r.file_path,
            original_title: r.original_title,
            original_artist: r.original_artist,
            original_album: r.original_album,
            playlist_track_id: r.playlist_track_id,
            position: r.position,
            added_at: r.added_at,
        }
    }
}
