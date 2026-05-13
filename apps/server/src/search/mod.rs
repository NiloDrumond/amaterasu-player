//! In-process Tantivy search index over the music catalog.
//!
//! The index is rebuilt from Postgres on startup and updated write-through
//! by handlers on catalog mutations. Index writes are best-effort: on failure
//! we log and continue, since the next restart will heal via full rebuild.

mod index;
pub mod indexers;
mod query;
pub mod sync;

#[allow(unused_imports)]
pub use index::{IndexDoc, SearchIndex};
#[allow(unused_imports)]
pub use query::SearchQueryOptions;

use crate::dto::response::SearchEntityType;

impl SearchEntityType {
    pub(crate) fn all() -> [SearchEntityType; 5] {
        [
            SearchEntityType::Track,
            SearchEntityType::Album,
            SearchEntityType::Artist,
            SearchEntityType::Playlist,
            SearchEntityType::Collection,
        ]
    }
}
