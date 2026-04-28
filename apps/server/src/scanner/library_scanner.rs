use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use sqlx::PgPool;
use walkdir::WalkDir;

use crate::scanner::persist::persist_scanned_file;
use crate::scanner::scan_file::ScannedFile;

pub struct ScanPermit {
    flag: Arc<AtomicBool>,
}

impl Drop for ScanPermit {
    fn drop(&mut self) {
        self.flag.store(false, Ordering::Release);
    }
}

#[derive(Clone)]
pub struct LibraryScanner {
    library_path: String,
    covers_dir: PathBuf,
    pool: PgPool,
    scanning: Arc<AtomicBool>,
}

impl LibraryScanner {
    pub fn new(library_path: String, covers_dir: PathBuf, pool: PgPool) -> Self {
        Self {
            library_path,
            covers_dir,
            pool,
            scanning: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn try_acquire_scan(&self) -> Option<ScanPermit> {
        if self.scanning.swap(true, Ordering::AcqRel) {
            None
        } else {
            Some(ScanPermit {
                flag: Arc::clone(&self.scanning),
            })
        }
    }

    pub async fn scan_library(&self) -> Result<(), anyhow::Error> {
        let Some(permit) = self.try_acquire_scan() else {
            anyhow::bail!("A library scan is already in progress");
        };
        self.run_scan(permit).await
    }

    pub async fn run_scan(&self, _permit: ScanPermit) -> Result<(), anyhow::Error> {
        let mut scanned: u64 = 0;
        let mut failed: u64 = 0;

        for entry in WalkDir::new(&self.library_path) {
            let entry = entry?;
            if !entry.file_type().is_file() {
                continue;
            }

            let path = entry.path();
            let scanned_file = match ScannedFile::scan(path, &self.library_path) {
                Ok(f) => f,
                Err(e) => {
                    tracing::warn!("Failed to scan {}: {}", path.display(), e);
                    failed += 1;
                    continue;
                }
            };

            match persist_scanned_file(&self.pool, &self.covers_dir, scanned_file).await {
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
