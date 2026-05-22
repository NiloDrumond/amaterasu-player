use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Album {
    pub id: Uuid,
    pub artist_id: Option<Uuid>,
    pub title: String,
    pub sort_title: String,
    pub date: Option<NaiveDate>,
    pub mbid: Option<String>, // MusicBrainz ID
    pub cover_path: Option<String>,
    pub locked_at: Option<DateTime<Utc>>,
    pub approved: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,

    pub replaygain_album_gain: Option<f32>,
    pub replaygain_album_peak: Option<f32>,

    pub mb_lookup_status: Option<String>,
    pub mb_lookup_attempted_at: Option<DateTime<Utc>>,
}

impl Album {
    pub fn new(
        artist_id: Option<Uuid>,
        title: String,
        sort_title: String,
        date: Option<NaiveDate>,
        replaygain_album_gain: Option<f32>,
        replaygain_album_peak: Option<f32>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            artist_id,
            title,
            sort_title,
            date,
            mbid: None,
            cover_path: None,
            locked_at: None,
            approved: false,
            replaygain_album_gain,
            replaygain_album_peak,
            mb_lookup_status: None,
            mb_lookup_attempted_at: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}
