use crate::db::entities::Track;
use crate::error::AppResult;
use crate::repositories::TrackRepository;
use sqlx::PgPool;
use uuid::Uuid;

pub struct LibraryService {
    track_repo: TrackRepository,
}

impl LibraryService {
    pub fn new(db: PgPool) -> Self {
        Self {
            track_repo: TrackRepository::new(db),
        }
    }

    pub async fn get_tracks(&self, limit: i64, offset: i64) -> AppResult<(Vec<Track>, i64)> {
        let tracks = self.track_repo.find_all(limit, offset).await?;
        let total = self.track_repo.count().await?;
        
        Ok((tracks, total))
    }

    pub async fn get_track_by_id(&self, id: Uuid) -> AppResult<Option<Track>> {
        self.track_repo.find_by_id(id).await
    }
}