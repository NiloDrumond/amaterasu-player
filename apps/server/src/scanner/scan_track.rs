use chrono::NaiveDate;
use ffmpeg_next::format::context::Input;

use crate::scanner::error::{ScannerError, ScannerResult};

pub fn parse_numeric_prefix(stem: &str) -> Option<(i32, String)> {
    let digit_end = stem.find(|c: char| !c.is_ascii_digit())?;
    if digit_end == 0 {
        return None;
    }
    let (prefix, sep_and_rest) = stem.split_at(digit_end);
    let n = prefix.parse::<i32>().ok()?;

    let rest = if let Some(after) = sep_and_rest.strip_prefix('_') {
        after.to_string()
    } else if let Some(after) = sep_and_rest.strip_prefix('.') {
        let trimmed = after.trim_start_matches(|c: char| c == ' ' || c == '\t');
        if trimmed.len() == after.len() {
            return None;
        }
        trimmed.to_string()
    } else {
        return None;
    };

    if rest.is_empty() {
        return None;
    }
    Some((n, rest))
}

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

        let get_string = |key: &str| -> Option<String> {
            metadata
                .get(key)
                .map(|v| String::from_utf8_lossy(v.as_bytes()).into_owned())
        };

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

    pub fn apply_filename_track_number(&mut self, file_stem: &str) {
        if self.track_no.is_some() {
            return;
        }
        let Some((n, rest)) = parse_numeric_prefix(file_stem) else {
            return;
        };
        self.track_no = Some(n);
        if self.title == file_stem {
            self.title = rest;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_numeric_prefix_underscore() {
        assert_eq!(parse_numeric_prefix("1_Hello"), Some((1, "Hello".into())));
        assert_eq!(parse_numeric_prefix("01_Hello"), Some((1, "Hello".into())));
        assert_eq!(
            parse_numeric_prefix("12_Foo_Bar"),
            Some((12, "Foo_Bar".into()))
        );
    }

    #[test]
    fn parse_numeric_prefix_dot_space() {
        assert_eq!(parse_numeric_prefix("1. Hello"), Some((1, "Hello".into())));
        assert_eq!(parse_numeric_prefix("01. Hello"), Some((1, "Hello".into())));
        assert_eq!(
            parse_numeric_prefix("12. Foo Bar"),
            Some((12, "Foo Bar".into()))
        );
    }

    #[test]
    fn parse_numeric_prefix_rejects_non_matches() {
        assert_eq!(parse_numeric_prefix("Hello"), None);
        assert_eq!(parse_numeric_prefix("1-Hello"), None);
        assert_eq!(parse_numeric_prefix("_Hello"), None);
        assert_eq!(parse_numeric_prefix("1_"), None);
        assert_eq!(parse_numeric_prefix("1."), None);
        assert_eq!(parse_numeric_prefix("1.Hello"), None);
        assert_eq!(parse_numeric_prefix(". Hello"), None);
        assert_eq!(parse_numeric_prefix("1. "), None);
    }

    fn meta(title: &str, track_no: Option<i32>) -> ScannedTrackMetadata {
        ScannedTrackMetadata {
            title: title.to_string(),
            disc: None,
            track_no,
            date: None,
            composer: None,
            comment: None,
            original_title: None,
            original_artist: None,
            original_album: None,
            sort_title: None,
            replaygain_track_gain: None,
            replaygain_track_peak: None,
        }
    }

    #[test]
    fn apply_noop_when_track_no_present() {
        let mut m = meta("1_Hello", Some(5));
        m.apply_filename_track_number("1_Hello");
        assert_eq!(m.track_no, Some(5));
        assert_eq!(m.title, "1_Hello");
    }

    #[test]
    fn apply_strips_prefix_when_title_equals_stem() {
        let mut m = meta("1_Hello", None);
        m.apply_filename_track_number("1_Hello");
        assert_eq!(m.track_no, Some(1));
        assert_eq!(m.title, "Hello");
    }

    #[test]
    fn apply_keeps_tagged_title_but_sets_track_no() {
        let mut m = meta("Real Title", None);
        m.apply_filename_track_number("1_Hello");
        assert_eq!(m.track_no, Some(1));
        assert_eq!(m.title, "Real Title");
    }

    #[test]
    fn apply_noop_when_stem_doesnt_match() {
        let mut m = meta("Hello", None);
        m.apply_filename_track_number("Hello");
        assert_eq!(m.track_no, None);
        assert_eq!(m.title, "Hello");
    }
}
