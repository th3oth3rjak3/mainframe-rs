use sqlx::SqlitePool;
use uuid::Uuid;

use crate::{errors::RepositoryError, roles::Role};

#[async_trait::async_trait]
pub trait IRoleRepository: Send + Sync {
    async fn get_by_id(&self, id: Uuid) -> Result<Role, RepositoryError>;
    async fn get_all(&self) -> Result<Vec<Role>, RepositoryError>;
    async fn get_by_user_id(&self, id: Uuid) -> Result<Vec<Role>, RepositoryError>;
    async fn get_by_role_ids(&self, roles: Vec<Uuid>) -> Result<Vec<Role>, RepositoryError>;
}

pub struct SqlxRoleRepository {
    pub pool: SqlitePool,
}

impl SqlxRoleRepository {
    pub const fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl IRoleRepository for SqlxRoleRepository {
    async fn get_by_id(&self, id: Uuid) -> Result<Role, RepositoryError> {
        let role = sqlx::query_as!(
            Role,
            r#"
            SELECT
                id AS "id: uuid::Uuid",
                name
            FROM roles
            WHERE id = ?
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        role.ok_or(RepositoryError::NotFound {
            entity: "role",
            property: "id",
            value: id.to_string(),
        })
    }

    async fn get_all(&self) -> Result<Vec<Role>, RepositoryError> {
        let roles = sqlx::query_as!(
            Role,
            r#"
            SELECT
                id AS "id: uuid::Uuid",
                name
            FROM roles"#,
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(roles)
    }

    async fn get_by_user_id(&self, id: Uuid) -> Result<Vec<Role>, RepositoryError> {
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
            id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(roles)
    }

    async fn get_by_role_ids(&self, roles: Vec<Uuid>) -> Result<Vec<Role>, RepositoryError> {
        let mut builder = sqlx::QueryBuilder::new("SELECT id, name FROM roles WHERE id IN (");

        for (idx, id) in roles.iter().enumerate() {
            builder.push_bind(id);
            if idx != roles.len() - 1 {
                builder.push(", ");
            }
        }

        builder.push(")");

        let query = builder.build_query_as::<Role>();
        let roles = query.fetch_all(&self.pool).await?;
        Ok(roles)
    }
}
