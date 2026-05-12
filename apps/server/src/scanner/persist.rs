use std::path::Path;

use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

use crate::db::entities::{Album, Artist, Track};
use crate::error::AppError;
use crate::repositories::{AlbumRepository, ArtistRepository, TrackRepository};

use super::scan_artist::ScannedArtistMetadata;
use super::scan_cover::ScannedCover;
use super::scan_file::ScannedFile;
use super::track_quality::quality_rank;

pub async fn persist_scanned_file(
    pool: &PgPool,
    covers_dir: &Path,
    scanned: ScannedFile,
) -> Result<Option<Track>, AppError> {
    let mut tx = pool.begin().await?;

    let album_artist_id =
        find_or_create_artist(&mut tx, &scanned.album_metadata.album_artist_metadata)
            .await?
            .map(|a| a.id);

    let album = find_or_create_album(&mut tx, &scanned, album_artist_id).await?;

    if let Some(cover) = &scanned.cover {
        if album.cover_path.is_none() {
            persist_album_cover(&mut tx, covers_dir, album.id, cover).await?;
        }
    }

    let track_artist_id = find_or_create_artist(&mut tx, &scanned.artist_metadata)
        .await?
        .map(|a| a.id);

    let track = upsert_track(&mut tx, scanned, album.id, track_artist_id).await?;

    tx.commit().await?;

    Ok(track)
}

fn apply_scanned_fields(
    track: &mut Track,
    scanned: ScannedFile,
    album_id: Uuid,
    artist_id: Option<Uuid>,
    audio_hash: Vec<u8>,
    sort_title: String,
) {
    track.file_path = scanned.file_path;
    track.album_id = Some(album_id);
    track.artist_id = artist_id;
    track.audio_hash = audio_hash;
    track.title = scanned.track_metadata.title;
    track.sort_title = sort_title;
    track.disc = scanned.track_metadata.disc;
    track.track_no = scanned.track_metadata.track_no;
    track.date = scanned.track_metadata.date;
    track.composer = scanned.track_metadata.composer;
    track.comment = scanned.track_metadata.comment;
    track.original_title = scanned.track_metadata.original_title;
    track.original_artist = scanned.track_metadata.original_artist;
    track.original_album = scanned.track_metadata.original_album;
    track.format = scanned.audio.format;
    track.codec = scanned.audio.codec;
    track.duration_ms = scanned.audio.duration_ms;
    track.bitrate = scanned.audio.bitrate;
    track.sample_rate = scanned.audio.sample_rate;
    track.file_size_bytes = scanned.audio.file_size_bytes;
    track.replaygain_track_gain = scanned.track_metadata.replaygain_track_gain;
    track.replaygain_track_peak = scanned.track_metadata.replaygain_track_peak;
}

async fn persist_album_cover(
    tx: &mut sqlx::PgConnection,
    covers_dir: &Path,
    album_id: Uuid,
    cover: &ScannedCover,
) -> Result<(), AppError> {
    let filename = format!("{}.{}", hex::encode(cover.hash), cover.ext);
    let path = covers_dir.join(&filename);
    if !path.exists() {
        tokio::fs::write(&path, &cover.bytes)
            .await
            .map_err(|e| AppError::Internal(anyhow::anyhow!("failed to write cover: {}", e)))?;
    }
    AlbumRepository::update_cover_path(&mut *tx, album_id, &filename).await?;
    Ok(())
}

async fn find_or_create_artist(
    tx: &mut sqlx::PgConnection,
    metadata: &ScannedArtistMetadata,
) -> Result<Option<Artist>, AppError> {
    let name = match &metadata.name {
        Some(name) => name,
        None => return Ok(None),
    };

    // Match by source_name so admin renames don't cause duplicates.
    if let Some(artist) = ArtistRepository::find_by_source_name(&mut *tx, name).await? {
        return Ok(Some(artist));
    }

    let sort_name = metadata.sort_name.as_deref().unwrap_or(name.as_str());
    let artist = Artist {
        id: Uuid::new_v4(),
        name: name.to_string(),
        sort_name: sort_name.to_string(),
        mbid: None,
        source_name: name.to_string(),
        locked_at: None,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };
    ArtistRepository::create(&mut *tx, &artist).await.map(Some)
}

async fn find_or_create_album(
    tx: &mut sqlx::PgConnection,
    scanned: &ScannedFile,
    album_artist_id: Option<Uuid>,
) -> Result<Album, AppError> {
    let title = &scanned.album_metadata.name;
    if let Some(album) =
        AlbumRepository::find_by_source_keys(&mut *tx, title, album_artist_id).await?
    {
        return Ok(album);
    }

    let sort_title = scanned
        .album_metadata
        .sort_name
        .as_deref()
        .unwrap_or(title.as_str());

    let album = Album::new(
        album_artist_id,
        title.clone(),
        sort_title.to_string(),
        scanned.album_metadata.date,
        scanned.album_metadata.replaygain_album_gain,
        scanned.album_metadata.replaygain_album_peak,
    );
    AlbumRepository::create(&mut *tx, &album).await
}

async fn upsert_track(
    tx: &mut sqlx::PgConnection,
    scanned: ScannedFile,
    album_id: Uuid,
    artist_id: Option<Uuid>,
) -> Result<Option<Track>, AppError> {
    let audio_hash = scanned.audio.audio_hash.to_vec();
    let existing = TrackRepository::find_by_audio_hash(&mut *tx, &audio_hash).await?;

    let sort_title = scanned
        .track_metadata
        .sort_title
        .clone()
        .unwrap_or_else(|| scanned.track_metadata.title.clone());

    if let Some(mut track) = existing {
        if track.file_path != scanned.file_path {
            let old_name = std::path::Path::new(&track.file_path).file_name();
            let new_name = std::path::Path::new(&scanned.file_path).file_name();

            if old_name != new_name {
                tracing::error!(
                    track_id = %track.id,
                    old_path = %track.file_path,
                    new_path = %scanned.file_path,
                    "Possible hash collision: same audio_hash but different file name, skipping"
                );
                return Err(AppError::Internal(anyhow::anyhow!(
                    "Possible hash collision: '{}' and '{}' share the same audio hash",
                    track.file_path,
                    scanned.file_path
                )));
            }

            tracing::info!(
                track_id = %track.id,
                old_path = %track.file_path,
                new_path = %scanned.file_path,
                "Track file moved"
            );
        }

        // Locked or soft-deleted tracks survive rescans untouched, except for
        // file_path so file moves still resolve.
        if track.locked_at.is_some() || track.deleted_at.is_some() {
            if track.file_path != scanned.file_path {
                TrackRepository::update_file_path(&mut *tx, track.id, &scanned.file_path).await?;
                track.file_path = scanned.file_path;
            }
            return Ok(Some(track));
        }

        apply_scanned_fields(
            &mut track, scanned, album_id, artist_id, audio_hash, sort_title,
        );
        return TrackRepository::update(&mut *tx, &track).await.map(Some);
    }

    // No audio_hash match. Check whether the same logical track exists in another format.
    if let Some(mut existing) =
        TrackRepository::find_by_album_and_title(&mut *tx, album_id, &scanned.track_metadata.title)
            .await?
    {
        if existing.locked_at.is_some() || existing.deleted_at.is_some() {
            tracing::info!(
                existing_path = %existing.file_path,
                skipped_path = %scanned.file_path,
                "Existing track is locked/deleted; skipping alternate-format version"
            );
            return Ok(None);
        }

        let new_rank = quality_rank(
            &scanned.audio.format,
            scanned.audio.bitrate,
            scanned.audio.sample_rate,
        );
        let existing_rank = quality_rank(&existing.format, existing.bitrate, existing.sample_rate);

        if new_rank > existing_rank {
            tracing::info!(
                replaced_path = %existing.file_path,
                with_path = %scanned.file_path,
                "Replacing lower-quality track with higher-quality version"
            );
            apply_scanned_fields(
                &mut existing,
                scanned,
                album_id,
                artist_id,
                audio_hash,
                sort_title,
            );
            return TrackRepository::update(&mut *tx, &existing).await.map(Some);
        } else {
            tracing::info!(
                kept_path = %existing.file_path,
                skipped_path = %scanned.file_path,
                "Skipped lower-quality duplicate"
            );
            return Ok(None);
        }
    }

    let track = Track {
        id: Uuid::new_v4(),
        audio_hash,
        album_id: Some(album_id),
        file_path: scanned.file_path,
        title: scanned.track_metadata.title,
        sort_title,
        artist_id,
        disc: scanned.track_metadata.disc,
        track_no: scanned.track_metadata.track_no,
        date: scanned.track_metadata.date,
        composer: scanned.track_metadata.composer,
        comment: scanned.track_metadata.comment,
        original_title: scanned.track_metadata.original_title,
        original_artist: scanned.track_metadata.original_artist,
        original_album: scanned.track_metadata.original_album,
        format: scanned.audio.format,
        codec: scanned.audio.codec,
        duration_ms: scanned.audio.duration_ms,
        bitrate: scanned.audio.bitrate,
        sample_rate: scanned.audio.sample_rate,
        channels: None,
        file_size_bytes: scanned.audio.file_size_bytes,
        file_modified_at: None,
        replaygain_track_gain: scanned.track_metadata.replaygain_track_gain,
        replaygain_track_peak: scanned.track_metadata.replaygain_track_peak,
        metadata_modified_at: None,
        deleted_at: None,
        locked_at: None,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };
    TrackRepository::create(&mut *tx, &track).await.map(Some)
}
