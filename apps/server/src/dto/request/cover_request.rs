use amaterasu_macros::api_type;
use garde::Validate;
use serde::Deserialize;

#[api_type("request/cover")]
#[derive(Deserialize, Validate)]
pub struct CoverPath {
    #[garde(pattern(r"^[0-9a-f]{64}\.(jpg|png|webp|gif)$"))]
    pub filename: String,
}
