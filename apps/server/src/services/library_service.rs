use std::collections::HashMap;

use sqlx::PgPool;
use uuid::Uuid;

use crate::db::entities::{Album, Artist, Track};
use crate::dto::request::SortDir;
use crate::error::AppResult;
use crate::filters::FilterNode;
use crate::repositories::{
    AlbumRepository, AlbumSortKey, ArtistRepository, ArtistSortKey, TrackPlayRepository,
    TrackRepository, TrackSortKey,
};

pub struct TrackWithRefs {
    pub track: Track,
    pub album: Option<Album>,
    pub artist: Option<Artist>,
    pub play_count: i64,
}

pub struct AlbumWithRefs {
    pub album: Album,
    pub artist: Option<Artist>,
    pub track_count: i64,
    pub total_duration_ms: i64,
    pub play_count: i64,
}

pub struct ArtistWithRefs {
    pub artist: Artist,
    pub album_count: i64,
    pub track_count: i64,
    pub play_count: i64,
}

pub struct LibraryService {
    pool: PgPool,
    user_id: Uuid,
}

impl LibraryService {
    pub fn new(pool: PgPool, user_id: Uuid) -> Self {
        Self { pool, user_id }
    }

    pub async fn get_tracks(
        &self,
        filter: Option<&FilterNode>,
        limit: i32,
        offset: i32,
        sort: Option<TrackSortKey>,
        dir: Option<SortDir>,
    ) -> AppResult<(Vec<TrackWithRefs>, i64)> {
        let tracks = TrackRepository::find(&self.pool, filter, limit, offset, sort, dir).await?;
        let total = TrackRepository::count(&self.pool, filter).await?;
        let bundled = self.attach_refs(tracks).await?;

        Ok((bundled, total))
    }

    pub async fn get_track_by_id(&self, id: Uuid) -> AppResult<Option<TrackWithRefs>> {
        let Some(track) = TrackRepository::find_by_id(&self.pool, id).await? else {
            return Ok(None);
        };
        let mut bundled = self.attach_refs(vec![track]).await?;
        Ok(bundled.pop())
    }

    pub async fn get_albums(
        &self,
        filter: Option<&FilterNode>,
        limit: i32,
        offset: i32,
        sort: Option<AlbumSortKey>,
        dir: Option<SortDir>,
    ) -> AppResult<(Vec<AlbumWithRefs>, i64)> {
        let albums = AlbumRepository::find(&self.pool, filter, limit, offset, sort, dir).await?;
        let total = AlbumRepository::count(&self.pool, filter).await?;
        let bundled = self.bundle_albums(albums).await?;
        Ok((bundled, total))
    }

    pub async fn get_album_by_id(&self, album_id: Uuid) -> AppResult<Option<AlbumWithRefs>> {
        let album = AlbumRepository::find_by_id(&self.pool, album_id).await?;
        let mut bundled = self.bundle_albums(vec![album]).await?;
        Ok(bundled.pop())
    }

    pub async fn get_tracks_by_album_id(&self, album_id: Uuid) -> AppResult<Vec<TrackWithRefs>> {
        let tracks = TrackRepository::find_by_album_id(&self.pool, album_id).await?;
        self.attach_refs(tracks).await
    }

    pub async fn get_artists_with_query(
        &self,
        query: Option<&str>,
        limit: i32,
        offset: i32,
        sort: Option<ArtistSortKey>,
        dir: Option<SortDir>,
    ) -> AppResult<(Vec<ArtistWithRefs>, i64)> {
        let artists =
            ArtistRepository::find_all_with_query(&self.pool, query, limit, offset, sort, dir)
                .await?;
        let total = ArtistRepository::count_with_query(&self.pool, query).await?;
        let bundled = self.bundle_artists(artists).await?;
        Ok((bundled, total))
    }

    pub async fn get_artist_by_id(&self, id: Uuid) -> AppResult<Option<ArtistWithRefs>> {
        let artists = ArtistRepository::find_by_ids(&self.pool, &[id]).await?;
        let Some(artist) = artists.into_iter().next() else {
            return Ok(None);
        };
        let mut bundled = self.bundle_artists(vec![artist]).await?;
        Ok(bundled.pop())
    }

    pub async fn get_tracks_by_artist_id(&self, artist_id: Uuid) -> AppResult<Vec<TrackWithRefs>> {
        let tracks = TrackRepository::find_by_artist_id(&self.pool, artist_id).await?;
        self.attach_refs(tracks).await
    }

    pub async fn get_albums_by_artist_id(&self, artist_id: Uuid) -> AppResult<Vec<AlbumWithRefs>> {
        let albums = AlbumRepository::find_by_artist_id(&self.pool, artist_id).await?;
        self.bundle_albums(albums).await
    }

    async fn bundle_albums(&self, albums: Vec<Album>) -> AppResult<Vec<AlbumWithRefs>> {
        let artist_ids: Vec<Uuid> = albums.iter().filter_map(|a| a.artist_id).collect();
        let album_ids: Vec<Uuid> = albums.iter().map(|a| a.id).collect();

        let artists = ArtistRepository::find_by_ids(&self.pool, &artist_ids).await?;
        let track_stats =
            AlbumRepository::get_track_stats_for_album_ids(&self.pool, &album_ids).await?;
        let play_counts =
            TrackPlayRepository::play_counts_for_albums(&self.pool, self.user_id, &album_ids)
                .await?;

        let artists_by_id: HashMap<Uuid, Artist> = artists.into_iter().map(|a| (a.id, a)).collect();
        let stats_by_album_id: HashMap<Uuid, (i64, i64)> = track_stats
            .into_iter()
            .map(|(id, count, dur)| (id, (count, dur)))
            .collect();
        let plays_by_album_id: HashMap<Uuid, i64> = play_counts.into_iter().collect();

        Ok(albums
            .into_iter()
            .map(|album| {
                let artist = album
                    .artist_id
                    .and_then(|id| artists_by_id.get(&id).cloned());
                let (track_count, total_duration_ms) =
                    stats_by_album_id.get(&album.id).copied().unwrap_or((0, 0));
                let play_count = plays_by_album_id.get(&album.id).copied().unwrap_or(0);
                AlbumWithRefs {
                    album,
                    artist,
                    track_count,
                    total_duration_ms,
                    play_count,
                }
            })
            .collect())
    }

    async fn bundle_artists(&self, artists: Vec<Artist>) -> AppResult<Vec<ArtistWithRefs>> {
        let artist_ids: Vec<Uuid> = artists.iter().map(|a| a.id).collect();
        let album_counts =
            AlbumRepository::get_album_count_for_artist_ids(&self.pool, &artist_ids).await?;
        let track_counts =
            TrackRepository::get_track_count_for_artist_ids(&self.pool, &artist_ids).await?;
        let play_counts =
            TrackPlayRepository::play_counts_for_artists(&self.pool, self.user_id, &artist_ids)
                .await?;
        let albums_by_id: HashMap<Uuid, i64> = album_counts.into_iter().collect();
        let tracks_by_id: HashMap<Uuid, i64> = track_counts.into_iter().collect();
        let plays_by_id: HashMap<Uuid, i64> = play_counts.into_iter().collect();

        Ok(artists
            .into_iter()
            .map(|artist| {
                let album_count = albums_by_id.get(&artist.id).copied().unwrap_or(0);
                let track_count = tracks_by_id.get(&artist.id).copied().unwrap_or(0);
                let play_count = plays_by_id.get(&artist.id).copied().unwrap_or(0);
                ArtistWithRefs {
                    artist,
                    album_count,
                    track_count,
                    play_count,
                }
            })
            .collect())
    }

    async fn attach_refs(&self, tracks: Vec<Track>) -> AppResult<Vec<TrackWithRefs>> {
        let album_ids: Vec<Uuid> = tracks.iter().filter_map(|t| t.album_id).collect();
        let artist_ids: Vec<Uuid> = tracks.iter().filter_map(|t| t.artist_id).collect();
        let track_ids: Vec<Uuid> = tracks.iter().map(|t| t.id).collect();

        let albums = AlbumRepository::find_by_ids(&self.pool, &album_ids).await?;
        let artists = ArtistRepository::find_by_ids(&self.pool, &artist_ids).await?;
        let play_counts =
            TrackPlayRepository::play_counts_for_tracks(&self.pool, self.user_id, &track_ids)
                .await?;

        let albums_by_id: HashMap<Uuid, Album> = albums.into_iter().map(|a| (a.id, a)).collect();
        let artists_by_id: HashMap<Uuid, Artist> = artists.into_iter().map(|a| (a.id, a)).collect();
        let plays_by_id: HashMap<Uuid, i64> = play_counts.into_iter().collect();

        Ok(tracks
            .into_iter()
            .map(|track| {
                let album = track.album_id.and_then(|id| albums_by_id.get(&id).cloned());
                let artist = track
                    .artist_id
                    .and_then(|id| artists_by_id.get(&id).cloned());
                let play_count = plays_by_id.get(&track.id).copied().unwrap_or(0);
                TrackWithRefs {
                    track,
                    album,
                    artist,
                    play_count,
                }
            })
            .collect())
    }
}
