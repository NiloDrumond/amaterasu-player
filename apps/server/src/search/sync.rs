//! Full rebuild of the search index from Postgres. Called once at server startup.

use std::collections::HashMap;

use anyhow::Result;
use sqlx::PgPool;
use uuid::Uuid;

use crate::dto::response::SearchEntityType;

use super::index::{IndexDoc, SearchIndex};

pub struct RebuildCounts {
    pub tracks: usize,
    pub albums: usize,
    pub artists: usize,
    pub playlists: usize,
    pub collections: usize,
}

pub async fn rebuild_from_postgres(pool: &PgPool, index: &SearchIndex) -> Result<RebuildCounts> {
    index.clear()?;

    // ----- Artists -----
    let artists: Vec<(Uuid, String)> = sqlx::query_as(r#"SELECT id, name FROM artists"#)
        .fetch_all(pool)
        .await?;
    let mut artist_name_by_id: HashMap<Uuid, String> = HashMap::with_capacity(artists.len());
    for (id, name) in &artists {
        artist_name_by_id.insert(*id, name.clone());
    }
    for (id, name) in &artists {
        index.upsert_no_commit(&IndexDoc {
            kind: SearchEntityType::Artist,
            id: *id,
            text: name.clone(),
            title: name.clone(),
            subtitle: None,
            user_id: None,
        })?;
    }

    // ----- Albums -----
    let albums: Vec<(Uuid, String, Option<Uuid>)> =
        sqlx::query_as(r#"SELECT id, title, artist_id FROM albums"#)
            .fetch_all(pool)
            .await?;
    for (id, title, artist_id) in &albums {
        let artist_name = artist_id.and_then(|a| artist_name_by_id.get(&a).cloned());
        let mut text = title.clone();
        if let Some(an) = &artist_name {
            text.push(' ');
            text.push_str(an);
        }
        index.upsert_no_commit(&IndexDoc {
            kind: SearchEntityType::Album,
            id: *id,
            text,
            title: title.clone(),
            subtitle: artist_name,
            user_id: None,
        })?;
    }

    // ----- Tracks -----
    // Skip soft-deleted.
    type TrackRow = (Uuid, String, Option<String>, Option<Uuid>, Option<Uuid>);
    let tracks: Vec<TrackRow> = sqlx::query_as(
        r#"SELECT id, title, original_title, artist_id, album_id
           FROM tracks
           WHERE deleted_at IS NULL"#,
    )
    .fetch_all(pool)
    .await?;

    let album_title_by_id: HashMap<Uuid, String> = albums
        .iter()
        .map(|(id, title, _)| (*id, title.clone()))
        .collect();

    for (id, title, original_title, artist_id, album_id) in &tracks {
        let artist_name = artist_id.and_then(|a| artist_name_by_id.get(&a).cloned());
        let album_title = album_id.and_then(|a| album_title_by_id.get(&a).cloned());

        let mut text = title.clone();
        if let Some(ot) = original_title {
            if !ot.is_empty() {
                text.push(' ');
                text.push_str(ot);
            }
        }
        if let Some(an) = &artist_name {
            text.push(' ');
            text.push_str(an);
        }

        let subtitle = match (&artist_name, &album_title) {
            (Some(a), Some(al)) => Some(format!("{} \u{00b7} {}", a, al)),
            (Some(a), None) => Some(a.clone()),
            (None, Some(al)) => Some(al.clone()),
            (None, None) => None,
        };

        index.upsert_no_commit(&IndexDoc {
            kind: SearchEntityType::Track,
            id: *id,
            text,
            title: title.clone(),
            subtitle,
            user_id: None,
        })?;
    }

    // ----- Playlists (user-scoped) -----
    let playlists: Vec<(Uuid, Uuid, String)> =
        sqlx::query_as(r#"SELECT id, user_id, name FROM playlists"#)
            .fetch_all(pool)
            .await?;
    for (id, user_id, name) in &playlists {
        index.upsert_no_commit(&IndexDoc {
            kind: SearchEntityType::Playlist,
            id: *id,
            text: name.clone(),
            title: name.clone(),
            subtitle: Some("Playlist".into()),
            user_id: Some(*user_id),
        })?;
    }

    // ----- Collections (user-scoped) -----
    let collections: Vec<(Uuid, Uuid, String)> =
        sqlx::query_as(r#"SELECT id, user_id, name FROM album_collections"#)
            .fetch_all(pool)
            .await?;
    for (id, user_id, name) in &collections {
        index.upsert_no_commit(&IndexDoc {
            kind: SearchEntityType::Collection,
            id: *id,
            text: name.clone(),
            title: name.clone(),
            subtitle: Some("Collection".into()),
            user_id: Some(*user_id),
        })?;
    }

    index.commit()?;

    Ok(RebuildCounts {
        tracks: tracks.len(),
        albums: albums.len(),
        artists: artists.len(),
        playlists: playlists.len(),
        collections: collections.len(),
    })
}
