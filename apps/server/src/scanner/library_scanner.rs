use std::collections::{BTreeMap, HashSet};
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use tokio::sync::Semaphore;
use tokio::task::JoinSet;
use uuid::Uuid;

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

use crate::musicbrainz::{LookupJob, LookupSender};
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
    /// Optional sender to the MB lookup worker. `None` when MB is disabled;
    /// the scanner runs the same regardless and just skips enqueuing.
    mb_lookup_sender: Option<LookupSender>,
}

impl LibraryScanner {
    pub fn new(library_path: String, covers_dir: PathBuf, pool: PgPool) -> Self {
        Self {
            library_path,
            covers_dir,
            pool,
            scanning: Arc::new(AtomicBool::new(false)),
            mb_lookup_sender: None,
        }
    }

    /// Builder hook called from `main.rs` once the MB worker is spawned.
    pub fn with_mb_lookup_sender(mut self, sender: LookupSender) -> Self {
        self.mb_lookup_sender = Some(sender);
        self
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
        ffmpeg_next::init()?;

        let mut scanned: u64 = 0;
        let mut failed: u64 = 0;
        let mut enqueued_albums: HashSet<Uuid> = HashSet::new();
        let mut enqueued_artists: HashSet<Uuid> = HashSet::new();

        // Phase 1: Walk & collect files grouped by folder.
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

        // Phase 2: Scan files in parallel across all folders.
        let max_parallel = std::thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(4);
        let semaphore = Arc::new(Semaphore::new(max_parallel));
        let mut join_set = JoinSet::new();

        for (parent, files) in &by_folder {
            for path in files {
                let permit = semaphore.clone().acquire_owned().await.unwrap();
                let path = path.clone();
                let parent = parent.clone();
                let library_path = self.library_path.clone();
                join_set.spawn_blocking(move || {
                    let _permit = permit;
                    let result = ScannedFile::scan(&path, &library_path);
                    (parent, path, result)
                });
            }
        }

        let mut scanned_by_folder: BTreeMap<PathBuf, Vec<ScannedFile>> = BTreeMap::new();
        while let Some(res) = join_set.join_next().await {
            match res {
                Ok((parent, _, Ok(f))) => {
                    scanned_by_folder.entry(parent).or_default().push(f);
                }
                Ok((_, path, Err(e))) => {
                    tracing::warn!("Failed to scan {}: {}", path.display(), e);
                    failed += 1;
                }
                Err(e) => {
                    tracing::warn!("Scan task panicked: {}", e);
                    failed += 1;
                }
            }
        }

        // Phase 3: Album gate + persist (sequential, per folder).
        for (_parent, mut scanned_files) in scanned_by_folder {
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
                    Ok(Some(track)) => {
                        scanned += 1;
                        if let Some(sender) = &self.mb_lookup_sender {
                            if let Some(album_id) = track.album_id {
                                if enqueued_albums.insert(album_id) {
                                    let _ = sender.enqueue(LookupJob::Album {
                                        id: album_id,
                                        force: false,
                                    });
                                }
                            }
                            if let Some(artist_id) = track.artist_id {
                                if enqueued_artists.insert(artist_id) {
                                    let _ = sender.enqueue(LookupJob::Artist {
                                        id: artist_id,
                                        force: false,
                                    });
                                }
                            }
                        }
                    }
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
