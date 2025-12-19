use std::sync::Arc;

use crate::{
    errors::ServiceError,
    roles::{IRoleRepository, Role},
};
use uuid::Uuid;

#[async_trait::async_trait]
pub trait IRoleService: Send + Sync {
    /// Get all roles
    async fn get_all(&self) -> Result<Vec<Role>, ServiceError>;

    /// Get a role by its id.
    async fn get_by_id(&self, id: Uuid) -> Result<Role, ServiceError>;
}

#[derive(Clone)]
pub struct RoleService {
    roles: Arc<dyn IRoleRepository>,
}

impl RoleService {
    pub fn new(role_repo: Arc<dyn IRoleRepository>) -> Self {
        Self { roles: role_repo }
    }
}

#[async_trait::async_trait]
impl IRoleService for RoleService {
    async fn get_all(&self) -> Result<Vec<Role>, ServiceError> {
        let roles = self.roles.get_all().await?;
        Ok(roles)
    }

    async fn get_by_id(&self, id: Uuid) -> Result<Role, ServiceError> {
        let role = self.roles.get_by_id(id).await?;
        Ok(role)
    }
}
