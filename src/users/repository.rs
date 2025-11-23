use crate::{errors::RepositoryError, users::User};
use async_trait::async_trait;
use sqlx::PgPool;

#[async_trait::async_trait]
pub trait IUserRepository {
    async fn get_by_id(&self, id: i32) -> Result<User, RepositoryError>;
    async fn get_all(&self) -> Result<Vec<User>, RepositoryError>;
    async fn create(&self, user: &User) -> Result<i32, RepositoryError>;
    async fn update(&self, user: &User) -> Result<(), RepositoryError>;
    async fn delete(&self, id: i32) -> Result<(), RepositoryError>;
}

pub struct SqlxUserRepository {
    pub pool: PgPool,
}

impl SqlxUserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl IUserRepository for SqlxUserRepository {
    async fn get_by_id(&self, id: i32) -> Result<User, RepositoryError> {
        let user = sqlx::query_as!(User, "SELECT * from public.users WHERE id = $1;", id)
            .fetch_one(&self.pool)
            .await?;

        return Ok(user);
    }

    async fn get_all(&self) -> Result<Vec<User>, RepositoryError> {
        let users = sqlx::query_as!(User, "SELECT * from public.users")
            .fetch_all(&self.pool)
            .await?;

        return Ok(users);
    }

    async fn create(&self, user: &User) -> Result<i32, RepositoryError> {
        let created = sqlx::query!(
            r#"INSERT INTO public.users (first_name, last_name, email, username, password)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id
            "#,
            user.first_name,
            user.last_name,
            user.email,
            user.username,
            user.password.to_string()
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(created.id)
    }

    async fn update(&self, user: &User) -> Result<(), RepositoryError> {
        sqlx::query!(
            r#"
            UPDATE public.users
            SET first_name = $1,
            last_name = $2,
            email = $3,
            username = $4,
            password = $5
            WHERE id = $6
            "#,
            user.first_name,
            user.last_name,
            user.email,
            user.username,
            user.password.to_string(),
            user.id
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn delete(&self, id: i32) -> Result<(), RepositoryError> {
        sqlx::query!("DELETE FROM public.users WHERE id = $1", id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}
