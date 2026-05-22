//! Background worker that drains lookup jobs at the MB rate limit. Owns the
//! channel sender exposed on `AppState`; the scanner and admin handlers push
//! jobs to it without blocking.

use tokio::sync::mpsc;
use uuid::Uuid;

use super::service::MetadataSuggestionService;

/// One unit of work for the worker. Album/Artist auto-enqueued during scan;
/// Track only via the admin button (per the design).
///
/// `force=true` bypasses the "already 'found'" early-exit -- used when the
/// admin manually clicks "Lookup on MB" and wants a fresh run.
#[derive(Debug, Clone, Copy)]
pub enum LookupJob {
    Album { id: Uuid, force: bool },
    Artist { id: Uuid, force: bool },
    Track { id: Uuid, force: bool },
}

/// Channel handle stored on `AppState`. `try_send` is the only call site
/// during the scan; the worker side awaits the next job sequentially.
#[derive(Clone)]
pub struct LookupSender(mpsc::Sender<LookupJob>);

impl LookupSender {
    /// Best-effort enqueue. Returns false when the channel is full or closed
    /// -- caller logs and continues. This is the scanner's path; it MUST NOT
    /// block.
    pub fn enqueue(&self, job: LookupJob) -> bool {
        match self.0.try_send(job) {
            Ok(()) => true,
            Err(mpsc::error::TrySendError::Full(_)) => {
                tracing::warn!("mb worker queue full; dropping job {:?}", job);
                false
            }
            Err(mpsc::error::TrySendError::Closed(_)) => false,
        }
    }
}

/// Spawn the worker. Returns a sender; the worker continues until the sender
/// is dropped. Job buffer is intentionally bounded so a runaway producer
/// doesn't blow memory -- a full queue just drops jobs (admin can retry via
/// the bulk endpoint).
pub fn spawn_worker(service: MetadataSuggestionService) -> LookupSender {
    let (tx, mut rx) = mpsc::channel::<LookupJob>(1024);
    tokio::spawn(async move {
        while let Some(job) = rx.recv().await {
            let outcome = match job {
                LookupJob::Album { id, force: false } => {
                    service.lookup_album(id).await.map(|n| ("album", id, n))
                }
                LookupJob::Album { id, force: true } => service
                    .lookup_album_force(id)
                    .await
                    .map(|n| ("album", id, n)),
                LookupJob::Artist { id, force: false } => {
                    service.lookup_artist(id).await.map(|n| ("artist", id, n))
                }
                LookupJob::Artist { id, force: true } => service
                    .lookup_artist_force(id)
                    .await
                    .map(|n| ("artist", id, n)),
                LookupJob::Track { id, force: false } => {
                    service.lookup_track(id).await.map(|n| ("track", id, n))
                }
                LookupJob::Track { id, force: true } => service
                    .lookup_track_force(id)
                    .await
                    .map(|n| ("track", id, n)),
            };
            match outcome {
                Ok((kind, id, n)) => {
                    tracing::info!(kind, %id, candidates = n, "mb lookup done");
                }
                Err(e) => {
                    tracing::warn!("mb worker job {:?} errored: {}", job, e);
                }
            }
        }
        tracing::info!("mb worker channel closed; worker shutting down");
    });
    LookupSender(tx)
}
