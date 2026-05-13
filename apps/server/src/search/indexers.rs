//! Per-entity write-through helpers. Each function is best-effort: failures
//! are logged but never propagated, so a failing index write cannot fail the
//! HTTP request that triggered it. The next startup rebuild will heal drift.

use uuid::Uuid;

use crate::db::entities::{Album, AlbumCollection, Artist, Playlist, Track};
use crate::dto::response::SearchEntityType;

use super::{IndexDoc, SearchIndex};

pub fn index_artist(index: &SearchIndex, artist: &Artist) {
    let doc = IndexDoc {
        kind: SearchEntityType::Artist,
        id: artist.id,
        text: artist.name.clone(),
        title: artist.name.clone(),
        subtitle: None,
        user_id: None,
    };
    if let Err(e) = index.upsert(&doc) {
        tracing::warn!("search: failed to index artist {}: {}", artist.id, e);
    }
}

pub fn index_album(index: &SearchIndex, album: &Album, artist_name: Option<&str>) {
    let mut text = album.title.clone();
    if let Some(an) = artist_name {
        text.push(' ');
        text.push_str(an);
    }
    let doc = IndexDoc {
        kind: SearchEntityType::Album,
        id: album.id,
        text,
        title: album.title.clone(),
        subtitle: artist_name.map(|s| s.to_string()),
        user_id: None,
    };
    if let Err(e) = index.upsert(&doc) {
        tracing::warn!("search: failed to index album {}: {}", album.id, e);
    }
}

pub fn index_track(
    index: &SearchIndex,
    track: &Track,
    artist_name: Option<&str>,
    album_title: Option<&str>,
) {
    let mut text = track.title.clone();
    if let Some(ot) = &track.original_title {
        if !ot.is_empty() {
            text.push(' ');
            text.push_str(ot);
        }
    }
    if let Some(an) = artist_name {
        text.push(' ');
        text.push_str(an);
    }
    let subtitle = match (artist_name, album_title) {
        (Some(a), Some(al)) => Some(format!("{} \u{00b7} {}", a, al)),
        (Some(a), None) => Some(a.to_string()),
        (None, Some(al)) => Some(al.to_string()),
        (None, None) => None,
    };
    let doc = IndexDoc {
        kind: SearchEntityType::Track,
        id: track.id,
        text,
        title: track.title.clone(),
        subtitle,
        user_id: None,
    };
    if let Err(e) = index.upsert(&doc) {
        tracing::warn!("search: failed to index track {}: {}", track.id, e);
    }
}

pub fn index_playlist(index: &SearchIndex, playlist: &Playlist) {
    let doc = IndexDoc {
        kind: SearchEntityType::Playlist,
        id: playlist.id,
        text: playlist.name.clone(),
        title: playlist.name.clone(),
        subtitle: Some("Playlist".into()),
        user_id: Some(playlist.user_id),
    };
    if let Err(e) = index.upsert(&doc) {
        tracing::warn!("search: failed to index playlist {}: {}", playlist.id, e);
    }
}

pub fn index_collection(index: &SearchIndex, collection: &AlbumCollection) {
    let doc = IndexDoc {
        kind: SearchEntityType::Collection,
        id: collection.id,
        text: collection.name.clone(),
        title: collection.name.clone(),
        subtitle: Some("Collection".into()),
        user_id: Some(collection.user_id),
    };
    if let Err(e) = index.upsert(&doc) {
        tracing::warn!(
            "search: failed to index collection {}: {}",
            collection.id,
            e
        );
    }
}

pub fn remove(index: &SearchIndex, kind: SearchEntityType, id: Uuid) {
    if let Err(e) = index.delete(kind, id) {
        tracing::warn!("search: failed to delete {}:{}: {}", kind.as_str(), id, e);
    }
}
