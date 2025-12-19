use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::{extractors::authenticated_user::AuthenticatedUser, sessions::Session, users::User};

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

impl From<LoginDetails> for AuthenticatedUser {
    fn from(value: LoginDetails) -> Self {
        Self {
            user: value.user.into(),
            session: value.session,
        }
    }
}

/// Marker indicating the handler explicitly managed the session cookie
#[derive(Debug, Clone, Copy)]
pub struct SessionCookieHandled;
