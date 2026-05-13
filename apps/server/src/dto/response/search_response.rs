use amaterasu_macros::api_type;
use serde::Serialize;
use ts_rs::TS;
use uuid::Uuid;

// Custom serde rename_all conflicts with the one injected by api_type, so this
// enum opts out of the macro and declares its TS export manually.
#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq, Hash, TS)]
#[serde(rename_all = "lowercase")]
#[ts(export, export_to = "response/search/search-entity-type.ts")]
pub enum SearchEntityType {
    Track,
    Album,
    Artist,
    Playlist,
    Collection,
}

impl SearchEntityType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Track => "track",
            Self::Album => "album",
            Self::Artist => "artist",
            Self::Playlist => "playlist",
            Self::Collection => "collection",
        }
    }

    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "track" => Some(Self::Track),
            "album" => Some(Self::Album),
            "artist" => Some(Self::Artist),
            "playlist" => Some(Self::Playlist),
            "collection" => Some(Self::Collection),
            _ => None,
        }
    }
}

#[api_type("response/search")]
#[derive(Debug, Clone, Serialize)]
pub struct SearchHit {
    pub id: Uuid,
    pub kind: SearchEntityType,
    pub title: String,
    pub subtitle: Option<String>,
    pub score: f32,
}

#[api_type("response/search")]
#[derive(Debug, Clone, Serialize, Default)]
pub struct PaletteSearchResponse {
    pub tracks: Vec<SearchHit>,
    pub albums: Vec<SearchHit>,
    pub artists: Vec<SearchHit>,
    pub playlists: Vec<SearchHit>,
    pub collections: Vec<SearchHit>,
}
