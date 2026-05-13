use std::path::PathBuf;
use std::sync::Arc;

use axum::extract::FromRef;
use sqlx::PgPool;

use crate::scanner::LibraryScanner;
use crate::search::SearchIndex;

#[derive(Clone, FromRef)]
pub struct AppState {
    pub db: PgPool,
    pub library_scanner: LibraryScanner,
    pub covers_dir: PathBuf,
    pub search: Arc<SearchIndex>,
}

impl FromRef<AppState> for () {
    fn from_ref(_: &AppState) {}
}

impl AppState {
    pub fn new(
        db: PgPool,
        library_scanner: LibraryScanner,
        covers_dir: PathBuf,
        search: Arc<SearchIndex>,
    ) -> Self {
        Self {
            db,
            library_scanner,
            covers_dir,
            search,
        }
    }
}
