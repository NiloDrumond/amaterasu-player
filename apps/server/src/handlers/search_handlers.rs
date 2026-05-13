use axum::{
    extract::{Query, State},
    Json,
};
use serde::Deserialize;

use crate::{
    auth::AuthUser,
    dto::response::{PaletteSearchResponse, SearchEntityType, SearchHit},
    error::{AppError, AppResult},
    search::SearchQueryOptions,
    state::AppState,
};

#[derive(Debug, Deserialize)]
pub struct PaletteSearchQuery {
    pub q: String,
    #[serde(default = "default_palette_limit")]
    pub limit: usize,
}

fn default_palette_limit() -> usize {
    5
}

pub async fn palette_search(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Query(params): Query<PaletteSearchQuery>,
) -> AppResult<Json<PaletteSearchResponse>> {
    let limit = params.limit.clamp(1, 20);
    let user_id = Some(auth_user.user.id);

    let mut resp = PaletteSearchResponse::default();
    for kind in SearchEntityType::all() {
        let hits = state
            .search
            .search(&SearchQueryOptions {
                q: &params.q,
                types: &[kind],
                user_id,
                limit,
                offset: 0,
            })
            .map_err(AppError::from)?;
        match kind {
            SearchEntityType::Track => resp.tracks = hits,
            SearchEntityType::Album => resp.albums = hits,
            SearchEntityType::Artist => resp.artists = hits,
            SearchEntityType::Playlist => resp.playlists = hits,
            SearchEntityType::Collection => resp.collections = hits,
        }
    }

    Ok(Json(resp))
}

#[derive(Debug, Deserialize)]
pub struct SearchAllQuery {
    pub q: String,
    /// One of: track, album, artist, playlist, collection.
    pub r#type: String,
    #[serde(default = "default_page")]
    pub page: usize,
    #[serde(default = "default_page_size")]
    pub page_size: usize,
}

fn default_page() -> usize {
    1
}

fn default_page_size() -> usize {
    25
}

pub async fn search_all(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Query(params): Query<SearchAllQuery>,
) -> AppResult<Json<Vec<SearchHit>>> {
    let kind = SearchEntityType::parse(&params.r#type)
        .ok_or_else(|| AppError::BadRequest(format!("unknown type: {}", params.r#type)))?;
    let page = params.page.max(1);
    let page_size = params.page_size.clamp(1, 100);
    let offset = (page - 1) * page_size;

    let hits = state
        .search
        .search(&SearchQueryOptions {
            q: &params.q,
            types: &[kind],
            user_id: Some(auth_user.user.id),
            limit: page_size,
            offset,
        })
        .map_err(AppError::from)?;

    Ok(Json(hits))
}
