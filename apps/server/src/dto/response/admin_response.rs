use amaterasu_macros::api_type;
use chrono::{DateTime, NaiveDate, Utc};
use serde::Serialize;
use uuid::Uuid;

use crate::db::entities::{Album, Artist, Track, User};
use crate::dto::response::common_response::PaginatedResponse;
use crate::repositories::{AlbumAliasRow, ArtistAliasRow};

#[api_type("response/admin")]
#[derive(Debug, Serialize)]
pub struct ArtistAliasResponse {
    pub id: Uuid,
    pub source_name: String,
}

impl From<ArtistAliasRow> for ArtistAliasResponse {
    fn from(r: ArtistAliasRow) -> Self {
        Self {
            id: r.id,
            source_name: r.source_name,
        }
    }
}

#[api_type("response/admin")]
#[derive(Debug, Serialize)]
pub struct AlbumAliasResponse {
    pub id: Uuid,
    pub source_title: String,
    pub source_album_artist_id: Option<Uuid>,
}

impl From<AlbumAliasRow> for AlbumAliasResponse {
    fn from(r: AlbumAliasRow) -> Self {
        Self {
            id: r.id,
            source_title: r.source_title,
            source_album_artist_id: r.source_album_artist_id,
        }
    }
}

#[api_type("response/admin")]
#[derive(Debug, Serialize)]
pub struct AdminUserResponse {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub role: String,
    pub created_at: DateTime<Utc>,
}

impl From<User> for AdminUserResponse {
    fn from(u: User) -> Self {
        Self {
            id: u.id,
            name: u.name,
            email: u.email,
            role: u.role,
            created_at: u.created_at,
        }
    }
}

#[api_type("response/admin")]
#[derive(Debug, Serialize)]
struct GetUsersResponse(PaginatedResponse<AdminUserResponse>);

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
    pub approved: bool,
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
            approved: t.approved,
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
    pub date: Option<NaiveDate>,
    pub cover_path: Option<String>,
    pub cover_url: Option<String>,
    pub locked_at: Option<DateTime<Utc>>,
    pub approved: bool,
    pub aliases: Vec<AlbumAliasResponse>,
}

impl From<Album> for AdminAlbumResponse {
    fn from(a: Album) -> Self {
        Self {
            id: a.id,
            title: a.title,
            sort_title: a.sort_title,
            artist_id: a.artist_id,
            date: a.date,
            cover_url: a.cover_path.as_ref().map(|p| format!("/api/covers/{p}")),
            cover_path: a.cover_path,
            locked_at: a.locked_at,
            approved: a.approved,
            aliases: Vec::new(),
        }
    }
}

impl AdminAlbumResponse {
    pub fn with_aliases(mut self, aliases: Vec<AlbumAliasRow>) -> Self {
        self.aliases = aliases.into_iter().map(Into::into).collect();
        self
    }
}

#[api_type("response/admin")]
#[derive(Debug, Serialize)]
pub struct AdminArtistResponse {
    pub id: Uuid,
    pub name: String,
    pub sort_name: String,
    pub locked_at: Option<DateTime<Utc>>,
    pub approved: bool,
    pub aliases: Vec<ArtistAliasResponse>,
}

impl From<Artist> for AdminArtistResponse {
    fn from(a: Artist) -> Self {
        Self {
            id: a.id,
            name: a.name,
            sort_name: a.sort_name,
            locked_at: a.locked_at,
            approved: a.approved,
            aliases: Vec::new(),
        }
    }
}

impl AdminArtistResponse {
    pub fn with_aliases(mut self, aliases: Vec<ArtistAliasRow>) -> Self {
        self.aliases = aliases.into_iter().map(Into::into).collect();
        self
    }
}

#[api_type("response/admin")]
#[derive(Debug, Serialize)]
pub struct AdminDeletedTrackResponse {
    #[serde(flatten)]
    pub track: AdminTrackResponse,
    pub file_missing: bool,
}

#[api_type("response/admin")]
#[derive(Debug, Serialize)]
pub struct ReviewQueueAlbumGroup {
    pub album: AdminAlbumResponse,
    pub artist: Option<AdminArtistResponse>,
    pub tracks: Vec<AdminTrackResponse>,
    pub track_artists: Vec<AdminArtistResponse>,
}

#[api_type("response/admin")]
#[derive(Debug, Serialize)]
pub struct ReviewQueueStandaloneArtist {
    pub artist: AdminArtistResponse,
    pub tracks: Vec<AdminTrackResponse>,
    pub track_albums: Vec<AdminAlbumResponse>,
}

#[api_type("response/admin")]
#[derive(Debug, Serialize)]
pub struct ReviewQueueCounts {
    pub pending_albums: i64,
    pub pending_tracks: i64,
    pub pending_artists: i64,
}

#[api_type("response/admin")]
#[derive(Debug, Serialize)]
pub struct ReviewQueueResponse {
    pub albums: Vec<ReviewQueueAlbumGroup>,
    pub standalone_artists: Vec<ReviewQueueStandaloneArtist>,
    pub counts: ReviewQueueCounts,
}

// ===================================================================
// MusicBrainz metadata suggestions
// ===================================================================

#[api_type("response/admin")]
#[derive(Debug, Serialize)]
pub struct MetadataSuggestionResponse {
    pub id: Uuid,
    pub entity_type: String,
    pub entity_id: Uuid,
    pub source: String,
    pub mbid: String,
    pub score: i32,
    pub rank: i32,
    /// Per-entity-type payload. See `musicbrainz::mapping` for the shape:
    /// `AlbumProposal` for album suggestions, `ArtistProposal` for artists,
    /// `TrackProposal` for tracks. The frontend discriminates on `entityType`.
    #[ts(type = "Record<string, unknown>")]
    pub proposed: serde_json::Value,
    pub status: String,
    pub created_at: DateTime<Utc>,
}

impl From<crate::repositories::MetadataSuggestion> for MetadataSuggestionResponse {
    fn from(s: crate::repositories::MetadataSuggestion) -> Self {
        Self {
            id: s.id,
            entity_type: s.entity_type,
            entity_id: s.entity_id,
            source: s.source,
            mbid: s.mbid,
            score: s.score as i32,
            rank: s.rank as i32,
            proposed: s.proposed,
            status: s.status,
            created_at: s.created_at,
        }
    }
}

#[api_type("response/admin")]
#[derive(Debug, Serialize)]
pub struct BulkMbLookupResponse {
    pub enqueued: i64,
}
