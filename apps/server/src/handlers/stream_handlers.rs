use axum::{
    body::Body,
    extract::{Path, State},
    http::{header, HeaderMap, HeaderValue, StatusCode},
    response::{IntoResponse, Response},
};
use chrono::{DateTime, Utc};
use tokio::io::{AsyncReadExt, AsyncSeekExt, SeekFrom};
use tokio_util::io::ReaderStream;
use uuid::Uuid;

use crate::{
    auth::middleware::AuthUser,
    db::entities::Track,
    error::{AppError, AppResult},
    repositories::TrackRepository,
    state::AppState,
    streaming::range_parser::{parse_range, RangeError},
};

const HTTP_DATE_FMT: &str = "%a, %d %b %Y %H:%M:%S GMT";

pub async fn stream_track(
    State(state): State<AppState>,
    _auth: AuthUser,
    Path(id): Path<Uuid>,
    headers: HeaderMap,
) -> AppResult<Response> {
    let track = TrackRepository::find_by_id(&state.db, id)
        .await?
        .ok_or(AppError::NotFound)?;

    let mut file = tokio::fs::File::open(&track.file_path)
        .await
        .map_err(|_| AppError::NotFound)?;

    let total = file
        .metadata()
        .await
        .map_err(|e| AppError::Internal(anyhow::anyhow!(e)))?
        .len();

    let etag = etag_for(&track, total);
    let last_modified = track.file_modified_at.map(format_http_date);

    if is_not_modified(&headers, &etag, track.file_modified_at) {
        return Ok(build_not_modified(&etag, last_modified.as_deref()));
    }

    let mime = mime_for(&track.file_path);
    let common = CommonHeaders {
        mime,
        etag: &etag,
        last_modified: last_modified.as_deref(),
    };

    if let Some(range_header) = headers.get(header::RANGE).and_then(|h| h.to_str().ok()) {
        match parse_range(range_header, total) {
            Ok((start, end, total)) => {
                file.seek(SeekFrom::Start(start))
                    .await
                    .map_err(|e| AppError::Internal(anyhow::anyhow!(e)))?;
                let len = end - start + 1;
                let stream = ReaderStream::new(file.take(len));
                let body = Body::from_stream(stream);
                return Ok(build_partial(body, start, end, total, len, common));
            }
            Err(RangeError::NotSatisfiable) => {
                return Ok(build_range_not_satisfiable(total));
            }
            Err(RangeError::Malformed) => return Err(AppError::InvalidRangeHeader),
        }
    }

    let stream = ReaderStream::new(file);
    let body = Body::from_stream(stream);
    Ok(build_full(body, total, common))
}

struct CommonHeaders<'a> {
    mime: &'static str,
    etag: &'a str,
    last_modified: Option<&'a str>,
}

fn build_full(body: Body, total: u64, common: CommonHeaders<'_>) -> Response {
    let mut response = (StatusCode::OK, body).into_response();
    let h = response.headers_mut();
    insert_common_headers(h, &common);
    h.insert(header::CONTENT_LENGTH, header_val(total.to_string()));
    response
}

fn build_partial(
    body: Body,
    start: u64,
    end: u64,
    total: u64,
    len: u64,
    common: CommonHeaders<'_>,
) -> Response {
    let mut response = (StatusCode::PARTIAL_CONTENT, body).into_response();
    let h = response.headers_mut();
    insert_common_headers(h, &common);
    h.insert(header::CONTENT_LENGTH, header_val(len.to_string()));
    h.insert(
        header::CONTENT_RANGE,
        header_val(format!("bytes {start}-{end}/{total}")),
    );
    response
}

fn build_range_not_satisfiable(total: u64) -> Response {
    let mut response = AppError::RangeNotSatisfiable.into_response();
    response.headers_mut().insert(
        header::CONTENT_RANGE,
        header_val(format!("bytes */{total}")),
    );
    response
}

fn build_not_modified(etag: &str, last_modified: Option<&str>) -> Response {
    let mut response = StatusCode::NOT_MODIFIED.into_response();
    let h = response.headers_mut();
    h.insert(header::ETAG, header_val(etag.to_string()));
    if let Some(lm) = last_modified {
        h.insert(header::LAST_MODIFIED, header_val(lm.to_string()));
    }
    h.insert(
        header::CACHE_CONTROL,
        HeaderValue::from_static("private, no-cache"),
    );
    response
}

fn insert_common_headers(h: &mut axum::http::HeaderMap, c: &CommonHeaders<'_>) {
    h.insert(header::CONTENT_TYPE, HeaderValue::from_static(c.mime));
    h.insert(header::ACCEPT_RANGES, HeaderValue::from_static("bytes"));
    h.insert(
        header::CACHE_CONTROL,
        HeaderValue::from_static("private, no-cache"),
    );
    h.insert(header::ETAG, header_val(c.etag.to_string()));
    if let Some(lm) = c.last_modified {
        h.insert(header::LAST_MODIFIED, header_val(lm.to_string()));
    }
}

fn header_val(s: String) -> HeaderValue {
    HeaderValue::from_str(&s).unwrap_or(HeaderValue::from_static(""))
}

fn mime_for(path: &str) -> &'static str {
    let ext = path
        .rsplit_once('.')
        .map(|(_, e)| e.to_ascii_lowercase())
        .unwrap_or_default();
    match ext.as_str() {
        "mp3" => "audio/mpeg",
        "flac" => "audio/flac",
        "ogg" | "oga" => "audio/ogg",
        "opus" => "audio/ogg; codecs=opus",
        "m4a" | "mp4" => "audio/mp4",
        "wav" => "audio/wav",
        "aac" => "audio/aac",
        _ => "application/octet-stream",
    }
}

fn etag_for(track: &Track, total: u64) -> String {
    if track.audio_hash.len() >= 8 {
        let hex: String = track
            .audio_hash
            .iter()
            .take(8)
            .map(|b| format!("{:02x}", b))
            .collect();
        format!("W/\"{hex}\"")
    } else {
        let mtime = track
            .file_modified_at
            .map(|t| t.timestamp())
            .unwrap_or_default();
        format!("W/\"{mtime}-{total}\"")
    }
}

fn format_http_date(dt: DateTime<Utc>) -> String {
    dt.format(HTTP_DATE_FMT).to_string()
}

fn is_not_modified(
    headers: &HeaderMap,
    etag: &str,
    file_modified_at: Option<DateTime<Utc>>,
) -> bool {
    if let Some(inm) = headers
        .get(header::IF_NONE_MATCH)
        .and_then(|v| v.to_str().ok())
    {
        if inm.split(',').any(|t| t.trim() == etag) {
            return true;
        }
    }

    if let (Some(ims), Some(modified)) = (
        headers
            .get(header::IF_MODIFIED_SINCE)
            .and_then(|v| v.to_str().ok()),
        file_modified_at,
    ) {
        if let Ok(ims_dt) = DateTime::parse_from_rfc2822(ims) {
            let ims_utc = ims_dt.with_timezone(&Utc);
            // Compare at second precision since HTTP dates don't carry sub-second.
            if modified.timestamp() <= ims_utc.timestamp() {
                return true;
            }
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::HeaderValue;
    use chrono::TimeZone;

    fn test_track(audio_hash: Vec<u8>, file_modified_at: Option<DateTime<Utc>>) -> Track {
        let now = Utc::now();
        Track {
            id: Uuid::nil(),
            audio_hash,
            album_id: None,
            file_path: String::new(),
            title: String::new(),
            sort_title: String::new(),
            artist_id: None,
            disc: None,
            track_no: None,
            date: None,
            composer: None,
            comment: None,
            original_title: None,
            original_artist: None,
            original_album: None,
            format: String::new(),
            codec: String::new(),
            duration_ms: 0,
            bitrate: None,
            sample_rate: None,
            channels: None,
            file_size_bytes: None,
            file_modified_at,
            replaygain_track_gain: None,
            replaygain_track_peak: None,
            metadata_modified_at: None,
            deleted_at: None,
            locked_at: None,
            created_at: now,
            updated_at: now,
        }
    }

    #[test]
    fn mime_for_known_extensions() {
        assert_eq!(mime_for("/music/song.mp3"), "audio/mpeg");
        assert_eq!(mime_for("track.FLAC"), "audio/flac");
        assert_eq!(mime_for("x.ogg"), "audio/ogg");
        assert_eq!(mime_for("x.opus"), "audio/ogg; codecs=opus");
        assert_eq!(mime_for("x.m4a"), "audio/mp4");
        assert_eq!(mime_for("x.wav"), "audio/wav");
    }

    #[test]
    fn mime_for_unknown_or_missing_extension() {
        assert_eq!(mime_for("noext"), "application/octet-stream");
        assert_eq!(mime_for(""), "application/octet-stream");
    }

    #[test]
    fn etag_for_uses_audio_hash_prefix_when_long_enough() {
        let hash: Vec<u8> = (0u8..16).collect();
        let t = test_track(hash, None);
        assert_eq!(etag_for(&t, 999), "W/\"0001020304050607\"");
    }

    #[test]
    fn etag_for_falls_back_to_mtime_when_hash_short() {
        let mtime = Utc.with_ymd_and_hms(2024, 6, 15, 10, 30, 0).unwrap();
        let t = test_track(vec![1, 2, 3], Some(mtime));
        assert_eq!(
            etag_for(&t, 4096),
            format!("W/\"{}-4096\"", mtime.timestamp())
        );
    }

    #[test]
    fn is_not_modified_if_none_match_exact() {
        let mut h = HeaderMap::new();
        h.insert(
            header::IF_NONE_MATCH,
            HeaderValue::from_static("W/\"abc123\""),
        );
        assert!(is_not_modified(&h, "W/\"abc123\"", None));
    }

    #[test]
    fn is_not_modified_if_none_match_in_list() {
        let mut h = HeaderMap::new();
        h.insert(
            header::IF_NONE_MATCH,
            HeaderValue::from_str(r#"W/"other", W/"target""#).unwrap(),
        );
        assert!(is_not_modified(&h, "W/\"target\"", None));
    }

    #[test]
    fn is_not_modified_false_when_etag_differs() {
        let mut h = HeaderMap::new();
        h.insert(header::IF_NONE_MATCH, HeaderValue::from_static("W/\"a\""));
        assert!(!is_not_modified(&h, "W/\"b\"", None));
    }

    #[test]
    fn is_not_modified_if_modified_since_when_not_newer() {
        let modified = Utc.with_ymd_and_hms(2020, 1, 1, 12, 0, 0).unwrap();
        let ims = format_http_date(Utc.with_ymd_and_hms(2020, 1, 1, 13, 0, 0).unwrap());
        let mut h = HeaderMap::new();
        h.insert(
            header::IF_MODIFIED_SINCE,
            HeaderValue::from_str(&ims).unwrap(),
        );
        assert!(is_not_modified(&h, "W/\"x\"", Some(modified)));
    }

    #[test]
    fn is_modified_when_newer_than_if_modified_since() {
        let modified = Utc.with_ymd_and_hms(2020, 1, 2, 12, 0, 0).unwrap();
        let ims = format_http_date(Utc.with_ymd_and_hms(2020, 1, 1, 0, 0, 0).unwrap());
        let mut h = HeaderMap::new();
        h.insert(
            header::IF_MODIFIED_SINCE,
            HeaderValue::from_str(&ims).unwrap(),
        );
        assert!(!is_not_modified(&h, "W/\"x\"", Some(modified)));
    }

    #[test]
    fn format_http_date_roundtrips_for_if_modified_since() {
        let dt = Utc.with_ymd_and_hms(2019, 11, 29, 0, 0, 0).unwrap();
        let s = format_http_date(dt);
        let parsed = DateTime::parse_from_rfc2822(&s)
            .unwrap()
            .with_timezone(&Utc);
        assert_eq!(parsed.timestamp(), dt.timestamp());
    }
}
