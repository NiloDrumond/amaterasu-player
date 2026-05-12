use amaterasu_macros::api_type;
use garde::Validate;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[api_type("request/track")]
#[derive(Serialize, Deserialize, Validate, Default)]
pub struct ScrobbleParams {
    #[garde(skip)]
    #[serde(default)]
    pub context_album_id: Option<Uuid>,
    #[garde(skip)]
    #[serde(default)]
    pub context_playlist_id: Option<Uuid>,
}
