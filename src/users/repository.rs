use crate::{errors::RepositoryError, users::User};
use async_trait::async_trait;
use sqlx::SqlitePool;
use uuid::Uuid;

#[async_trait::async_trait]
pub trait IUserRepository {
    async fn get_by_id(&self, id: Uuid) -> Result<User, RepositoryError>;
    async fn get_by_username(&self, username: &str) -> Result<User, RepositoryError>;
    async fn get_all(&self) -> Result<Vec<User>, RepositoryError>;
    async fn create(&self, user: &User) -> Result<i32, RepositoryError>;
    async fn update(&self, user: &User) -> Result<(), RepositoryError>;
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
    async fn get_by_id(&self, id: uuid::Uuid) -> Result<User, RepositoryError> {
        let user = sqlx::query_as!(User, r#"
            SELECT 
                id as "id: uuid::Uuid", 
                email, 
                first_name, 
                last_name, 
                username, 
                password_hash,
                last_login,
                failed_login_attempts,
                last_failed_login_attempt,
                created_at,
                updated_at
            FROM users 
            WHERE id = ?;"#, 
            id)
            .fetch_one(&self.pool)
            .await?;

        return Ok(user);
    }

    async fn get_by_username(&self, username: &str) -> Result<User, RepositoryError> {
        let maybe_user = sqlx::query_as!(
            User,
            "SELECT * FROM users WHERE LOWER(username) = LOWER(?)",
            username
        )
        .fetch_optional(&self.pool)
        .await?;

        maybe_user.ok_or_else(|| RepositoryError::Unauthorized)
    }

    async fn get_all(&self) -> Result<Vec<User>, RepositoryError> {
        let users = sqlx::query_as!(User, "SELECT * from public.users")
            .fetch_all(&self.pool)
            .await?;

        return Ok(users);
    }

    async fn create(&self, user: &User) -> Result<i32, RepositoryError> {
        let created = sqlx::query!(
            r#"INSERT INTO public.users (first_name, last_name, email, username, password, is_admin)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id
            "#,
            user.first_name,
            user.last_name,
            user.email,
            user.username,
            user.password.to_string(),
            user.is_admin
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(created.id)
    }

    async fn update(&self, user: &User) -> Result<(), RepositoryError> {
        sqlx::query!(
            r#"
            UPDATE users
            SET first_name = ?,
            last_name = ?,
            email = ?,
            username = ?,
            password = ?,
            is_admin = ?,
            last_login = ?
            WHERE id = ?
            "#,
            user.first_name,
            user.last_name,
            user.email,
            user.username,
            user.password.to_string(),
            user.is_admin,
            user.last_login,
            user.id
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn delete(&self, id: Uuid) -> Result<(), RepositoryError> {
        sqlx::query!("DELETE FROM users WHERE id = ?", id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}
