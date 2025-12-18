use std::sync::Arc;

use async_trait::async_trait;

use crate::{
    errors::ServiceError,
    sessions::{ISessionRepository, SessionSummary},
};

#[async_trait]
pub trait ISessionService: Send + Sync {
    /// Get a list of session details for all active sessions.
    async fn get_session_summaries(&self) -> Result<Vec<SessionSummary>, ServiceError>;
}

pub struct SessionService {
    session_repo: Arc<dyn ISessionRepository>,
}

impl SessionService {
    pub fn new(session_repo: Arc<dyn ISessionRepository>) -> Self {
        Self { session_repo }
    }
}

#[async_trait]
impl ISessionService for SessionService {
    async fn get_session_summaries(&self) -> Result<Vec<SessionSummary>, ServiceError> {
        let details = self.session_repo.get_active_summary().await?;
        Ok(details)
    }
}
