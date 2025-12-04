use std::collections::HashMap;

use crate::flog;
use crate::scanner::audio_hash::compute_audio_hash;

use super::error::ScannerError;
use super::error::ScannerResult;
use chrono::NaiveDate;
use symphonia::core::codecs::CodecType;
use symphonia::core::codecs::CODEC_TYPE_NULL;
use symphonia::core::meta::StandardTagKey;
use symphonia::core::meta::Value;
use symphonia::core::probe::ProbeResult;
use symphonia::core::{
    formats::FormatOptions, io::MediaSourceStream, meta::MetadataOptions, probe::Hint,
};

pub struct ScannedFile {
    audio_hash: [u8; 32],
    title: String,
    artist: Option<String>,
    album: Option<String>,
    album_artist: Option<String>,
    disc: Option<i32>,
    track_no: Option<i32>,
    date: Option<NaiveDate>,
    composer: Option<String>,
    comment: Option<String>,
    codec: i32,
    duration_ms: i32,
    bitrate: Option<i32>,
    sample_rate: Option<i32>,
    channels: Option<i32>,
    file_size_bytes: Option<i64>,
    replaygain_track_gain: Option<f32>,
    replaygain_album_gain: Option<f32>,
    raw_metadata: HashMap<StandardTagKey, Value>,
}

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
    raw_metadata: HashMap<StandardTagKey, Value>,
}

struct ScannedFileAudio {
    audio_hash: [u8; 32],
    codec: CodecType,
    duration_ms: i32,
    bitrate: Option<i32>,
    sample_rate: Option<i64>,
    file_size_bytes: Option<i64>,
}

impl ScannedFileMetadata {
    fn from_probed(path: &std::path::Path, probed: ProbeResult) -> ScannerResult<Self> {
        let file_name = path
            .file_stem()
            .and_then(|s| s.to_str())
            .ok_or(ScannerError::InvalidFileName(
                path.to_str().map(|s| s.to_string()),
            ))?
            .to_string();

        let mut metadata = probed.metadata;
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
            raw_metadata: tags.iter().map(|(k, v)| (*k, (*v).clone())).collect(),
        })
    }
}

impl ScannedFileAudio {
    pub fn from_probed(path: &std::path::Path, probed: ProbeResult) -> ScannerResult<Self> {
        // Get file size from filesystem metadata
        let file_size_bytes = std::fs::metadata(path).map(|m| m.len() as i64).ok();

        let format = probed.format;
        let track = format
            .tracks()
            .iter()
            .find(|t| t.codec_params.codec != CODEC_TYPE_NULL)
            .ok_or(ScannerError::FailedToDetectFormat)?;

        let codec_params = track.codec_params;
        let sample_rate = codec_params.sample_rate.map(|r| r as i64);

        let duration_ms = match (codec_params.time_base, codec_params.n_frames) {
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

        Ok(Self {
            codec: codec_params.codec,
            audio_hash: compute_audio_hash(probed)?,
            file_size_bytes,
            sample_rate,
            duration_ms,
            bitrate
        })
    }
}

pub fn scan_file(path: &std::path::Path) -> ScannerResult<()> {
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
    let format = probed.format;
    let mut metadata = probed.metadata;
    let metadata = metadata.get();
    let Some(mut metadata) = metadata else {
        return Err(ScannerError::FailedToExtractMetadata);
    };
    let Some(metadata) = metadata.skip_to_latest() else {
        return Err(ScannerError::FailedToExtractMetadata);
    };

    let track = format
        .tracks()
        .iter()
        .find(|t| t.codec_params.codec != CODEC_TYPE_NULL)
        .ok_or(ScannerError::FailedToDetectFormat)?;

    let codec = track.codec_params.codec;
    flog!("{:?}", metadata.tags());

    Ok(())
}
