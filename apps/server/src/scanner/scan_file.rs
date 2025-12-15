use crate::db::entities::Track;
use crate::scanner::audio_hash::compute_audio_hash;

use super::error::ScannerError;
use super::error::ScannerResult;
use chrono::NaiveDate;
use chrono::Utc;
use ffmpeg_next as ffmpeg;
use ffmpeg::format::context::Input;
use ffmpeg::media::Type as MediaType;
use uuid::Uuid;

struct ScannedFileMetadata {
    title: String,
    artist: Option<String>,
    disc: Option<i32>,
    track_no: Option<i32>,
    date: Option<NaiveDate>,
    composer: Option<String>,
    comment: Option<String>,
    sort_title: Option<String>,
    sort_artist: Option<String>,
    replaygain_track_gain: Option<f32>,
    replaygain_track_peak: Option<f32>,
}

pub struct ScannedAlbumMetadata {
    pub name: Option<String>,
    pub artist: Option<String>,
    pub sort_name: Option<String>,
    pub sort_artist: Option<String>,
    pub replaygain_album_gain: Option<f32>,
    pub replaygain_album_peak: Option<f32>,
}

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
    metadata: ScannedFileMetadata,
    audio: ScannedFileAudio,
    pub album_metadata: ScannedAlbumMetadata,
}

impl From<ScannedFile> for Track {
    fn from(scanned: ScannedFile) -> Self {
        Track {
            id: Uuid::new_v4(),
            audio_hash: scanned.audio.audio_hash.to_vec(),
            album_id: None,
            file_path: scanned.file_path,
            title: scanned.metadata.title,
            sort_title: scanned.metadata.sort_title,
            artist: scanned.metadata.artist,
            sort_artist: scanned.metadata.sort_artist,
            disc: scanned.metadata.disc,
            track_no: scanned.metadata.track_no,
            date: scanned.metadata.date,
            composer: scanned.metadata.composer,
            comment: scanned.metadata.comment,
            duration_ms: scanned.audio.duration_ms,
            bitrate: scanned.audio.bitrate,
            sample_rate: scanned.audio.sample_rate,
            channels: None,
            file_size_bytes: scanned.audio.file_size_bytes,
            file_modified_at: None,
            replaygain_track_gain: scanned.metadata.replaygain_track_gain,
            replaygain_track_peak: scanned.metadata.replaygain_track_peak,
            metadata_modified_at: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}

impl ScannedFile {
    pub fn scan(path: &std::path::Path) -> ScannerResult<Self> {
        ffmpeg::init()?;

        let path_str = path
            .to_str()
            .ok_or(ScannerError::InvalidFileName(path.to_str().map(|s| s.to_string())))?;
        let ictx = ffmpeg::format::input(&path_str)?;

        let metadata = ScannedFileMetadata::from_context(path, &ictx)?;
        let album_metadata = ScannedAlbumMetadata::from_context(&ictx);
        let audio = ScannedFileAudio::from_context(path, ictx)?;

        Ok(Self {
            file_path: path.to_string_lossy().into_owned(),
            metadata,
            audio,
            album_metadata,
        })
    }
}

impl ScannedFileMetadata {
    fn from_context(path: &std::path::Path, ictx: &Input) -> ScannerResult<Self> {
        let file_name = path
            .file_stem()
            .and_then(|s| s.to_str())
            .ok_or(ScannerError::InvalidFileName(
                path.to_str().map(|s| s.to_string()),
            ))?
            .to_string();

        let metadata = ictx.metadata();

        let get_string = |key: &str| -> Option<String> {
            metadata.get(key).map(|v| v.to_string())
        };

        let get_int = |key: &str| -> Option<i32> {
            metadata.get(key).and_then(|v| v.parse::<i32>().ok())
        };

        // Helper to parse ReplayGain gain values (e.g., "-6.50 dB")
        let parse_gain = |key: &str| -> Option<f32> {
            metadata.get(key).and_then(|v| {
                v.trim_end_matches(" dB")
                    .trim_end_matches("dB")
                    .trim()
                    .parse::<f32>()
                    .ok()
            })
        };

        // Helper to parse ReplayGain peak values (e.g., "0.988831")
        let parse_peak = |key: &str| -> Option<f32> {
            metadata.get(key).and_then(|v| v.parse::<f32>().ok())
        };

        Ok(Self {
            title: get_string("title").unwrap_or(file_name),
            artist: get_string("artist"),
            disc: get_int("disc").or_else(|| get_int("discnumber")),
            track_no: get_int("track").or_else(|| get_int("tracknumber")),
            date: get_string("date").and_then(|s| {
                // Try parsing as full date first, then year-only
                s.parse::<NaiveDate>()
                    .ok()
                    .or_else(|| s.parse::<i32>().ok().and_then(|y| NaiveDate::from_ymd_opt(y, 1, 1)))
            }),
            composer: get_string("composer"),
            comment: get_string("comment"),
            sort_title: get_string("titlesort").or_else(|| get_string("sort_title")),
            sort_artist: get_string("artistsort").or_else(|| get_string("sort_artist")),
            replaygain_track_gain: parse_gain("replaygain_track_gain"),
            replaygain_track_peak: parse_peak("replaygain_track_peak"),
        })
    }
}

impl ScannedAlbumMetadata {
    fn from_context(ictx: &Input) -> Self {
        let metadata = ictx.metadata();

        let get_string = |key: &str| -> Option<String> {
            metadata.get(key).map(|v| v.to_string())
        };

        let parse_gain = |key: &str| -> Option<f32> {
            metadata.get(key).and_then(|v| {
                v.trim_end_matches(" dB")
                    .trim_end_matches("dB")
                    .trim()
                    .parse::<f32>()
                    .ok()
            })
        };

        let parse_peak = |key: &str| -> Option<f32> {
            metadata.get(key).and_then(|v| v.parse::<f32>().ok())
        };

        Self {
            name: get_string("album"),
            artist: get_string("album_artist").or_else(|| get_string("albumartist")),
            sort_name: get_string("albumsort").or_else(|| get_string("sort_album")),
            sort_artist: get_string("albumartistsort").or_else(|| get_string("sort_album_artist")),
            replaygain_album_gain: parse_gain("replaygain_album_gain"),
            replaygain_album_peak: parse_peak("replaygain_album_peak"),
        }
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
                let duration_sec =
                    stream_duration as f64 * (time_base.numerator() as f64 / time_base.denominator() as f64);
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
