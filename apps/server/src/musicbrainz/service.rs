//! High-level metadata-suggestion service: orchestrates MB lookups, persists
//! candidates, applies accepted candidates back onto entities, and downloads
//! cover art on accept.

// `&mut *tx` is the project-wide pattern for reborrowing a `Transaction` as
// `&mut PgConnection` -- repositories take `impl PgExecutor<'_>`, which is
// implemented for `&mut PgConnection` but not `&mut Transaction`. Clippy
// thinks Deref would auto-do this; it can't, here.
#![allow(clippy::explicit_auto_deref)]

use std::collections::HashSet;
use std::path::PathBuf;

use serde_json::{json, Value as JsonValue};
use sqlx::PgPool;
use uuid::Uuid;

use crate::db::entities::{Album, Artist, Track};
use crate::error::{AppError, AppResult};
use crate::repositories::{
    AlbumRepository, ArtistRepository, MbLookupStatusRepository, MetadataSuggestion,
    MetadataSuggestionRepository, NewSuggestion, SuggestionEntityType, TrackRepository,
};
use crate::services::cover_storage;

use super::client::MusicBrainzClient;
use super::cover_art::CoverArtClient;
use super::error::MbError;
use super::mapping::{
    artist_to_candidate, recording_to_candidate, release_group_to_album_candidate, AlbumProposal,
    ArtistProposal, TrackProposal,
};

#[derive(Clone)]
pub struct MetadataSuggestionService {
    pool: PgPool,
    covers_dir: PathBuf,
    mb: MusicBrainzClient,
    caa: CoverArtClient,
    max_candidates: usize,
}

impl MetadataSuggestionService {
    pub fn new(
        pool: PgPool,
        covers_dir: PathBuf,
        mb: MusicBrainzClient,
        caa: CoverArtClient,
        max_candidates: usize,
    ) -> Self {
        Self {
            pool,
            covers_dir,
            mb,
            caa,
            max_candidates,
        }
    }

    // ------------------------------------------------------------------
    // Lookups
    // ------------------------------------------------------------------

    /// Queries MB for the album, persists candidates, stamps lookup status.
    /// Errors are logged inside; this returns `Ok(num_candidates)` on success
    /// (even when zero) and `Err` only for unrecoverable DB issues. Network
    /// failures get folded into `mb_lookup_status='failed'` and Ok(0).
    ///
    /// If the album already has a successful lookup (`mb_lookup_status='found'`)
    /// this is a no-op. Manual triggers from the admin UI bypass this via
    /// [`lookup_album_force`].
    pub async fn lookup_album(&self, album_id: Uuid) -> AppResult<usize> {
        self.lookup_album_inner(album_id, false).await
    }

    /// Force a fresh lookup even when status is `'found'`. Wired to the
    /// manual "Lookup on MB" admin button so an admin who's unhappy with
    /// existing candidates can replace them.
    pub async fn lookup_album_force(&self, album_id: Uuid) -> AppResult<usize> {
        self.lookup_album_inner(album_id, true).await
    }

    async fn lookup_album_inner(&self, album_id: Uuid, force: bool) -> AppResult<usize> {
        let album = match AlbumRepository::find_by_id(&self.pool, album_id).await {
            Ok(a) => a,
            Err(AppError::Database(sqlx::Error::RowNotFound)) => {
                return Err(AppError::NotFound);
            }
            Err(e) => return Err(e),
        };

        if !force && album.mb_lookup_status.as_deref() == Some("found") {
            tracing::debug!(album_id = %album_id, "mb: skipping album already 'found'");
            return Ok(0);
        }

        let artist_name = match album.artist_id {
            Some(aid) => ArtistRepository::find_by_id(&self.pool, aid)
                .await?
                .map(|a| a.name),
            None => None,
        };

        let resp = match self
            .mb
            .search_release_group(artist_name.as_deref(), &album.title, self.max_candidates)
            .await
        {
            Ok(r) => r,
            Err(e) => {
                tracing::warn!(album_id = %album_id, "musicbrainz album lookup failed: {}", e);
                MbLookupStatusRepository::mark_album(&self.pool, album_id, "failed").await?;
                return Ok(0);
            }
        };

        let rejected: HashSet<String> = MetadataSuggestionRepository::rejected_mbids_for_entity(
            &self.pool,
            SuggestionEntityType::Album,
            album_id,
        )
        .await?
        .into_iter()
        .collect();

        let candidates: Vec<NewSuggestion> = resp
            .release_groups
            .iter()
            .filter_map(release_group_to_album_candidate)
            .filter(|c| !rejected.contains(&c.mbid))
            .take(self.max_candidates)
            .map(|c| NewSuggestion {
                mbid: c.mbid,
                score: c.score,
                proposed: proposed_album_json(&c.proposal),
                raw: None,
            })
            .collect();

        let count = candidates.len();
        let status = if count == 0 { "not_found" } else { "found" };

        let mut tx = self.pool.begin().await?;
        MetadataSuggestionRepository::replace_pending_for_entity(
            &mut *tx,
            SuggestionEntityType::Album,
            album_id,
            &candidates,
        )
        .await?;
        MbLookupStatusRepository::mark_album(&mut *tx, album_id, status).await?;
        tx.commit().await?;
        Ok(count)
    }

    pub async fn lookup_artist(&self, artist_id: Uuid) -> AppResult<usize> {
        self.lookup_artist_inner(artist_id, false).await
    }

    pub async fn lookup_artist_force(&self, artist_id: Uuid) -> AppResult<usize> {
        self.lookup_artist_inner(artist_id, true).await
    }

    async fn lookup_artist_inner(&self, artist_id: Uuid, force: bool) -> AppResult<usize> {
        let artist = ArtistRepository::find_by_id(&self.pool, artist_id)
            .await?
            .ok_or(AppError::NotFound)?;
        if !force && artist.mb_lookup_status.as_deref() == Some("found") {
            tracing::debug!(artist_id = %artist_id, "mb: skipping artist already 'found'");
            return Ok(0);
        }

        let resp = match self
            .mb
            .search_artist(&artist.name, self.max_candidates)
            .await
        {
            Ok(r) => r,
            Err(e) => {
                tracing::warn!(artist_id = %artist_id, "musicbrainz artist lookup failed: {}", e);
                MbLookupStatusRepository::mark_artist(&self.pool, artist_id, "failed").await?;
                return Ok(0);
            }
        };

        let rejected: HashSet<String> = MetadataSuggestionRepository::rejected_mbids_for_entity(
            &self.pool,
            SuggestionEntityType::Artist,
            artist_id,
        )
        .await?
        .into_iter()
        .collect();

        let candidates: Vec<NewSuggestion> = resp
            .artists
            .iter()
            .map(artist_to_candidate)
            .filter(|c| !rejected.contains(&c.mbid))
            .take(self.max_candidates)
            .map(|c| NewSuggestion {
                mbid: c.mbid,
                score: c.score,
                proposed: proposed_artist_json(&c.proposal),
                raw: None,
            })
            .collect();

        let count = candidates.len();
        let status = if count == 0 { "not_found" } else { "found" };

        let mut tx = self.pool.begin().await?;
        MetadataSuggestionRepository::replace_pending_for_entity(
            &mut *tx,
            SuggestionEntityType::Artist,
            artist_id,
            &candidates,
        )
        .await?;
        MbLookupStatusRepository::mark_artist(&mut *tx, artist_id, status).await?;
        tx.commit().await?;
        Ok(count)
    }

    pub async fn lookup_track(&self, track_id: Uuid) -> AppResult<usize> {
        self.lookup_track_inner(track_id, false).await
    }

    pub async fn lookup_track_force(&self, track_id: Uuid) -> AppResult<usize> {
        self.lookup_track_inner(track_id, true).await
    }

    async fn lookup_track_inner(&self, track_id: Uuid, force: bool) -> AppResult<usize> {
        let track = TrackRepository::find_by_id(&self.pool, track_id)
            .await?
            .ok_or(AppError::NotFound)?;
        if !force && track.mb_lookup_status.as_deref() == Some("found") {
            tracing::debug!(track_id = %track_id, "mb: skipping track already 'found'");
            return Ok(0);
        }
        let artist_name = match track.artist_id {
            Some(aid) => ArtistRepository::find_by_id(&self.pool, aid)
                .await?
                .map(|a| a.name),
            None => None,
        };
        let album_title = match track.album_id {
            Some(aid) => AlbumRepository::find_by_id(&self.pool, aid)
                .await
                .ok()
                .map(|a| a.title),
            None => None,
        };

        let resp = match self
            .mb
            .search_recording(
                artist_name.as_deref(),
                album_title.as_deref(),
                &track.title,
                self.max_candidates,
            )
            .await
        {
            Ok(r) => r,
            Err(e) => {
                tracing::warn!(track_id = %track_id, "musicbrainz track lookup failed: {}", e);
                MbLookupStatusRepository::mark_track(&self.pool, track_id, "failed").await?;
                return Ok(0);
            }
        };

        let rejected: HashSet<String> = MetadataSuggestionRepository::rejected_mbids_for_entity(
            &self.pool,
            SuggestionEntityType::Track,
            track_id,
        )
        .await?
        .into_iter()
        .collect();

        let candidates: Vec<NewSuggestion> = resp
            .recordings
            .iter()
            .map(recording_to_candidate)
            .filter(|c| !rejected.contains(&c.mbid))
            .take(self.max_candidates)
            .map(|c| NewSuggestion {
                mbid: c.mbid,
                score: c.score,
                proposed: proposed_track_json(&c.proposal),
                raw: None,
            })
            .collect();

        let count = candidates.len();
        let status = if count == 0 { "not_found" } else { "found" };

        let mut tx = self.pool.begin().await?;
        MetadataSuggestionRepository::replace_pending_for_entity(
            &mut *tx,
            SuggestionEntityType::Track,
            track_id,
            &candidates,
        )
        .await?;
        MbLookupStatusRepository::mark_track(&mut *tx, track_id, status).await?;
        tx.commit().await?;
        Ok(count)
    }

    // ------------------------------------------------------------------
    // Accept / reject
    // ------------------------------------------------------------------

    /// Applies the suggestion's `proposed` fields back onto the parent
    /// entity, stamps the MBID, optionally downloads the cover, and marks
    /// sibling pending suggestions as `superseded`. Returns the freshly read
    /// entity in its `AcceptedEntity` variant so the handler can re-index
    /// the search index.
    pub async fn accept_suggestion(&self, id: Uuid) -> AppResult<AcceptedEntity> {
        let s = MetadataSuggestionRepository::find_by_id(&self.pool, id)
            .await?
            .ok_or(AppError::NotFound)?;
        let entity_type = SuggestionEntityType::parse(&s.entity_type).ok_or_else(|| {
            AppError::Internal(anyhow::anyhow!(
                "invalid entity_type stored: {}",
                s.entity_type
            ))
        })?;

        match entity_type {
            SuggestionEntityType::Album => self.accept_album_suggestion(s).await,
            SuggestionEntityType::Artist => self.accept_artist_suggestion(s).await,
            SuggestionEntityType::Track => self.accept_track_suggestion(s).await,
        }
    }

    async fn accept_album_suggestion(&self, s: MetadataSuggestion) -> AppResult<AcceptedEntity> {
        let proposal: AlbumProposal = serde_json::from_value(s.proposed.clone())
            .map_err(|e| AppError::Internal(anyhow::anyhow!("bad album proposal: {}", e)))?;
        let current = AlbumRepository::find_by_id(&self.pool, s.entity_id).await?;

        let new_title = proposal
            .title
            .clone()
            .unwrap_or_else(|| current.title.clone());
        let new_sort = proposal
            .sort_title
            .clone()
            .or_else(|| Some(current.sort_title.clone()))
            .unwrap_or_else(|| new_title.clone());
        let new_date = proposal.date.or(current.date);
        let new_artist_id = current.artist_id; // artist re-pointing requires a separate decision

        let mut tx = self.pool.begin().await?;
        AlbumRepository::update(
            &mut *tx,
            s.entity_id,
            &new_title,
            &new_sort,
            new_artist_id,
            new_date,
        )
        .await?;
        AlbumRepository::set_mbid(&mut *tx, s.entity_id, &proposal.mbid).await?;
        MetadataSuggestionRepository::accept_and_supersede_siblings(&mut *tx, s.id).await?;
        tx.commit().await?;

        // Cover download happens *after* the transaction so a slow CAA fetch
        // doesn't hold a DB connection. Failure here is non-fatal -- the rest
        // of the accept already landed. Prefer the specific release MBID if
        // present (more accurate); fall back to the release-group MBID which
        // CAA also serves.
        let cover_mbid_kind = match proposal.primary_release_mbid.as_deref() {
            Some(rel) => CoverFetchTarget::Release(rel.to_string()),
            None => CoverFetchTarget::ReleaseGroup(proposal.release_group_mbid.clone()),
        };
        if let Err(e) = self.try_apply_cover(s.entity_id, cover_mbid_kind).await {
            tracing::warn!(album_id = %s.entity_id, "cover apply failed: {}", e);
        }

        let updated = AlbumRepository::find_by_id(&self.pool, s.entity_id).await?;
        Ok(AcceptedEntity::Album(Box::new(updated)))
    }

    async fn accept_artist_suggestion(&self, s: MetadataSuggestion) -> AppResult<AcceptedEntity> {
        let proposal: ArtistProposal = serde_json::from_value(s.proposed.clone())
            .map_err(|e| AppError::Internal(anyhow::anyhow!("bad artist proposal: {}", e)))?;
        let current = ArtistRepository::find_by_id(&self.pool, s.entity_id)
            .await?
            .ok_or(AppError::NotFound)?;
        let new_name = proposal
            .name
            .clone()
            .unwrap_or_else(|| current.name.clone());
        let new_sort = proposal
            .sort_name
            .clone()
            .unwrap_or_else(|| current.sort_name.clone());

        let mut tx = self.pool.begin().await?;
        ArtistRepository::update(&mut *tx, s.entity_id, &new_name, &new_sort).await?;
        ArtistRepository::set_mbid(&mut *tx, s.entity_id, &proposal.mbid).await?;
        MetadataSuggestionRepository::accept_and_supersede_siblings(&mut *tx, s.id).await?;
        tx.commit().await?;

        let updated = ArtistRepository::find_by_id(&self.pool, s.entity_id)
            .await?
            .ok_or(AppError::NotFound)?;
        Ok(AcceptedEntity::Artist(Box::new(updated)))
    }

    async fn accept_track_suggestion(&self, s: MetadataSuggestion) -> AppResult<AcceptedEntity> {
        let proposal: TrackProposal = serde_json::from_value(s.proposed.clone())
            .map_err(|e| AppError::Internal(anyhow::anyhow!("bad track proposal: {}", e)))?;
        // For tracks we don't yet have a generic mbid column; we still apply
        // the title (if proposed) via admin_update and mark the suggestion
        // accepted. Future work: add tracks.mbid.
        let mut patch = crate::repositories::track_repository::TrackUpdate::default();
        if let Some(title) = proposal.title.clone() {
            patch.title = Some(title);
        }
        let updated = TrackRepository::admin_update(&self.pool, s.entity_id, &patch).await?;
        MetadataSuggestionRepository::accept_and_supersede_siblings(&self.pool, s.id).await?;
        Ok(AcceptedEntity::Track(Box::new(updated)))
    }

    pub async fn reject_suggestion(&self, id: Uuid) -> AppResult<()> {
        let s = MetadataSuggestionRepository::find_by_id(&self.pool, id)
            .await?
            .ok_or(AppError::NotFound)?;
        let entity_type = SuggestionEntityType::parse(&s.entity_type).ok_or_else(|| {
            AppError::Internal(anyhow::anyhow!(
                "invalid entity_type stored: {}",
                s.entity_type
            ))
        })?;
        let mut tx = self.pool.begin().await?;
        MetadataSuggestionRepository::set_status(&mut *tx, id, "rejected").await?;
        MetadataSuggestionRepository::record_rejection(&mut *tx, entity_type, s.entity_id, &s.mbid)
            .await?;
        tx.commit().await?;
        Ok(())
    }

    // ------------------------------------------------------------------
    // Cover-art download
    // ------------------------------------------------------------------

    async fn try_apply_cover(
        &self,
        album_id: Uuid,
        target: CoverFetchTarget,
    ) -> Result<(), MbError> {
        let bytes = match &target {
            CoverFetchTarget::Release(mbid) => self.caa.fetch_release_front(mbid).await?,
            CoverFetchTarget::ReleaseGroup(mbid) => {
                self.caa.fetch_release_group_front(mbid).await?
            }
        };
        let Some(bytes) = bytes else {
            tracing::debug!(album_id = %album_id, ?target, "no CAA front cover");
            return Ok(());
        };

        let filename = match cover_storage::store_cover_bytes(&self.covers_dir, &bytes).await {
            Ok(f) => f,
            Err(e) => {
                tracing::warn!(album_id = %album_id, "failed to store CAA cover: {}", e);
                return Ok(());
            }
        };
        // set_cover_path is a write that happens on the pool, not a tx --
        // we're past the transactional accept. Any failure here gets logged
        // and swallowed.
        if let Err(e) =
            AlbumRepository::set_cover_path(&self.pool, album_id, Some(filename.as_str())).await
        {
            tracing::warn!(album_id = %album_id, "failed to set cover_path: {}", e);
        }
        Ok(())
    }
}

#[derive(Debug)]
pub enum AcceptedEntity {
    Album(Box<Album>),
    Artist(Box<Artist>),
    Track(Box<Track>),
}

/// Which CAA endpoint to hit when fetching cover art for an album. The MB
/// release-group search rarely includes child releases, so we usually fall
/// back to release-group covers.
#[derive(Debug)]
enum CoverFetchTarget {
    Release(String),
    ReleaseGroup(String),
}

fn proposed_album_json(p: &AlbumProposal) -> JsonValue {
    serde_json::to_value(p).unwrap_or_else(|_| json!({}))
}

fn proposed_artist_json(p: &ArtistProposal) -> JsonValue {
    serde_json::to_value(p).unwrap_or_else(|_| json!({}))
}

fn proposed_track_json(p: &TrackProposal) -> JsonValue {
    serde_json::to_value(p).unwrap_or_else(|_| json!({}))
}
