use crate::db::entities::Track;
use crate::scanner::audio_hash::compute_audio_hash;
use crate::scanner::scan_album::ScannedAlbumMetadata;
use crate::scanner::scan_artist::ScannedArtistMetadata;
use crate::scanner::scan_track::ScannedTrackMetadata;

use super::error::ScannerError;
use super::error::ScannerResult;
use chrono::Utc;
use ffmpeg::format::context::Input;
use ffmpeg::media::Type as MediaType;
use ffmpeg_next as ffmpeg;
use uuid::Uuid;

struct ScannedFileAudio {
    audio_hash: [u8; 32],
    codec_id: ffmpeg::codec::Id,
    duration_ms: i32,
    bitrate: Option<i32>,
    sample_rate: Option<i64>,
    file_size_bytes: Option<i64>,
}

pub struct ScannedFile {
    file_path: String,
    audio: ScannedFileAudio,
    pub track_metadata: ScannedTrackMetadata,
    pub album_metadata: ScannedAlbumMetadata,
    pub artist_metadata: ScannedArtistMetadata,
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
            duration_ms: scanned.audio.duration_ms,
            bitrate: scanned.audio.bitrate,
            sample_rate: scanned.audio.sample_rate,
            channels: None,
            file_size_bytes: scanned.audio.file_size_bytes,
            file_modified_at: None,
            replaygain_track_gain: scanned.track_metadata.replaygain_track_gain,
            replaygain_track_peak: scanned.track_metadata.replaygain_track_peak,
            metadata_modified_at: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}

impl ScannedFile {
    pub fn scan(path: &std::path::Path) -> ScannerResult<Self> {
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

        let metadata = ScannedTrackMetadata::from_context(path, &ictx)?;
        let album_metadata = ScannedAlbumMetadata::from_context(&ictx, folder_name)?;
        let artist_metadata = ScannedArtistMetadata::from_track_context(&ictx);
        let audio = ScannedFileAudio::from_context(path, ictx)?;

        Ok(Self {
            file_path: path.to_string_lossy().into_owned(),
            track_metadata: metadata,
            audio,
            album_metadata,
            artist_metadata,
        })
    }
}

impl ScannedFileAudio {
    pub fn from_context(path: &std::path::Path, ictx: Input) -> ScannerResult<Self> {
        let file_size_bytes = std::fs::metadata(path).map(|m| m.len() as i64).ok();

        let audio_stream = ictx
            .streams()
            .best(MediaType::Audio)
            .ok_or(ScannerError::FailedToDetectFormat)?;

        let audio_stream_index = audio_stream.index();

        let codec_ctx =
            ffmpeg::codec::context::Context::from_parameters(audio_stream.parameters())?;
        let codec_id = codec_ctx.id();

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

        let audio_hash = compute_audio_hash(ictx, audio_stream_index)?;

        Ok(Self {
            codec_id,
            audio_hash,
            file_size_bytes,
            sample_rate,
            duration_ms,
            bitrate,
        })
    }
}
