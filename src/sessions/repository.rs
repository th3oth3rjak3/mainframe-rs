use async_trait::async_trait;
use sqlx::PgPool;
use time::OffsetDateTime;
use uuid::Uuid;

use crate::{errors::RepositoryError, sessions::Session, users::UserResponse};

#[async_trait]
pub trait ISessionRepository: Send + Sync {
    /// Create a new user session in the database.
    async fn create(&self, session: &Session) -> Result<(), RepositoryError>;

    /// Delete a session by its id.
    async fn delete(&self, id: Uuid) -> Result<(), RepositoryError>;

    /// Delete all expired sessions for cleanup purposes.
    async fn delete_expired(&self) -> Result<(), RepositoryError>;

    /// Get a session and the associated user for auth.
    async fn get_session_with_user(
        &self,
        session_id: Uuid,
    ) -> Result<(Session, UserResponse), RepositoryError>;
}

pub struct SqlxSessionRepository {
    pub pool: PgPool,
}

impl SqlxSessionRepository {
    pub const fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ISessionRepository for SqlxSessionRepository {
    async fn create(&self, session: &Session) -> Result<(), RepositoryError> {
        sqlx::query!(
            r#"
            INSERT INTO public.sessions (id, user_id, created_at, expires_at)
            VALUES (?, ?, ?, ?)"#,
            session.id,
            session.user_id,
            session.created_at,
            session.expires_at
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn delete(&self, id: Uuid) -> Result<(), RepositoryError> {
        sqlx::query!("DELETE FROM public.sessions WHERE id = ?", id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    async fn delete_expired(&self) -> Result<(), RepositoryError> {
        sqlx::query!("DELETE FROM public.sessions WHERE expires_at < NOW()")
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    async fn get_session_with_user(
        &self,
        session_id: Uuid,
    ) -> Result<(Session, UserResponse), RepositoryError> {
        let row = sqlx::query!(
            r#"
            SELECT
                u.id AS user_id,
                u.first_name,
                u.last_name,
                u.email,
                u.username,
                u.last_login,
                u.is_admin,
                s.id AS session_id,
                s.created_at,
                s.expires_at
            FROM public.users u
            INNER JOIN public.sessions s on u.id = s.user_id
            WHERE s.id = ?
            "#,
            session_id
        )
        .fetch_optional(&self.pool)
        .await?;

        if let Some(row) = row {
            if row.expires_at < OffsetDateTime::now_utc() {
                self.delete(session_id).await?;
                Err(RepositoryError::Unauthorized)
            } else {
                let user = UserResponse {
                    id: row.user_id,
                    first_name: row.first_name,
                    last_name: row.last_name,
                    email: row.email,
                    username: row.username,
                    last_login: row.last_login,
                    is_admin: row.is_admin,
                };

                let session = Session {
                    id: row.session_id,
                    user_id: row.user_id,
                    created_at: row.created_at,
                    expires_at: row.expires_at,
                };

                Ok((session, user))
            }
        } else {
            Err(RepositoryError::Unauthorized)
        }
    }
}
