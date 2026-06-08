//! Detection of folder-level cover-art images (e.g. `cover.jpg`, `folder.png`).
//!
//! Many libraries store album art as a standalone image next to the tracks
//! rather than embedding it in the file tags. The scanner uses this to fill an
//! album's cover when such a file is present.

use std::path::{Path, PathBuf};

/// Recognized basenames, highest priority first.
const COVER_STEMS: &[&str] = &["cover", "folder", "front"];
/// Recognized extensions, highest priority first. Limited to the types
/// `cover_storage::store_cover_bytes` accepts.
const COVER_EXTS: &[&str] = &["jpg", "jpeg", "png", "webp"];

/// Returns the best folder-level cover image in `folder`, if any.
///
/// Matching is case-insensitive; candidates are ranked by stem priority
/// (`cover` > `folder` > `front`) then extension order. Returns `None` when the
/// directory cannot be read or holds no recognized image.
pub fn find_folder_cover(folder: &Path) -> Option<PathBuf> {
    let entries = std::fs::read_dir(folder).ok()?;

    let mut best: Option<(usize, usize, PathBuf)> = None;
    for entry in entries.flatten() {
        let path = entry.path();
        let Some(stem) = path
            .file_stem()
            .and_then(|s| s.to_str())
            .map(|s| s.to_ascii_lowercase())
        else {
            continue;
        };
        let Some(ext) = path
            .extension()
            .and_then(|e| e.to_str())
            .map(|e| e.to_ascii_lowercase())
        else {
            continue;
        };

        let Some(stem_rank) = COVER_STEMS.iter().position(|s| *s == stem) else {
            continue;
        };
        let Some(ext_rank) = COVER_EXTS.iter().position(|e| *e == ext) else {
            continue;
        };

        let better = match &best {
            None => true,
            Some((bs, be, _)) => (stem_rank, ext_rank) < (*bs, *be),
        };
        if better {
            best = Some((stem_rank, ext_rank, path));
        }
    }

    best.map(|(_, _, path)| path)
}
