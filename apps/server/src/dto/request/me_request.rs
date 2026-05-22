use amaterasu_macros::api_type;
use garde::Validate;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[api_type("request/me")]
#[derive(Serialize, Deserialize, Validate)]
pub struct PinPlaylistParams {
    #[garde(skip)]
    pub playlist_id: Uuid,
}

#[api_type("request/me")]
#[derive(Serialize, Deserialize, Validate)]
pub struct ReorderPinnedPlaylistsParams {
    #[garde(length(min = 1, max = 6))]
    pub ordered_playlist_ids: Vec<Uuid>,
}
