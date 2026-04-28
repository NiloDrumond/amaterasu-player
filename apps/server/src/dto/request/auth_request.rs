use amaterasu_macros::api_type;
use garde::Validate;
use serde::{Deserialize, Serialize};

#[api_type("request/auth")]
#[derive(Serialize, Deserialize, Validate)]
pub struct RegisterEmailParams {
    #[garde(email)]
    pub email: String,
    #[garde(length(min = 1, max = 100))]
    pub name: String,
    #[garde(length(min = 6, max = 100))]
    pub password: String,
}

#[api_type("request/auth")]
#[derive(Serialize, Deserialize, Validate)]
pub struct SignInEmailParams {
    #[garde(email)]
    pub email: String,
    #[garde(length(min = 6, max = 100))]
    pub password: String,
}
