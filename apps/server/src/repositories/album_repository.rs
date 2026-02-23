use sqlx::PgExecutor;
use uuid::Uuid;

use crate::db::entities::Album;
use crate::error::AppError;

pub struct AlbumRepository;

impl AlbumRepository {
    pub async fn create(executor: impl PgExecutor<'_>, album: &Album) -> Result<Album, AppError> {
        let created = sqlx::query_as!(
            Album,
            r#"
            INSERT INTO albums (id, artist_id, title, sort_title, date, mbid, replaygain_album_gain, replaygain_album_peak, created_at, updated_at)
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            RETURNING
                *
            "#,
            album.id,
            album.artist_id,
            album.title,
            album.sort_title,
            album.date,
            album.mbid,
            album.replaygain_album_gain,
            album.replaygain_album_peak,
            album.created_at,
            album.updated_at
        )
        .fetch_one(executor)
        .await?;

        Ok(created)
    }

    pub async fn find_by_title_and_artist(
        executor: impl PgExecutor<'_>,
        title: &str,
        artist_id: Option<Uuid>,
    ) -> Result<Option<Album>, AppError> {
        let album = sqlx::query_as!(
            Album,
            r#"
            SELECT
                *
            FROM
                albums
            WHERE
                LOWER(title) = LOWER($1)
                AND (artist_id = $2
                    OR (artist_id IS NULL
                        AND $2 IS NULL))
            "#,
            title,
            artist_id
        )
        .fetch_optional(executor)
        .await?;

        Ok(album)
    }
}
