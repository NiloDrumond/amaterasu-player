use serde::Serialize;

use crate::db::entities::User;

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
