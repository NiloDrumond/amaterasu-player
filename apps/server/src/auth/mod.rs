pub mod error;
pub mod middleware;
pub mod password_hash;

pub const SESSION_DURATION_HOURS: u32 = 24 * 30;
pub const SESSION_COOKIE_NAME: &str = "SESSION";

pub use error::*;
pub use middleware::*;
pub use password_hash::*;
