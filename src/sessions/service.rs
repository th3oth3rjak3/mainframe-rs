use std::sync::Arc;

use async_trait::async_trait;
use uuid::Uuid;

use crate::{
    errors::ApiError,
    sessions::{ISessionRepository, Session},
    users::UserResponse,
};

#[async_trait]
pub trait ISessionService: Send + Sync {
    /// Create a new session for a given user.
    async fn create_session(&self, user_id: i32) -> Result<Session, ApiError>;

    /// Delete a session by id (e.g., logout).
    async fn delete_session(&self, session_id: Uuid) -> Result<(), ApiError>;

    /// Delete all expired sessions (lazy cleanup or scheduled job).
    async fn cleanup_expired_sessions(&self) -> Result<(), ApiError>;

    /// Get a session and the user details for auth.
    async fn get_session_with_user(
        &self,
        session_id: Uuid,
    ) -> Result<(Session, UserResponse), ApiError>;

    /// Refresh the session with a new cookie if the current session isn't yet expired.
    async fn refresh_session(&self, session_id: Uuid) -> Result<(Session, UserResponse), ApiError>;
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
    async fn create_session(&self, user_id: i32) -> Result<Session, ApiError> {
        let session = Session::new(user_id);
        self.session_repo.create(&session).await?;
        Ok(session)
    }

    async fn delete_session(&self, session_id: Uuid) -> Result<(), ApiError> {
        self.session_repo.delete(session_id).await?;
        Ok(())
    }

    async fn cleanup_expired_sessions(&self) -> Result<(), ApiError> {
        self.session_repo.delete_expired().await?;
        Ok(())
    }

    async fn get_session_with_user(
        &self,
        session_id: Uuid,
    ) -> Result<(Session, UserResponse), ApiError> {
        let result = self.session_repo.get_session_with_user(session_id).await?;
        Ok(result)
    }

    async fn refresh_session(&self, session_id: Uuid) -> Result<(Session, UserResponse), ApiError> {
        self.session_repo.delete_expired().await?;
        let (session, user) = self.session_repo.get_session_with_user(session_id).await?;

        let new_session = Session::new(user.id);
        self.session_repo.create(&new_session).await?;
        self.session_repo.delete(session.id).await?;

        Ok((new_session, user))
    }
}
