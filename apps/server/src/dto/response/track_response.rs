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
    pub format: String,
    pub bitrate: Option<i32>,
    pub file_path: String,
}

impl From<Track> for TrackResponse {
    fn from(track: Track) -> Self {
        Self {
            id: track.id,
            title: track.title,
            artist: track.artist,
            album: track.album,
            track_no: track.track_no,
            disc: track.disc,
            duration_ms: track.duration_ms,
            format: track.format,
            bitrate: track.bitrate,
            file_path: track.file_path,
        }
    }
}
