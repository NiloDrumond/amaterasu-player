//! Thin MusicBrainz HTTP client.
//!
//! Owns its own `reqwest::Client` and a global 1 req/sec rate limiter (MB's
//! anonymous limit). Every outbound request carries the configured
//! `User-Agent` -- MB rejects generic UAs with 403.

use std::num::NonZeroU32;
use std::sync::Arc;
use std::time::Duration;

use governor::clock::DefaultClock;
use governor::state::{InMemoryState, NotKeyed};
use governor::{Quota, RateLimiter};
use reqwest::StatusCode;
use serde::de::DeserializeOwned;

use super::error::{MbError, MbResult};
use super::models::{ArtistSearchResponse, RecordingSearchResponse, ReleaseGroupSearchResponse};

type DirectRateLimiter = RateLimiter<NotKeyed, InMemoryState, DefaultClock>;

#[derive(Clone)]
pub struct MusicBrainzClient {
    http: reqwest::Client,
    base_url: String,
    limiter: Arc<DirectRateLimiter>,
}

impl MusicBrainzClient {
    pub fn new(user_agent: &str, base_url: String) -> MbResult<Self> {
        let http = reqwest::Client::builder()
            .user_agent(user_agent)
            .timeout(Duration::from_secs(30))
            .build()
            .map_err(MbError::Http)?;
        // 1 request per second -- MB's anonymous quota.
        let quota = Quota::per_second(NonZeroU32::new(1).expect("1 is nonzero"));
        Ok(Self {
            http,
            base_url: ensure_trailing_slash(base_url),
            limiter: Arc::new(RateLimiter::direct(quota)),
        })
    }

    /// Search release-groups (album-level) by artist + title. Caller chooses
    /// `limit` (MB caps at 100). Returns candidates in MB's score-descending
    /// order.
    ///
    /// Uses `dismax=true` -- MB's "fuzzy bag-of-words across multiple fields"
    /// query parser. Much more forgiving than the default Lucene parser for
    /// titles with non-ASCII chars, partial matches, or translation
    /// differences (e.g. "Soundtrack" vs "オリジナル・サウンドトラック").
    pub async fn search_release_group(
        &self,
        artist: Option<&str>,
        title: &str,
        limit: usize,
    ) -> MbResult<ReleaseGroupSearchResponse> {
        let query = build_dismax_query(&[Some(title), artist]);
        let url = format!("{}release-group", self.base_url);
        self.get_json(
            &url,
            &[
                ("query", query.as_str()),
                ("dismax", "true"),
                ("fmt", "json"),
                ("limit", &limit.to_string()),
            ],
        )
        .await
    }

    pub async fn search_artist(&self, name: &str, limit: usize) -> MbResult<ArtistSearchResponse> {
        let query = build_dismax_query(&[Some(name)]);
        let url = format!("{}artist", self.base_url);
        self.get_json(
            &url,
            &[
                ("query", query.as_str()),
                ("dismax", "true"),
                ("fmt", "json"),
                ("limit", &limit.to_string()),
            ],
        )
        .await
    }

    pub async fn search_recording(
        &self,
        artist: Option<&str>,
        release: Option<&str>,
        title: &str,
        limit: usize,
    ) -> MbResult<RecordingSearchResponse> {
        let query = build_dismax_query(&[Some(title), artist, release]);
        let url = format!("{}recording", self.base_url);
        self.get_json(
            &url,
            &[
                ("query", query.as_str()),
                ("dismax", "true"),
                ("fmt", "json"),
                ("limit", &limit.to_string()),
            ],
        )
        .await
    }

    async fn get_json<T: DeserializeOwned>(
        &self,
        url: &str,
        query: &[(&str, &str)],
    ) -> MbResult<T> {
        // 1 attempt + up to 2 retries on transient errors. Capped at 3 total
        // -- MB rarely benefits from longer chains and we don't want to hold
        // a worker on a single job forever.
        let mut attempt = 0u32;
        loop {
            self.limiter.until_ready().await;
            let resp = self.http.get(url).query(query).send().await;
            match resp {
                Ok(r) => {
                    let status = r.status();
                    if status.is_success() {
                        return r.json().await.map_err(MbError::Http);
                    }
                    if (status == StatusCode::SERVICE_UNAVAILABLE
                        || status == StatusCode::TOO_MANY_REQUESTS
                        || status.is_server_error())
                        && attempt < 2
                    {
                        let backoff = Duration::from_millis(500 * (1 << attempt));
                        tracing::warn!(
                            "musicbrainz: {} on {} (attempt {}), backing off {:?}",
                            status,
                            url,
                            attempt + 1,
                            backoff
                        );
                        tokio::time::sleep(backoff).await;
                        attempt += 1;
                        continue;
                    }
                    return Err(MbError::Status(status));
                }
                Err(e) => {
                    if attempt < 2 && (e.is_timeout() || e.is_connect()) {
                        let backoff = Duration::from_millis(500 * (1 << attempt));
                        tracing::warn!(
                            "musicbrainz: transport error on {} (attempt {}): {} -- backing off {:?}",
                            url,
                            attempt + 1,
                            e,
                            backoff
                        );
                        tokio::time::sleep(backoff).await;
                        attempt += 1;
                        continue;
                    }
                    return Err(MbError::Http(e));
                }
            }
        }
    }
}

fn ensure_trailing_slash(mut url: String) -> String {
    if !url.ends_with('/') {
        url.push('/');
    }
    url
}

/// Concatenate non-empty terms into a single dismax query string. dismax mode
/// is just bag-of-words against all relevant fields, so we don't need Lucene
/// escapes -- just strip characters that would confuse the parser at the
/// top level (`+`, `-`, leading operators).
fn build_dismax_query(terms: &[Option<&str>]) -> String {
    terms
        .iter()
        .filter_map(|t| {
            t.and_then(|s| {
                let trimmed = s.trim();
                if trimmed.is_empty() {
                    None
                } else {
                    Some(trimmed)
                }
            })
        })
        .map(sanitize_dismax_term)
        .collect::<Vec<_>>()
        .join(" ")
}

/// dismax tolerates most special chars but a few are still operator-ish at
/// the start of a token (`+`, `-`, `!`). Replace them with spaces; the
/// resulting token boundaries fold into the bag-of-words.
fn sanitize_dismax_term(s: &str) -> String {
    s.chars()
        .map(|c| match c {
            '+' | '-' | '!' | '(' | ')' | '{' | '}' | '[' | ']' | '^' | '~' | '*' | '?' | ':'
            | '\\' | '/' | '"' => ' ',
            other => other,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dismax_concatenates_terms_with_spaces() {
        let q = build_dismax_query(&[Some("Path of the Goddess"), Some("Capcom Sound Team")]);
        assert_eq!(q, "Path of the Goddess Capcom Sound Team");
    }

    #[test]
    fn dismax_skips_none_and_empty() {
        let q = build_dismax_query(&[Some("Title"), None, Some("   ")]);
        assert_eq!(q, "Title");
    }

    #[test]
    fn dismax_strips_operator_chars() {
        // Full-width colon (U+FF1A) is preserved -- only ASCII operators
        // confuse the parser. Forward slash and quote get stripped.
        assert_eq!(
            sanitize_dismax_term(r#"祇：Path of the "Goddess"/sound"#),
            r#"祇：Path of the  Goddess  sound"#
        );
    }
}
