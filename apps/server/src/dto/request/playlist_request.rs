use amaterasu_macros::api_type;
use garde::Validate;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::filters::FilterNode;

#[api_type("request/playlist")]
#[derive(Serialize, Deserialize, Validate)]
pub struct CreatePlaylistParams {
    #[garde(length(min = 1, max = 255))]
    pub name: String,
    /// If present, creates a dynamic playlist whose tracks are derived from
    /// this filter tree. Omit for a normal manual playlist.
    #[garde(skip)]
    #[serde(default)]
    pub filter_definition: Option<FilterNode>,
}

#[api_type("request/playlist")]
#[derive(Serialize, Deserialize, Validate)]
pub struct UpdatePlaylistFilterParams {
    #[garde(skip)]
    pub filter_definition: FilterNode,
}

#[api_type("request/playlist")]
#[derive(Serialize, Deserialize, Validate)]
pub struct RenamePlaylistParams {
    #[garde(length(min = 1, max = 255))]
    pub name: String,
}

#[api_type("request/playlist")]
#[derive(Serialize, Deserialize, Validate)]
pub struct AddTracksParams {
    #[garde(length(min = 1))]
    pub track_ids: Vec<Uuid>,
}

#[api_type("request/playlist")]
#[derive(Serialize, Deserialize, Validate)]
pub struct ReorderTrackParams {
    /// The track_id to insert after. None means move to front.
    #[garde(skip)]
    pub after_id: Option<Uuid>,
}
