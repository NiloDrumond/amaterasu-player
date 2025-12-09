use std::collections::HashMap;

use crate::db::entities::Track;
use crate::scanner::audio_hash::compute_audio_hash;

use super::error::ScannerError;
use super::error::ScannerResult;
use chrono::NaiveDate;
use chrono::Utc;
use symphonia::core::codecs::CodecType;
use symphonia::core::codecs::CODEC_TYPE_NULL;
use symphonia::core::formats::FormatReader;
use symphonia::core::meta::StandardTagKey;
use symphonia::core::meta::Value;
use symphonia::core::probe::ProbedMetadata;
use symphonia::core::{
    formats::FormatOptions, io::MediaSourceStream, meta::MetadataOptions, probe::Hint,
};
use uuid::Uuid;

struct ScannedFileMetadata {
    title: String,
    artist: Option<String>,
    album: Option<String>,
    album_artist: Option<String>,
    disc: Option<i32>,
    track_no: Option<i32>,
    date: Option<NaiveDate>,
    composer: Option<String>,
    comment: Option<String>,
    sort_title: Option<String>,
    sort_artist: Option<String>,
}

struct ScannedFileAudio {
    audio_hash: [u8; 32],
    codec: CodecType,
    duration_ms: i32,
    bitrate: Option<i32>,
    sample_rate: Option<i64>,
    file_size_bytes: Option<i64>,
}

pub struct ScannedFile {
    file_path: String,
    metadata: ScannedFileMetadata,
    audio: ScannedFileAudio,
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
            replaygain_track_gain: None,
            replaygain_album_gain: None,
            metadata_modified_at: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}

impl ScannedFile {
    pub fn scan(path: &std::path::Path) -> ScannerResult<Self> {
        let probe = symphonia::default::get_probe();
        let src = std::fs::File::open(path)?;
        let mss = MediaSourceStream::new(Box::new(src), Default::default());

        let mut hint = Hint::new();
        if let Some(extension) = path.extension() {
            if let Some(extension) = extension.to_str() {
                hint.with_extension(extension);
            }
        }
        let meta_opts: MetadataOptions = Default::default();
        let fmt_opts: FormatOptions = Default::default();

        let probed = probe.format(&hint, mss, &fmt_opts, &meta_opts)?;
        let metadata = ScannedFileMetadata::from_metadata(path, probed.metadata)?;
        let audio = ScannedFileAudio::from_format(path, probed.format)?;

        Ok(Self {
            file_path: path.to_string_lossy().into_owned(),
            metadata,
            audio,
        })
    }
}

impl ScannedFileMetadata {
    fn from_metadata(path: &std::path::Path, mut metadata: ProbedMetadata) -> ScannerResult<Self> {
        let file_name = path
            .file_stem()
            .and_then(|s| s.to_str())
            .ok_or(ScannerError::InvalidFileName(
                path.to_str().map(|s| s.to_string()),
            ))?
            .to_string();

        let metadata = metadata.get();
        let Some(mut metadata) = metadata else {
            return Err(ScannerError::FailedToExtractMetadata);
        };
        let Some(metadata) = metadata.skip_to_latest() else {
            return Err(ScannerError::FailedToExtractMetadata);
        };

        let tags = metadata.tags();
        let tags: HashMap<StandardTagKey, &Value> = tags
            .iter()
            .filter_map(|tag| tag.std_key.map(|key| (key, &tag.value)))
            .collect();

        Ok(Self {
            title: tags
                .get(&StandardTagKey::TrackTitle)
                .map(|v| v.to_string())
                .unwrap_or(file_name),
            artist: tags.get(&StandardTagKey::Artist).and_then(|v| match v {
                Value::String(s) => Some(s.clone()),
                _ => None,
            }),
            album: tags.get(&StandardTagKey::Album).and_then(|v| match v {
                Value::String(s) => Some(s.clone()),
                _ => None,
            }),
            album_artist: tags
                .get(&StandardTagKey::AlbumArtist)
                .and_then(|v| match v {
                    Value::String(s) => Some(s.clone()),
                    _ => None,
                }),
            disc: tags.get(&StandardTagKey::DiscNumber).and_then(|v| match v {
                Value::SignedInt(i) => Some(*i as i32),
                Value::UnsignedInt(u) => Some(*u as i32),
                _ => None,
            }),
            track_no: tags
                .get(&StandardTagKey::TrackNumber)
                .and_then(|v| match v {
                    Value::SignedInt(i) => Some(*i as i32),
                    Value::UnsignedInt(u) => Some(*u as i32),
                    _ => None,
                }),
            date: tags.get(&StandardTagKey::Date).and_then(|v| match v {
                Value::String(s) => Some(s.clone().parse::<NaiveDate>().ok()?),
                _ => None,
            }),
            composer: tags.get(&StandardTagKey::Composer).and_then(|v| match v {
                Value::String(s) => Some(s.clone()),
                _ => None,
            }),
            comment: tags.get(&StandardTagKey::Comment).and_then(|v| match v {
                Value::String(s) => Some(s.clone()),
                _ => None,
            }),
            sort_title: tags
                .get(&StandardTagKey::SortTrackTitle)
                .and_then(|v| match v {
                    Value::String(s) => Some(s.clone()),
                    _ => None,
                }),
            sort_artist: tags.get(&StandardTagKey::SortArtist).and_then(|v| match v {
                Value::String(s) => Some(s.clone()),
                _ => None,
            }),
        })
    }
}

impl ScannedFileAudio {
    pub fn from_format(
        path: &std::path::Path,
        format: Box<dyn FormatReader>,
    ) -> ScannerResult<Self> {
        // Get file size from filesystem metadata
        let file_size_bytes = std::fs::metadata(path).map(|m| m.len() as i64).ok();

        let track = format
            .tracks()
            .iter()
            .find(|t| t.codec_params.codec != CODEC_TYPE_NULL)
            .ok_or(ScannerError::FailedToDetectFormat)?;

        let codec = track.codec_params.codec;
        let sample_rate = track.codec_params.sample_rate.map(|r| r as i64);

        let duration_ms = match (track.codec_params.time_base, track.codec_params.n_frames) {
            (Some(time_base), Some(n_frames)) => {
                let time = time_base.calc_time(n_frames);
                // time.seconds is u64, time.frac is f64 (fractional seconds)
                ((time.seconds as f64 + time.frac) * 1000.0) as i32
            }
            _ => 0, // or handle this case differently
        };

        let bitrate = match (file_size_bytes, duration_ms) {
            (Some(size), dur) if dur > 0 => {
                // bits per second: (bytes * 8) / seconds
                let duration_sec = dur as f64 / 1000.0;
                Some(((size as f64 * 8.0) / duration_sec) as i32)
            }
            _ => None,
        };

        let audio_hash = compute_audio_hash(format)?;

        Ok(Self {
            codec,
            audio_hash,
            file_size_bytes,
            sample_rate,
            duration_ms,
            bitrate,
        })
    }
}
