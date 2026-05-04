use amaterasu_macros::api_type;
use garde::Validate;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[api_type("request/tag-category")]
#[derive(Serialize, Deserialize, Validate)]
pub struct CreateTagCategoryParams {
    #[garde(length(min = 1, max = 64))]
    pub name: String,
    #[garde(inner(length(min = 1, max = 16)))]
    pub color: Option<String>,
}

#[api_type("request/tag-category")]
#[derive(Serialize, Deserialize, Validate)]
pub struct UpdateTagCategoryParams {
    #[garde(inner(length(min = 1, max = 64)))]
    pub name: Option<String>,
    #[garde(inner(length(min = 1, max = 16)))]
    pub color: Option<String>,
}

#[api_type("request/tag-category")]
#[derive(Serialize, Deserialize, Validate)]
pub struct ReorderTagCategoriesParams {
    #[garde(skip)]
    pub ordered_ids: Vec<Uuid>,
}
