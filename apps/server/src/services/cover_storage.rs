//! Shared helpers for persisting album cover-art bytes into `covers_dir`.
//!
//! Used by both the MB/CAA download path (`musicbrainz::service`) and the
//! admin manual-upload endpoint. Files are named by the sha256 of their
//! contents so the same image, no matter where it came from, dedupes on
//! disk.

use std::path::Path;

use sha2::{Digest, Sha256};

/// Maximum size of an uploaded cover. Larger uploads are rejected with
/// `CoverStorageError::TooLarge`. 10 MB is comfortably above album art served
/// by the Cover Art Archive (typically <500 KB) and any reasonable scan.
pub const COVER_MAX_BYTES: usize = 10 * 1024 * 1024;

#[derive(Debug, thiserror::Error)]
pub enum CoverStorageError {
    #[error("unsupported image type (only JPEG, PNG, and WebP are accepted)")]
    UnsupportedImageType,
    #[error("file too large ({0} bytes, max {COVER_MAX_BYTES})")]
    TooLarge(usize),
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
}

/// Detects the image type and returns the file extension we'll use on disk.
/// Returns `None` for non-image bytes or types we don't accept.
pub fn detect_image_ext(bytes: &[u8]) -> Option<&'static str> {
    infer::get(bytes).and_then(|t| match t.mime_type() {
        "image/jpeg" => Some("jpg"),
        "image/png" => Some("png"),
        "image/webp" => Some("webp"),
        _ => None,
    })
}

/// Writes `bytes` to `covers_dir/<sha256>.<ext>` and returns the basename.
/// Idempotent: if the file already exists (same hash), no write is performed.
///
/// Returns `CoverStorageError::UnsupportedImageType` if `bytes` isn't a
/// recognizable image and `CoverStorageError::TooLarge` if it exceeds
/// `COVER_MAX_BYTES`.
pub async fn store_cover_bytes(
    covers_dir: &Path,
    bytes: &[u8],
) -> Result<String, CoverStorageError> {
    if bytes.len() > COVER_MAX_BYTES {
        return Err(CoverStorageError::TooLarge(bytes.len()));
    }
    let ext = detect_image_ext(bytes).ok_or(CoverStorageError::UnsupportedImageType)?;

    let mut hasher = Sha256::new();
    hasher.update(bytes);
    let hash = hex::encode(hasher.finalize());
    let filename = format!("{}.{}", hash, ext);
    let path = covers_dir.join(&filename);
    if !path.exists() {
        tokio::fs::write(&path, bytes).await?;
    }
    Ok(filename)
}
