use std::collections::BTreeMap;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use sqlx::PgPool;
use walkdir::{DirEntry, WalkDir};

const AUDIO_EXTENSIONS: &[&str] = &[
    "mp3", "flac", "wav", "ogg", "oga", "opus", "m4a", "m4b", "aac", "wma", "aiff", "aif", "alac",
    "ape", "wv",
];

fn is_audio_file(path: &std::path::Path) -> bool {
    path.extension()
        .and_then(|e| e.to_str())
        .map(|e| e.to_ascii_lowercase())
        .map(|e| AUDIO_EXTENSIONS.contains(&e.as_str()))
        .unwrap_or(false)
}

fn is_system_junk(entry: &DirEntry) -> bool {
    let name = entry.file_name().to_string_lossy();
    if name == "__MACOSX" || name == ".DS_Store" || name == "Thumbs.db" {
        return true;
    }
    name.starts_with("._")
}

use crate::scanner::persist::persist_scanned_file;
use crate::scanner::scan_file::ScannedFile;
use crate::scanner::scan_track::parse_numeric_prefix;

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

        let mut by_folder: BTreeMap<PathBuf, Vec<PathBuf>> = BTreeMap::new();
        let walker = WalkDir::new(&self.library_path)
            .into_iter()
            .filter_entry(|e| !is_system_junk(e));
        for entry in walker {
            let entry = entry?;
            if !entry.file_type().is_file() {
                continue;
            }
            if !is_audio_file(entry.path()) {
                continue;
            }
            let path = entry.path().to_path_buf();
            let parent = path
                .parent()
                .map(|p| p.to_path_buf())
                .unwrap_or_else(|| PathBuf::from("."));
            by_folder.entry(parent).or_default().push(path);
        }

        for (_parent, files) in by_folder {
            let mut scanned_files: Vec<ScannedFile> = Vec::with_capacity(files.len());
            for path in &files {
                match ScannedFile::scan(path, &self.library_path) {
                    Ok(f) => scanned_files.push(f),
                    Err(e) => {
                        tracing::warn!("Failed to scan {}: {}", path.display(), e);
                        failed += 1;
                    }
                }
            }

            let album_gate = scanned_files.len() >= 2
                && scanned_files
                    .iter()
                    .all(|f| f.track_metadata.track_no.is_none())
                && scanned_files
                    .iter()
                    .all(|f| parse_numeric_prefix(f.file_stem()).is_some());

            if album_gate {
                for f in &mut scanned_files {
                    let stem = f.file_stem().to_string();
                    f.track_metadata.apply_filename_track_number(&stem);
                }
            }

            for scanned_file in scanned_files {
                let display_path = scanned_file.file_path().to_string();
                match persist_scanned_file(&self.pool, &self.covers_dir, scanned_file).await {
                    Ok(Some(_)) => scanned += 1,
                    Ok(None) => {}
                    Err(e) => {
                        tracing::warn!("Failed to persist {}: {}", display_path, e);
                        failed += 1;
                    }
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
