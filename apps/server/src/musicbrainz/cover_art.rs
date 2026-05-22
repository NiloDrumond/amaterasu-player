//! Cover Art Archive client. Separate from MB client because CAA has its own
//! domain (and its own implicit rate budget that we respect by holding to 1
//! req/sec on the parallel limiter).

use std::num::NonZeroU32;
use std::sync::Arc;
use std::time::Duration;

use governor::clock::DefaultClock;
use governor::state::{InMemoryState, NotKeyed};
use governor::{Quota, RateLimiter};
use reqwest::StatusCode;

use super::error::{MbError, MbResult};

type DirectRateLimiter = RateLimiter<NotKeyed, InMemoryState, DefaultClock>;

#[derive(Clone)]
pub struct CoverArtClient {
    http: reqwest::Client,
    base_url: String,
    limiter: Arc<DirectRateLimiter>,
}

impl CoverArtClient {
    pub fn new(user_agent: &str, base_url: String) -> MbResult<Self> {
        let http = reqwest::Client::builder()
            .user_agent(user_agent)
            .timeout(Duration::from_secs(60))
            .build()
            .map_err(MbError::Http)?;
        let quota = Quota::per_second(NonZeroU32::new(1).expect("1 is nonzero"));
        Ok(Self {
            http,
            base_url: ensure_trailing_slash(base_url),
            limiter: Arc::new(RateLimiter::direct(quota)),
        })
    }

    /// Front cover for a specific release by MBID. `Ok(None)` when CAA has
    /// nothing (404 is a normal outcome -- not an error).
    pub async fn fetch_release_front(&self, release_mbid: &str) -> MbResult<Option<Vec<u8>>> {
        self.fetch(&format!("release/{}/front-500", release_mbid))
            .await
    }

    /// Front cover of a release-group. CAA serves a representative release's
    /// front when one exists. `Ok(None)` on 404.
    pub async fn fetch_release_group_front(
        &self,
        release_group_mbid: &str,
    ) -> MbResult<Option<Vec<u8>>> {
        self.fetch(&format!("release-group/{}/front-500", release_group_mbid))
            .await
    }

    /// Legacy alias retained for the tests added in the initial wiremock pass.
    #[cfg(test)]
    pub async fn fetch_front(&self, release_mbid: &str) -> MbResult<Option<Vec<u8>>> {
        self.fetch_release_front(release_mbid).await
    }

    async fn fetch(&self, path_suffix: &str) -> MbResult<Option<Vec<u8>>> {
        self.limiter.until_ready().await;
        let url = format!("{}{}", self.base_url, path_suffix);
        let resp = self.http.get(&url).send().await.map_err(MbError::Http)?;
        match resp.status() {
            s if s.is_success() => {
                let bytes = resp.bytes().await.map_err(MbError::Http)?;
                Ok(Some(bytes.to_vec()))
            }
            StatusCode::NOT_FOUND => Ok(None),
            other => Err(MbError::Status(other)),
        }
    }
}

fn ensure_trailing_slash(mut url: String) -> String {
    if !url.ends_with('/') {
        url.push('/');
    }
    url
}
