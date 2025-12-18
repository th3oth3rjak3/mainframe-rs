use async_trait::async_trait;
use sqlx::SqlitePool;
use uuid::Uuid;

use crate::{
    errors::RepositoryError,
    sessions::{Session, SessionSummary},
    users::UserBaseResponse,
};

#[async_trait]
pub trait ISessionRepository: Send + Sync {
    /// Update the expiration of a session.
    async fn update(&self, session: &Session) -> Result<(), RepositoryError>;

    /// Delete a session by its id.
    async fn delete(&self, id: Uuid) -> Result<(), RepositoryError>;

    /// Delete all expired sessions for cleanup purposes.
    async fn delete_expired(&self) -> Result<u64, RepositoryError>;

    /// Delete all sessions for a given user.
    async fn delete_all_for_user(&self, user_id: Uuid) -> Result<(), RepositoryError>;

    /// Get a session and the associated user for auth.
    async fn get_by_id(&self, session_id: Uuid) -> Result<Session, RepositoryError>;

    /// Find all active sessions and get the details for each user and a count of active
    /// sessions.
    async fn get_active_summary(&self) -> Result<Vec<SessionSummary>, RepositoryError>;
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

    async fn delete_expired(&self) -> Result<u64, RepositoryError> {
        let result = sqlx::query!("DELETE FROM sessions WHERE expires_at < CURRENT_TIMESTAMP")
            .execute(&self.pool)
            .await?;

        Ok(result.rows_affected())
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
                token,
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

    async fn get_active_summary(&self) -> Result<Vec<SessionSummary>, RepositoryError> {
        let rows = sqlx::query!(
            r#"
            SELECT DISTINCT
                COUNT(1) as count,
                u.id as "id: uuid::Uuid",
                u.first_name,
                u.last_name,
                u.email,
                u.username,
                u.last_login,
                u.is_disabled
            FROM users u
            INNER JOIN sessions s
                ON s.user_id = u.id
            WHERE s.expires_at > CURRENT_TIMESTAMP
            GROUP BY u.id
        "#
        )
        .fetch_all(&self.pool)
        .await?;

        let results: Vec<SessionSummary> = rows
            .into_iter()
            .map(|row| {
                let count = row.count;
                let user_base = UserBaseResponse {
                    id: row.id,
                    first_name: row.first_name,
                    last_name: row.last_name,
                    email: row.email,
                    username: row.username,
                    last_login: row.last_login,
                    is_disabled: row.is_disabled,
                };

                SessionSummary {
                    user: user_base,
                    active_sessions: count,
                }
            })
            .collect();

        Ok(results)
    }
}
