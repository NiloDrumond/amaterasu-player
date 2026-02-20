use sqlx::PgPool;
use walkdir::WalkDir;

use crate::scanner::persist::persist_scanned_file;
use crate::scanner::scan_file::ScannedFile;

#[derive(Clone)]
pub struct LibraryScanner {
    library_path: String,
    pool: PgPool,
}

impl LibraryScanner {
    pub fn new(library_path: String, pool: PgPool) -> Self {
        Self {
            library_path,
            pool,
        }
    }

    pub async fn scan_library(&self) -> Result<(), anyhow::Error> {
        let mut scanned: u64 = 0;
        let mut failed: u64 = 0;

        for entry in WalkDir::new(&self.library_path) {
            let entry = entry?;
            if !entry.file_type().is_file() {
                continue;
            }

            let path = entry.path();
            let scanned_file = match ScannedFile::scan(path) {
                Ok(f) => f,
                Err(e) => {
                    tracing::warn!("Failed to scan {}: {}", path.display(), e);
                    failed += 1;
                    continue;
                }
            };

            match persist_scanned_file(&self.pool, scanned_file).await {
                Ok(_) => scanned += 1,
                Err(e) => {
                    tracing::warn!("Failed to persist {}: {}", path.display(), e);
                    failed += 1;
                }
            }
        }

        tracing::info!(
            "Scan complete: {} files scanned, {} failed",
            scanned,
            failed
        );
        Ok(())
    }
}
