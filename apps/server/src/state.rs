use sqlx::PgPool;
use std::sync::Arc;

use crate::scanner::LibraryScanner;

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub library_scanner: LibraryScanner,
}

impl AppState {
    pub fn new(db: PgPool, library_scanner: LibraryScanner) -> Arc<Self> {
        Arc::new(Self {
            db,
            library_scanner,
        })
    }
}
