use serde::Serialize;
use uuid::Uuid;

use crate::db::entities::Track;

#[derive(Debug, Serialize)]
pub struct TrackResponse {
    pub id: Uuid,
    pub title: String,
    pub artist: Option<String>,
    pub album: Option<String>,
    pub track_no: Option<i32>,
    pub disc: Option<i32>,
    pub duration_ms: i32,
    pub codec: String,
    pub bitrate: Option<i32>,
    pub file_path: String,
    pub original_title: Option<String>,
    pub original_artist: Option<String>,
    pub original_album: Option<String>,
}

impl From<Track> for TrackResponse {
    fn from(track: Track) -> Self {
        Self {
            id: track.id,
            title: track.title,
            artist: Some("TODO".to_string()),
            album: Some("TODO".to_string()),
            track_no: track.track_no,
            disc: track.disc,
            duration_ms: track.duration_ms,
            // TODO: Convert codec to string
            codec: "TODO".to_string(),
            bitrate: track.bitrate,
            file_path: track.file_path,
            original_title: track.original_title,
            original_artist: track.original_artist,
            original_album: track.original_album,
        }
    }
}
