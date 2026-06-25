use uuid::Uuid;

use crate::dto::request::SortDir;

pub struct FindParams<S> {
    pub limit: i32,
    pub offset: i32,
    pub sort: Option<S>,
    pub dir: Option<SortDir>,
    pub seed: Option<i64>,
    pub user_id: Option<Uuid>,
}

pub mod album_collection_repository;
pub mod album_repository;
pub mod alias_repository;
pub mod artist_repository;
pub mod metadata_suggestion_repository;
pub mod pinned_playlist_repository;
pub mod playlist_repository;
pub mod session_repository;
pub mod tag_category_repository;
pub mod tag_repository;
pub mod track_favorite_repository;
pub mod track_play_repository;
pub mod track_repository;
pub mod user_repository;

pub use album_collection_repository::AlbumCollectionRepository;
pub use album_repository::{AlbumRepository, AlbumSortKey};
pub use alias_repository::{AlbumAliasRow, AliasRepository, ArtistAliasRow};
pub use artist_repository::{ArtistRepository, ArtistSortKey};
pub use metadata_suggestion_repository::{
    MbLookupStatusRepository, MetadataSuggestion, MetadataSuggestionRepository, NewSuggestion,
    SuggestionEntityType,
};
pub use pinned_playlist_repository::{PinnedPlaylistRepository, MAX_PINNED_PLAYLISTS};
pub use playlist_repository::{PlaylistRepository, PlaylistSortKey};
pub use session_repository::SessionRepository;
pub use tag_category_repository::TagCategoryRepository;
pub use tag_repository::TagRepository;
pub use track_favorite_repository::TrackFavoriteRepository;
pub use track_play_repository::TrackPlayRepository;
pub use track_repository::{TrackRepository, TrackSortKey};
pub use user_repository::UserRepository;
