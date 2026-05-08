use amaterasu_macros::api_type;
use garde::Validate;
use serde::{Deserialize, Serialize};

use crate::filters::FilterNode;

#[api_type("request/album-collection")]
#[derive(Serialize, Deserialize, Validate)]
pub struct CreateAlbumCollectionParams {
    #[garde(length(min = 1, max = 255))]
    pub name: String,
    #[garde(skip)]
    pub filter_definition: FilterNode,
}

#[api_type("request/album-collection")]
#[derive(Serialize, Deserialize, Validate)]
pub struct RenameAlbumCollectionParams {
    #[garde(length(min = 1, max = 255))]
    pub name: String,
}

#[api_type("request/album-collection")]
#[derive(Serialize, Deserialize, Validate)]
pub struct UpdateAlbumCollectionFilterParams {
    #[garde(skip)]
    pub filter_definition: FilterNode,
}
