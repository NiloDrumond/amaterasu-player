use amaterasu_macros::api_type;
use serde::Serialize;

use crate::dto::response::{AlbumResponse, PlaylistResponse};

/// Heterogeneous homepage item — either an album or a playlist. Used by
/// "Listen again" and "Forgotten favorites" where both kinds of context can
/// surface in the same ordered list.
#[api_type("response/me")]
#[derive(Debug, Serialize)]
#[serde(tag = "kind", content = "data")]
pub enum HomeItemResponse {
    Album(AlbumResponse),
    Playlist(PlaylistResponse),
}
