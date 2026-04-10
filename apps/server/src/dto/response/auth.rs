use amaterasu_macros::api_type;
use serde::Serialize;

use crate::db::entities::User;

#[api_type("response/auth")]
#[derive(Debug, Serialize)]
pub struct CurrentUserResponse {
    name: String,
    email: String,
}

impl From<User> for CurrentUserResponse {
    fn from(value: User) -> Self {
        Self {
            name: value.name,
            email: value.email,
        }
    }
}
