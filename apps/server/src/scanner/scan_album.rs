use chrono::NaiveDate;
use ffmpeg_next::format::context::Input;

use crate::scanner::{
    error::{ScannerError, ScannerResult},
    scan_artist::ScannedArtistMetadata,
};

pub struct ScannedAlbumMetadata {
    pub name: String,
    pub sort_name: Option<String>,
    pub date: Option<NaiveDate>,
    pub replaygain_album_gain: Option<f32>,
    pub replaygain_album_peak: Option<f32>,
    pub album_artist_metadata: ScannedArtistMetadata,
}

impl ScannedAlbumMetadata {
    pub fn from_context(
        ictx: &Input,
        folder_name: Option<String>,
        artist_folder_name: Option<String>,
    ) -> ScannerResult<Self> {
        let metadata = ictx.metadata();

        let get_string = |key: &str| -> Option<String> {
            metadata
                .get(key)
                .map(|v| String::from_utf8_lossy(v.as_bytes()).into_owned())
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

        let parse_peak =
            |key: &str| -> Option<f32> { metadata.get(key).and_then(|v| v.parse::<f32>().ok()) };

        let album_artist = ScannedArtistMetadata::from_album_context(ictx, artist_folder_name);

        Ok(Self {
            name: get_string("album")
                .unwrap_or(folder_name.ok_or(ScannerError::FailedToExtractMetadata)?),
            sort_name: get_string("albumsort").or_else(|| get_string("sort_album")),
            date: get_string("date").and_then(|s| {
                // Try parsing as full date first, then year-only
                s.parse::<NaiveDate>().ok().or_else(|| {
                    s.parse::<i32>()
                        .ok()
                        .and_then(|y| NaiveDate::from_ymd_opt(y, 1, 1))
                })
            }),
            replaygain_album_gain: parse_gain("replaygain_album_gain"),
            replaygain_album_peak: parse_peak("replaygain_album_peak"),
            album_artist_metadata: album_artist,
        })
    }
}
