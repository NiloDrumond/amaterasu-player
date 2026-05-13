use amaterasu_macros::api_type;
use serde::Serialize;

use crate::db::entities::User;
use crate::dto::response::UserPreferences;

#[api_type("response/auth")]
#[derive(Debug, Serialize)]
pub struct CurrentUserResponse {
    name: String,
    email: String,
    role: String,
    preferences: UserPreferences,
}

impl From<User> for CurrentUserResponse {
    fn from(value: User) -> Self {
        Self {
            name: value.name,
            email: value.email,
            role: value.role,
            preferences: serde_json::from_value(value.preferences).unwrap_or_default(),
        }
    }
}
