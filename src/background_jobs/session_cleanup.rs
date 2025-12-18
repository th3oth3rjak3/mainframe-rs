use crate::sessions::ISessionRepository;
use std::sync::Arc;
use tokio::time::{Duration, interval};

/// Spawns a background task that cleans up expired sessions every 5 minutes.
/// The task will be cancelled when the returned `JoinHandle` is dropped or the tokio runtime shuts down.
pub fn spawn_cleanup_task(
    session_repo: Arc<dyn ISessionRepository>,
) -> tokio::task::JoinHandle<()> {
    tokio::spawn(async move {
        let mut ticker = interval(Duration::from_mins(5));

        loop {
            ticker.tick().await;

            tracing::debug!("Running session cleanup task");

            match session_repo.delete_expired().await {
                Ok(count) => {
                    if count > 0 {
                        tracing::info!("Cleaned up {} expired session(s)", count);
                    } else {
                        tracing::debug!("No expired sessions to clean up");
                    }
                }
                Err(err) => {
                    tracing::error!("Failed to clean up expired sessions: {:?}", err);
                }
            }
        }
    })
}
