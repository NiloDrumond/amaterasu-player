//! Subset of MusicBrainz / Cover Art Archive JSON shapes that we read. We only
//! deserialize the fields actually consumed by `mapping.rs` so future MB
//! response changes don't break us.

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ReleaseGroupSearchResponse {
    #[serde(default, rename = "release-groups")]
    pub release_groups: Vec<ReleaseGroup>,
}

#[derive(Debug, Deserialize)]
pub struct ReleaseGroup {
    pub id: String,
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub score: i32,
    #[serde(default, rename = "primary-type")]
    pub primary_type: Option<String>,
    #[serde(default, rename = "first-release-date")]
    pub first_release_date: Option<String>,
    #[serde(default, rename = "artist-credit")]
    pub artist_credit: Vec<ArtistCredit>,
    #[serde(default)]
    pub releases: Vec<ReleaseSummary>,
}

#[derive(Debug, Deserialize)]
pub struct ReleaseSummary {
    pub id: String,
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub date: Option<String>,
    #[serde(default)]
    pub country: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ArtistCredit {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub joinphrase: Option<String>,
    pub artist: Option<ArtistRef>,
}

#[derive(Debug, Deserialize)]
pub struct ArtistRef {
    pub id: String,
    #[serde(default)]
    pub name: String,
    #[serde(default, rename = "sort-name")]
    pub sort_name: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ArtistSearchResponse {
    #[serde(default)]
    pub artists: Vec<Artist>,
}

#[derive(Debug, Deserialize)]
pub struct Artist {
    pub id: String,
    #[serde(default)]
    pub name: String,
    #[serde(default, rename = "sort-name")]
    pub sort_name: Option<String>,
    #[serde(default)]
    pub score: i32,
    #[serde(default, rename = "type")]
    pub artist_type: Option<String>,
    #[serde(default)]
    pub country: Option<String>,
    #[serde(default)]
    pub disambiguation: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct RecordingSearchResponse {
    #[serde(default)]
    pub recordings: Vec<Recording>,
}

#[derive(Debug, Deserialize)]
pub struct Recording {
    pub id: String,
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub score: i32,
    #[serde(default)]
    pub length: Option<i64>,
    #[serde(default, rename = "artist-credit")]
    pub artist_credit: Vec<ArtistCredit>,
    #[serde(default)]
    pub releases: Vec<ReleaseSummary>,
}
