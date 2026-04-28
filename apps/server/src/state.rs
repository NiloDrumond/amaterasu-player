use std::path::PathBuf;

use axum::extract::FromRef;
use sqlx::PgPool;

use crate::scanner::LibraryScanner;

#[derive(Clone, FromRef)]
pub struct AppState {
    pub db: PgPool,
    pub library_scanner: LibraryScanner,
    pub covers_dir: PathBuf,
}

impl FromRef<AppState> for () {
    fn from_ref(_: &AppState) {}
}

impl AppState {
    pub fn new(db: PgPool, library_scanner: LibraryScanner, covers_dir: PathBuf) -> Self {
        Self {
            db,
            library_scanner,
            covers_dir,
        }
    }
}
