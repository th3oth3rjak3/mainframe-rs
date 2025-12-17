use sqlx::SqlitePool;

use crate::{authentication::LoginDetails, errors::RepositoryError, roles::Role, sessions::Session, users::{User, UserBase}};

#[async_trait::async_trait]
pub trait IAuthenticationRepository: Send + Sync {
    async fn login(&self, user_base: UserBase, session: Session) -> Result<LoginDetails, RepositoryError>;
}

pub struct SqlxAuthenticationRepository {
    pub pool: SqlitePool,
}

impl SqlxAuthenticationRepository {
    pub const fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl IAuthenticationRepository for SqlxAuthenticationRepository {
    async fn login(&self, user_base: UserBase, session: Session) -> Result<LoginDetails, RepositoryError> {
        let mut tx = self.pool.begin().await?;
        
        sqlx::query!(r#"
            UPDATE users 
            SET last_login = ?, 
            failed_login_attempts = ?, 
            last_failed_login_attempt = ?, 
            updated_at = ? 
            WHERE id = ?"#,
            user_base.last_login,
            user_base.failed_login_attempts,
            user_base.last_failed_login_attempt,
            user_base.updated_at,
            user_base.id
        )
        .execute(&mut *tx)
        .await?;

        sqlx::query!(
            r#"INSERT INTO sessions (id, user_id, expires_at)
            VALUES (?, ?, ?)"#,
            session.id,
            session.user_id,
            session.expires_at
        )
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;

        let mut user: User = user_base.into();

        let roles = sqlx::query_as!(
            Role,
            r#"
            SELECT
                r.id AS "id: uuid::Uuid",
                r.name
            FROM roles r
            INNER JOIN user_roles ur
                ON r.id = ur.role_id
            WHERE ur.user_id = ?
            "#,
            user.id
        )
        .fetch_all(&self.pool)
        .await?;
    
        user.roles = roles;
        
        Ok(LoginDetails { session, user })
    }
}
