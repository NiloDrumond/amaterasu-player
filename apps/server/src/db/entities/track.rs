use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Track {
    pub id: Uuid,
    pub album_id: Option<Uuid>,
    pub file_path: String,
    
    // Metadata - initially from file, fully editable by user
    pub title: String,
    pub artist: Option<String>,
    pub album: Option<String>,
    pub album_artist: Option<String>,
    pub disc: Option<i32>,
    pub track_no: Option<i32>,
    pub year: Option<i32>,
    pub composer: Option<String>,
    pub comment: Option<String>,
    
    // Technical metadata (read-only, from file)
    pub duration_ms: i32,
    pub format: String,
    pub bitrate: Option<i32>,
    pub sample_rate: Option<i32>,
    pub channels: Option<i32>,
    pub file_size_bytes: Option<i64>,
    pub file_modified_at: Option<DateTime<Utc>>,
    
    // Audio analysis (optional)
    pub replaygain_track_gain: Option<f32>,
    pub replaygain_album_gain: Option<f32>,
    
    // Tracking user modifications
    pub metadata_modified_at: Option<DateTime<Utc>>,
    
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Track {
    /// Create a new track from file scan
    pub fn new(file_path: String, title: String, duration_ms: i32, format: String) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            album_id: None,
            file_path,
            title,
            artist: None,
            album: None,
            album_artist: None,
            disc: None,
            track_no: None,
            year: None,
            composer: None,
            comment: None,
            duration_ms,
            format,
            bitrate: None,
            sample_rate: None,
            channels: None,
            file_size_bytes: None,
            file_modified_at: None,
            replaygain_track_gain: None,
            replaygain_album_gain: None,
            metadata_modified_at: None,
            created_at: now,
            updated_at: now,
        }
    }
    
    /// Mark that user has edited the metadata
    pub fn mark_as_user_edited(&mut self) {
        self.metadata_modified_at = Some(Utc::now());
    }
    
    /// Check if the track has been edited by user
    pub fn is_user_edited(&self) -> bool {
        self.metadata_modified_at.is_some()
    }
    
    /// Reset the user edit flag (e.g., when resetting to file metadata)
    pub fn reset_user_edit(&mut self) {
        self.metadata_modified_at = None;
    }
    
    /// Format duration as MM:SS or HH:MM:SS
    pub fn format_duration(&self) -> String {
        let total_seconds = self.duration_ms / 1000;
        let hours = total_seconds / 3600;
        let minutes = (total_seconds % 3600) / 60;
        let seconds = total_seconds % 60;
        
        if hours > 0 {
            format!("{}:{:02}:{:02}", hours, minutes, seconds)
        } else {
            format!("{}:{:02}", minutes, seconds)
        }
    }
    
    /// Get display artist (falls back to album artist if track artist is missing)
    pub fn display_artist(&self) -> Option<&str> {
        self.artist.as_deref().or(self.album_artist.as_deref())
    }
}
