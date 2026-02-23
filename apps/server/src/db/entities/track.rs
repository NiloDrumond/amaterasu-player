use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MetadataValue {
    Binary(Box<[u8]>),
    Boolean(bool),
    Flag,
    Float(f64),
    SignedInt(i64),
    String(String),
    UnsignedInt(u64),
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, Default)]
pub struct Track {
    pub id: Uuid,
    pub audio_hash: Vec<u8>,
    pub album_id: Option<Uuid>,
    pub file_path: String,

    pub title: String,
    pub sort_title: String,
    pub artist_id: Option<Uuid>,
    pub disc: Option<i32>,     // CHECK: >= 0
    pub track_no: Option<i32>, // CHECK: >= 0
    pub date: Option<NaiveDate>,
    pub composer: Option<String>,
    pub comment: Option<String>,
    pub original_title: Option<String>,
    pub original_artist: Option<String>,
    pub original_album: Option<String>,

    // TODO: symphonia 6.0 CodecDescriptor
    // pub codec: i64,                   // CHECK: >= 0
    pub duration_ms: i32,             // CHECK: >= 0
    pub bitrate: Option<i32>,         // CHECK: >= 0
    pub sample_rate: Option<i64>,     // CHECK: >= 0
    pub channels: Option<i32>,        // CHECK: >= 0
    pub file_size_bytes: Option<i64>, // CHECK: >= 0
    pub file_modified_at: Option<DateTime<Utc>>,
    pub replaygain_track_gain: Option<f32>,
    pub replaygain_track_peak: Option<f32>,
    pub metadata_modified_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
