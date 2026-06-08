use std::path::PathBuf;
use std::sync::Arc;

use axum::extract::FromRef;
use sqlx::PgPool;

use crate::musicbrainz::{LookupSender, MetadataSuggestionService};
use crate::scanner::LibraryScanner;
use crate::search::SearchIndex;
use crate::services::RecommendationCache;

#[derive(Clone, FromRef)]
pub struct AppState {
    pub db: PgPool,
    pub library_scanner: LibraryScanner,
    pub covers_dir: PathBuf,
    pub search: Arc<SearchIndex>,
    pub grafana_proxy: Option<crate::handlers::grafana_proxy_handlers::GrafanaProxy>,
    /// `None` when `LOKI_URL` is unset; frontend logs then fall back to the
    /// server's own tracing output.
    pub client_log: Option<crate::handlers::client_log_handlers::LokiIngest>,
    /// `None` when MUSICBRAINZ_ENABLED=false.
    pub mb_service: Option<MetadataSuggestionService>,
    pub mb_lookup_sender: Option<LookupSender>,
    pub recommendation_cache: Arc<RecommendationCache>,
    pub trust_proxy_headers: bool,
}

impl FromRef<AppState> for () {
    fn from_ref(_: &AppState) {}
}

impl AppState {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        db: PgPool,
        library_scanner: LibraryScanner,
        covers_dir: PathBuf,
        search: Arc<SearchIndex>,
        grafana_proxy: Option<crate::handlers::grafana_proxy_handlers::GrafanaProxy>,
        client_log: Option<crate::handlers::client_log_handlers::LokiIngest>,
        mb_service: Option<MetadataSuggestionService>,
        mb_lookup_sender: Option<LookupSender>,
        trust_proxy_headers: bool,
    ) -> Self {
        Self {
            db,
            library_scanner,
            covers_dir,
            search,
            grafana_proxy,
            client_log,
            mb_service,
            mb_lookup_sender,
            recommendation_cache: Arc::new(RecommendationCache::new()),
            trust_proxy_headers,
        }
    }
}
