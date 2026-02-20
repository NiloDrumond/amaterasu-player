use crate::db::entities::Track;
use crate::error::AppResult;
use crate::repositories::TrackRepository;
use sqlx::PgPool;
use uuid::Uuid;

pub struct LibraryService {
    pool: PgPool,
}

impl LibraryService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn get_tracks(&self, limit: i64, offset: i64) -> AppResult<(Vec<Track>, i64)> {
        let tracks = TrackRepository::find_all(&self.pool, limit, offset).await?;
        let total = TrackRepository::count(&self.pool).await?;

        Ok((tracks, total))
    }

    pub async fn get_track_by_id(&self, id: Uuid) -> AppResult<Option<Track>> {
        TrackRepository::find_by_id(&self.pool, id).await
    }
}
