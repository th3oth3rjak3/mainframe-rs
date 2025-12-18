use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::{
    sessions::Session,
    users::{User, UserResponse},
};

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Clone)]
pub struct LoginDetails {
    pub session: Session,
    pub user: User,
}

#[derive(Debug, Clone)]
pub struct AuthenticatedUser {
    pub user: UserResponse,
    pub session: Session,
}

impl From<LoginDetails> for AuthenticatedUser {
    fn from(value: LoginDetails) -> Self {
        Self {
            user: value.user.into(),
            session: value.session,
        }
    }
}
