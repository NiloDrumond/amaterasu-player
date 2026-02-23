use chrono::NaiveDate;
use ffmpeg_next::format::context::Input;

use crate::scanner::error::{ScannerError, ScannerResult};

pub struct ScannedTrackMetadata {
    pub title: String,
    pub disc: Option<i32>,
    pub track_no: Option<i32>,
    pub date: Option<NaiveDate>,
    pub composer: Option<String>,
    pub comment: Option<String>,
    pub original_title: Option<String>,
    pub original_artist: Option<String>,
    pub original_album: Option<String>,
    pub sort_title: Option<String>,
    pub replaygain_track_gain: Option<f32>,
    pub replaygain_track_peak: Option<f32>,
}

impl ScannedTrackMetadata {
    pub fn from_context(path: &std::path::Path, ictx: &Input) -> ScannerResult<Self> {
        let file_name = path
            .file_stem()
            .and_then(|s| s.to_str())
            .ok_or(ScannerError::InvalidFileName(
                path.to_str().map(|s| s.to_string()),
            ))?
            .to_string();

        let metadata = ictx.metadata();

        let get_string = |key: &str| -> Option<String> { metadata.get(key).map(|v| v.to_string()) };

        let get_int =
            |key: &str| -> Option<i32> { metadata.get(key).and_then(|v| v.parse::<i32>().ok()) };

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
        let parse_peak =
            |key: &str| -> Option<f32> { metadata.get(key).and_then(|v| v.parse::<f32>().ok()) };

        Ok(Self {
            title: get_string("title").unwrap_or(file_name),
            disc: get_int("disc").or_else(|| get_int("discnumber")),
            track_no: get_int("track").or_else(|| get_int("tracknumber")),
            date: get_string("date").and_then(|s| {
                // Try parsing as full date first, then year-only
                s.parse::<NaiveDate>().ok().or_else(|| {
                    s.parse::<i32>()
                        .ok()
                        .and_then(|y| NaiveDate::from_ymd_opt(y, 1, 1))
                })
            }),
            composer: get_string("composer"),
            comment: get_string("comment"),
            original_title: get_string("original_title")
                .or_else(|| get_string("ORIGINAL_TITLE"))
                .or_else(|| get_string("ORIGINALTITLE")),
            original_artist: get_string("original_artist")
                .or_else(|| get_string("ORIGINAL_ARTIST"))
                .or_else(|| get_string("ORIGINALARTIST")),
            original_album: get_string("original_album")
                .or_else(|| get_string("ORIGINAL_ALBUM"))
                .or_else(|| get_string("ORIGINALALBUM")),
            sort_title: get_string("titlesort").or_else(|| get_string("sort_title")),
            replaygain_track_gain: parse_gain("replaygain_track_gain"),
            replaygain_track_peak: parse_peak("replaygain_track_peak"),
        })
    }
}
