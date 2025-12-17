use async_trait::async_trait;
use sqlx::SqlitePool;
use uuid::Uuid;

use crate::{errors::RepositoryError, sessions::Session};

#[async_trait]
pub trait ISessionRepository: Send + Sync {
    /// Create a new user session in the database.
    async fn create(&self, session: &Session) -> Result<(), RepositoryError>;

    /// Update the expiration of a session.
    async fn update(&self, session: &Session) -> Result<(), RepositoryError>;

    /// Delete a session by its id.
    async fn delete(&self, id: Uuid) -> Result<(), RepositoryError>;

    /// Delete all expired sessions for cleanup purposes.
    async fn delete_expired(&self) -> Result<(), RepositoryError>;

    /// Delete all sessions for a given user.
    async fn delete_all_for_user(&self, user_id: Uuid) -> Result<(), RepositoryError>;

    /// Get a session and the associated user for auth.
    async fn get_by_id(&self, session_id: Uuid) -> Result<Session, RepositoryError>;
}

pub struct SqlxSessionRepository {
    pub pool: SqlitePool,
}

impl SqlxSessionRepository {
    pub const fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ISessionRepository for SqlxSessionRepository {
    async fn create(&self, session: &Session) -> Result<(), RepositoryError> {
        sqlx::query!(
            r#"INSERT INTO sessions (id, user_id, expires_at)
            VALUES (?, ?, ?)"#,
            session.id,
            session.user_id,
            session.expires_at
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn update(&self, session: &Session) -> Result<(), RepositoryError> {
        sqlx::query!(
            r#"UPDATE sessions
            SET expires_at = ?
            WHERE id = ?
        "#,
            session.expires_at,
            session.id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn delete(&self, id: Uuid) -> Result<(), RepositoryError> {
        sqlx::query!("DELETE FROM sessions WHERE id = ?", id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    async fn delete_expired(&self) -> Result<(), RepositoryError> {
        sqlx::query!("DELETE FROM sessions WHERE expires_at < CURRENT_TIMESTAMP")
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    async fn delete_all_for_user(&self, user_id: Uuid) -> Result<(), RepositoryError> {
        sqlx::query!("DELETE FROM sessions WHERE user_id = ?", user_id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    async fn get_by_id(&self, session_id: Uuid) -> Result<Session, RepositoryError> {
        let session = sqlx::query_as!(
            Session,
            r#"
            SELECT
                id AS "id: uuid::Uuid",
                user_id AS "user_id: uuid::Uuid",
                expires_at
            FROM sessions
            WHERE id = ?
            "#,
            session_id
        )
        .fetch_optional(&self.pool)
        .await?
        .ok_or(RepositoryError::NotFound {
            entity: "session",
            property: "id",
            value: session_id.to_string(),
        })?;

        Ok(session)
    }
}
