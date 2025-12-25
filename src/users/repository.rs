use crate::{
    errors::RepositoryError,
    users::{User, UserBase},
};
use async_trait::async_trait;
use sqlx::SqlitePool;
use uuid::Uuid;

#[async_trait::async_trait]
pub trait IUserRepository: Send + Sync {
    async fn get_by_id(&self, id: Uuid) -> Result<UserBase, RepositoryError>;
    async fn get_by_username(&self, username: &str) -> Result<UserBase, RepositoryError>;
    async fn get_all(&self) -> Result<Vec<UserBase>, RepositoryError>;
    async fn create(&self, user: &User) -> Result<(), RepositoryError>;
    async fn update_base(&self, user: &UserBase) -> Result<(), RepositoryError>;
    async fn update_password(&self, user: &UserBase) -> Result<(), RepositoryError>;
    async fn delete(&self, id: Uuid) -> Result<(), RepositoryError>;
}

pub struct SqlxUserRepository {
    pub pool: SqlitePool,
}

impl SqlxUserRepository {
    pub const fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl IUserRepository for SqlxUserRepository {
    async fn get_by_id(&self, id: uuid::Uuid) -> Result<UserBase, RepositoryError> {
        let user_base = sqlx::query_as!(
            UserBase,
            r#"
            SELECT 
                id as "id: uuid::Uuid", 
                email, 
                first_name, 
                last_name, 
                username, 
                password_hash,
                password_expiration,
                last_login,
                failed_login_attempts,
                last_failed_login_attempt,
                is_disabled,
                created_at,
                updated_at
            FROM users 
            WHERE id = ?;"#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        user_base.ok_or(RepositoryError::NotFound {
            entity: "user",
            property: "id",
            value: id.to_string(),
        })
    }

    async fn get_by_username(&self, username: &str) -> Result<UserBase, RepositoryError> {
        let user_base = sqlx::query_as!(
            UserBase,
            r#"
            SELECT 
                id as "id: uuid::Uuid", 
                email, 
                first_name, 
                last_name, 
                username, 
                password_hash,
                password_expiration,
                last_login,
                is_disabled,
                failed_login_attempts,
                last_failed_login_attempt,
                created_at,
                updated_at
            FROM users 
            WHERE LOWER(username) = LOWER(?)"#,
            username
        )
        .fetch_optional(&self.pool)
        .await?;

        user_base.ok_or(RepositoryError::NotFound {
            entity: "user",
            property: "username",
            value: username.to_owned(),
        })
    }

    async fn get_all(&self) -> Result<Vec<UserBase>, RepositoryError> {
        let users = sqlx::query_as!(
            UserBase,
            r#"
            SELECT 
                id as "id: uuid::Uuid", 
                email, 
                first_name, 
                last_name, 
                username, 
                password_hash,
                password_expiration,
                last_login,
                is_disabled,
                failed_login_attempts,
                last_failed_login_attempt,
                created_at,
                updated_at
            FROM users"#
        )
        .fetch_all(&self.pool)
        .await?;

        return Ok(users);
    }

    async fn create(&self, user: &User) -> Result<(), RepositoryError> {
        sqlx::query!(
            r#"INSERT INTO users (id, email, first_name, last_name, username, password_hash, password_expiration)
            VALUES (?, ?, ?, ?, ?, ?, ?)
            "#,
            user.id,
            user.email,
            user.first_name,
            user.last_name,
            user.username,
            user.password_hash,
            user.password_expiration,
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn update_base(&self, user: &UserBase) -> Result<(), RepositoryError> {
        let result = sqlx::query!(
            r#"
            UPDATE users
            SET first_name = ?,
            last_name = ?,
            email = ?,
            username = ?,
            is_disabled = ?
            WHERE id = ?
            "#,
            user.first_name,
            user.last_name,
            user.email,
            user.username,
            user.is_disabled,
            user.id
        )
        .execute(&self.pool)
        .await?;

        if result.rows_affected() == 0 {
            return Err(RepositoryError::NotFound {
                entity: "user",
                property: "id",
                value: user.id.to_string(),
            });
        }

        Ok(())
    }

    async fn update_password(&self, user: &UserBase) -> Result<(), RepositoryError> {
        sqlx::query!(
            "UPDATE users SET password_hash = ?, password_expiration = ? WHERE id = ?",
            user.password_hash,
            user.password_expiration,
            user.id
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn delete(&self, id: Uuid) -> Result<(), RepositoryError> {
        let result = sqlx::query!("DELETE FROM users WHERE id = ?", id)
            .execute(&self.pool)
            .await?;

        if result.rows_affected() == 0 {
            return Err(RepositoryError::NotFound {
                entity: "user",
                property: "id",
                value: id.to_string(),
            });
        }

        Ok(())
    }
}
