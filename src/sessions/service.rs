use std::sync::Arc;

use async_trait::async_trait;
use uuid::Uuid;

use crate::{
    errors::ServiceError,
    sessions::{ISessionRepository, Session},
};

#[async_trait]
pub trait ISessionService: Send + Sync {
    /// Create a new session for a given user.
    async fn create_session(&self, user_id: Uuid) -> Result<Session, ServiceError>;

    /// Delete a session by id (e.g., logout).
    async fn delete_session(&self, session_id: Uuid) -> Result<(), ServiceError>;

    /// Delete all expired sessions (lazy cleanup or scheduled job).
    async fn cleanup_expired_sessions(&self) -> Result<(), ServiceError>;
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
    async fn create_session(&self, user_id: Uuid) -> Result<Session, ServiceError> {
        let session = Session::new(user_id);
        self.session_repo.create(&session).await?;
        Ok(session)
    }

    async fn delete_session(&self, session_id: Uuid) -> Result<(), ServiceError> {
        self.session_repo.delete(session_id).await?;
        Ok(())
    }

    async fn cleanup_expired_sessions(&self) -> Result<(), ServiceError> {
        self.session_repo.delete_expired().await?;
        Ok(())
    }
}
