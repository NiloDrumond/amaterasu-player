pub mod client;
pub mod cover_art;
pub mod error;
pub mod mapping;
pub mod models;
pub mod service;
pub mod worker;

#[cfg(test)]
mod tests;

pub use client::MusicBrainzClient;
pub use cover_art::CoverArtClient;
pub use service::{AcceptedEntity, MetadataSuggestionService};
pub use worker::{spawn_worker, LookupJob, LookupSender};
