use std::sync::Arc;

use async_trait::async_trait;
use time::OffsetDateTime;

use crate::{
    errors::ApiError,
    users::{
        CreateUserRequest, IUserRepository, LoginRequest, Password, UpdateUserRequest, User,
        UserResponse,
    },
};

type UserRepoImpl = Arc<dyn IUserRepository + Send + Sync>;

#[async_trait]
pub trait IUserService: Send + Sync {
    async fn get_by_id(&self, id: i32) -> Result<UserResponse, ApiError>;
    async fn get_all(&self) -> Result<Vec<UserResponse>, ApiError>;
    async fn create(&self, request: CreateUserRequest) -> Result<UserResponse, ApiError>;
    async fn update(&self, id: i32, request: UpdateUserRequest) -> Result<UserResponse, ApiError>;
    async fn delete(&self, id: i32) -> Result<(), ApiError>;
    async fn login(&self, request: LoginRequest) -> Result<UserResponse, ApiError>;
}

#[derive(Clone)]
pub struct UserService {
    user_repo: UserRepoImpl,
}

impl UserService {
    pub fn new(user_repo: UserRepoImpl) -> Self {
        Self { user_repo }
    }
}

#[async_trait]
impl IUserService for UserService {
    async fn get_by_id(&self, id: i32) -> Result<UserResponse, ApiError> {
        let user = self.user_repo.get_by_id(id).await?;
        Ok(user.into())
    }

    async fn get_all(&self) -> Result<Vec<UserResponse>, ApiError> {
        let users = self
            .user_repo
            .get_all()
            .await?
            .into_iter()
            .map(|u| u.into())
            .collect::<Vec<_>>();

        Ok(users)
    }

    async fn create(&self, request: CreateUserRequest) -> Result<UserResponse, ApiError> {
        let mut new_user: User = request.into();
        let id = self.user_repo.create(&new_user).await?;
        new_user.id = id;
        Ok(new_user.into())
    }

    async fn update(&self, id: i32, request: UpdateUserRequest) -> Result<UserResponse, ApiError> {
        let mut existing = self.user_repo.get_by_id(id).await?;
        existing.first_name = request.first_name;
        existing.last_name = request.last_name;
        existing.email = request.email;
        existing.username = request.username;
        existing.is_admin = request.is_admin;
        if let Some(pw) = request.raw_password {
            existing.password = Password::new(&pw);
        }

        self.user_repo.update(&existing).await?;
        Ok(existing.into())
    }

    async fn delete(&self, id: i32) -> Result<(), ApiError> {
        self.user_repo.delete(id).await?;
        Ok(())
    }

    async fn login(&self, request: LoginRequest) -> Result<UserResponse, ApiError> {
        let mut user = self.user_repo.get_by_username(&request.username).await?;
        let is_valid = user.password.verify(request.password.as_bytes());
        if is_valid {
            user.last_login = Some(OffsetDateTime::now_utc());
            self.user_repo.update(&user).await?;
            return Ok(user.into());
        }

        Err(ApiError::unauthorized())
    }
}
