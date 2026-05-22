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
    /// `None` when MUSICBRAINZ_ENABLED=false.
    pub mb_service: Option<MetadataSuggestionService>,
    pub mb_lookup_sender: Option<LookupSender>,
    pub recommendation_cache: Arc<RecommendationCache>,
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
        mb_service: Option<MetadataSuggestionService>,
        mb_lookup_sender: Option<LookupSender>,
    ) -> Self {
        Self {
            db,
            library_scanner,
            covers_dir,
            search,
            grafana_proxy,
            mb_service,
            mb_lookup_sender,
            recommendation_cache: Arc::new(RecommendationCache::new()),
        }
    }
}
