use crate::db::entities::Track;
use crate::scanner::audio_hash::scan_packets;
use crate::scanner::scan_album::ScannedAlbumMetadata;
use crate::scanner::scan_artist::ScannedArtistMetadata;
use crate::scanner::scan_cover::ScannedCover;
use crate::scanner::scan_track::ScannedTrackMetadata;

use super::error::ScannerError;
use super::error::ScannerResult;
use chrono::Utc;
use ffmpeg::format::context::Input;
use ffmpeg::media::Type as MediaType;
use ffmpeg_next as ffmpeg;
use uuid::Uuid;

pub(super) struct ScannedFileAudio {
    pub audio_hash: [u8; 32],
    pub format: String,
    pub codec: String,
    pub duration_ms: i32,
    pub bitrate: Option<i32>,
    pub sample_rate: Option<i64>,
    pub file_size_bytes: Option<i64>,
}

pub struct ScannedFile {
    pub(super) file_path: String,
    pub(super) audio: ScannedFileAudio,
    pub track_metadata: ScannedTrackMetadata,
    pub album_metadata: ScannedAlbumMetadata,
    pub artist_metadata: ScannedArtistMetadata,
    pub(super) cover: Option<ScannedCover>,
}

impl From<ScannedFile> for Track {
    fn from(scanned: ScannedFile) -> Self {
        Track {
            id: Uuid::new_v4(),
            audio_hash: scanned.audio.audio_hash.to_vec(),
            artist_id: None,
            album_id: None,
            file_path: scanned.file_path,
            title: scanned.track_metadata.title.to_string(),
            sort_title: scanned
                .track_metadata
                .sort_title
                .unwrap_or(scanned.track_metadata.title),
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
            approved: false,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}

impl ScannedFile {
    pub fn file_stem(&self) -> &str {
        std::path::Path::new(&self.file_path)
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("")
    }

    pub fn file_path(&self) -> &str {
        &self.file_path
    }

    pub fn scan(path: &std::path::Path, library_path: &str) -> ScannerResult<Self> {
        ffmpeg::init()?;

        let path_str = path.to_str().ok_or(ScannerError::InvalidFileName(
            path.to_str().map(|s| s.to_string()),
        ))?;
        let ictx = ffmpeg::format::input(&path_str)?;

        let folder_name = path
            .parent()
            .and_then(|p| p.file_name())
            .and_then(|n| n.to_str())
            .map(|s| s.to_string());

        let artist_folder_name = path
            .parent()
            .and_then(|p| p.parent())
            .filter(|gp| *gp != std::path::Path::new(library_path))
            .and_then(|gp| gp.file_name())
            .and_then(|n| n.to_str())
            .map(|s| s.to_string());

        let metadata = ScannedTrackMetadata::from_context(path, &ictx)?;
        let album_metadata =
            ScannedAlbumMetadata::from_context(&ictx, folder_name, artist_folder_name.clone())?;
        let artist_metadata = ScannedArtistMetadata::from_track_context(&ictx, artist_folder_name);
        let (audio, cover) = ScannedFileAudio::from_context(path, ictx)?;

        Ok(Self {
            file_path: path.to_string_lossy().into_owned(),
            track_metadata: metadata,
            audio,
            album_metadata,
            artist_metadata,
            cover,
        })
    }
}

impl ScannedFileAudio {
    pub fn from_context(
        path: &std::path::Path,
        ictx: Input,
    ) -> ScannerResult<(Self, Option<ScannedCover>)> {
        let format = path
            .extension()
            .and_then(|ext| ext.to_str())
            .map(|ext| ext.to_ascii_lowercase())
            .ok_or_else(|| ScannerError::InvalidFileName(path.to_str().map(|s| s.to_string())))?;

        let file_size_bytes = std::fs::metadata(path).map(|m| m.len() as i64).ok();

        let audio_stream = ictx
            .streams()
            .best(MediaType::Audio)
            .ok_or(ScannerError::FailedToDetectFormat)?;

        let audio_stream_index = audio_stream.index();
        let cover_stream_index = ScannedCover::stream_index(&ictx);

        let codec_ctx =
            ffmpeg::codec::context::Context::from_parameters(audio_stream.parameters())?;
        let codec = codec_ctx.id().name().to_string();

        let sample_rate = if let Ok(audio_decoder) = codec_ctx.decoder().audio() {
            Some(audio_decoder.rate() as i64)
        } else {
            None
        };

        let duration_ms = {
            let stream_duration = audio_stream.duration();
            let time_base = audio_stream.time_base();
            if stream_duration > 0 {
                // Convert stream duration to milliseconds
                let duration_sec = stream_duration as f64
                    * (time_base.numerator() as f64 / time_base.denominator() as f64);
                (duration_sec * 1000.0) as i32
            } else {
                let ctx_duration = ictx.duration();
                if ctx_duration > 0 {
                    (ctx_duration as f64 / 1000.0) as i32 // microseconds to milliseconds
                } else {
                    0
                }
            }
        };

        let bitrate = match (file_size_bytes, duration_ms) {
            (Some(size), dur) if dur > 0 => {
                let duration_sec = dur as f64 / 1000.0;
                Some(((size as f64 * 8.0) / duration_sec) as i32)
            }
            _ => None,
        };

        let scan = scan_packets(ictx, audio_stream_index, cover_stream_index)?;

        Ok((
            Self {
                format,
                codec,
                audio_hash: scan.audio_hash,
                file_size_bytes,
                sample_rate,
                duration_ms,
                bitrate,
            },
            scan.cover,
        ))
    }
}
