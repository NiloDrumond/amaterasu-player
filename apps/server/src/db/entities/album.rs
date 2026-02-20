use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, Default)]
pub struct Album {
    pub id: Uuid,
    pub artist_id: Option<Uuid>,
    pub title: String,
    pub sort_title: String,
    pub date: Option<NaiveDate>,
    pub mbid: Option<String>, // MusicBrainz ID
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,

    pub replaygain_album_gain: Option<f32>,
    pub replaygain_album_peak: Option<f32>,
}
