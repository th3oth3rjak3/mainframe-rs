use std::sync::Arc;

use time::OffsetDateTime;
use uuid::Uuid;

use crate::{
    errors::ServiceError,
    roles::IRoleRepository,
    users::{
        CreateUserRequest, IUserRepository, Password, UpdatePasswordRequest, UpdateUserRequest,
        User, UserBaseResponse, UserResponse,
    },
};

#[async_trait::async_trait]
pub trait IUserService: Send + Sync {
    async fn get_by_id(&self, id: Uuid) -> Result<UserResponse, ServiceError>;
    async fn get_all(&self) -> Result<Vec<UserBaseResponse>, ServiceError>;
    async fn create(&self, request: CreateUserRequest) -> Result<Uuid, ServiceError>;
    async fn update(&self, id: Uuid, request: UpdateUserRequest) -> Result<(), ServiceError>;
    async fn update_password_for_user(
        &self,
        id: Uuid,
        request: UpdatePasswordRequest,
    ) -> Result<(), ServiceError>;
    async fn delete(&self, id: Uuid) -> Result<(), ServiceError>;
}

#[derive(Clone)]
pub struct UserService {
    user_repo: Arc<dyn IUserRepository>,
    role_repo: Arc<dyn IRoleRepository>,
}

impl UserService {
    pub fn new(user_repo: Arc<dyn IUserRepository>, role_repo: Arc<dyn IRoleRepository>) -> Self {
        Self {
            user_repo,
            role_repo,
        }
    }
}

#[async_trait::async_trait]
impl IUserService for UserService {
    async fn get_by_id(&self, id: Uuid) -> Result<UserResponse, ServiceError> {
        let user_base = self.user_repo.get_by_id(id).await?;

        let roles = self.role_repo.get_by_user_id(id).await?;

        let mut user: User = user_base.into();
        user.roles = roles;

        Ok(user.into())
    }

    async fn get_all(&self) -> Result<Vec<UserBaseResponse>, ServiceError> {
        let users = self
            .user_repo
            .get_all()
            .await?
            .into_iter()
            .map(UserBaseResponse::from)
            .collect::<Vec<_>>();

        Ok(users)
    }

    async fn create(&self, request: CreateUserRequest) -> Result<Uuid, ServiceError> {
        let role_ids = request.roles.clone();
        let mut new_user: User = request.try_into()?;
        let roles = self.role_repo.get_by_role_ids(role_ids).await?;
        new_user.roles = roles;
        self.user_repo.create(&new_user).await?;
        Ok(new_user.id)
    }

    async fn update(&self, id: Uuid, request: UpdateUserRequest) -> Result<(), ServiceError> {
        let mut existing = self.user_repo.get_by_id(id).await?;
        existing.first_name = request.first_name;
        existing.last_name = request.last_name;
        existing.email = request.email;
        existing.username = request.username;
        existing.updated_at = OffsetDateTime::now_utc();

        self.user_repo.update_base(&existing).await?;
        Ok(())
    }

    async fn update_password_for_user(
        &self,
        id: Uuid,
        request: UpdatePasswordRequest,
    ) -> Result<(), ServiceError> {
        let mut existing = self.user_repo.get_by_id(id).await?;
        let new_password = Password::new(&request.raw_password)?;
        existing.password_hash = new_password;
        self.user_repo.update_password(&existing).await?;
        Ok(())
    }

    async fn delete(&self, id: Uuid) -> Result<(), ServiceError> {
        self.user_repo.delete(id).await?;
        Ok(())
    }
}
