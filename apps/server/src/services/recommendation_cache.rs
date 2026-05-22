use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};

use tokio::sync::RwLock;
use uuid::Uuid;

use crate::dto::response::HomeItemResponse;

/// Identifies which homepage recommendation list is being cached for a user.
/// Each kind owns its own freshness policy via [`RecommendationKind::ttl`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RecommendationKind {
    ListenAgain,
    ForgottenFavorites,
}

impl RecommendationKind {
    /// How long a cached result for this kind is considered fresh.
    /// `ListenAgain` reacts to "what did I play today" so we keep it tighter;
    /// `ForgottenFavorites` is intrinsically a long-tail signal and a full day
    /// of staleness is fine.
    pub fn ttl(self) -> Duration {
        match self {
            RecommendationKind::ListenAgain => Duration::from_secs(4 * 60 * 60),
            RecommendationKind::ForgottenFavorites => Duration::from_secs(24 * 60 * 60),
        }
    }
}

#[derive(Clone)]
struct Entry {
    computed_at: Instant,
    items: Arc<Vec<HomeItemResponse>>,
}

/// Per-user in-memory cache of recommendation lists. Wrapped in an `Arc` and
/// shared via `AppState`. TTL is determined per-kind by
/// [`RecommendationKind::ttl`].
///
/// Cache key is (user_id, kind, limit) so callers requesting different limits
/// don't poison each other's results.
pub struct RecommendationCache {
    inner: RwLock<HashMap<(Uuid, RecommendationKind, i64), Entry>>,
}

impl RecommendationCache {
    pub fn new() -> Self {
        Self {
            inner: RwLock::new(HashMap::new()),
        }
    }

    pub async fn get(
        &self,
        user_id: Uuid,
        kind: RecommendationKind,
        limit: i64,
    ) -> Option<Arc<Vec<HomeItemResponse>>> {
        let guard = self.inner.read().await;
        let entry = guard.get(&(user_id, kind, limit))?;
        if entry.computed_at.elapsed() < kind.ttl() {
            Some(Arc::clone(&entry.items))
        } else {
            None
        }
    }

    pub async fn put(
        &self,
        user_id: Uuid,
        kind: RecommendationKind,
        limit: i64,
        items: Vec<HomeItemResponse>,
    ) -> Arc<Vec<HomeItemResponse>> {
        let arc = Arc::new(items);
        let mut guard = self.inner.write().await;
        guard.insert(
            (user_id, kind, limit),
            Entry {
                computed_at: Instant::now(),
                items: Arc::clone(&arc),
            },
        );
        arc
    }
}

impl Default for RecommendationCache {
    fn default() -> Self {
        Self::new()
    }
}
