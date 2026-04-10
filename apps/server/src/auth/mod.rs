pub mod error;
pub mod middleware;
pub mod password_hash;

use std::time::Duration;

pub use error::*;
pub use middleware::*;
pub use password_hash::*;

pub const SESSION_DURATION_HOURS: u32 = 24 * 30;
pub const SESSION_COOKIE_NAME: &str = "SESSION";
pub const MAX_USER_SESSIONS: i64 = 10;

pub const SESSION_CLEANUP_INTERVAL: Duration = Duration::from_secs(60 * 20);
