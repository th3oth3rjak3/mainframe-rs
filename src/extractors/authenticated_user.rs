use axum::extract::FromRequestParts;

use crate::{sessions::Session, users::UserResponse};

#[derive(Debug, Clone)]
pub struct AuthenticatedUser {
    pub user: UserResponse,
    pub session: Session,
}

impl<S> FromRequestParts<S> for AuthenticatedUser
where
    S: Send + Sync,
{
    type Rejection = crate::errors::ApiError;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        _: &S,
    ) -> Result<Self, Self::Rejection> {
        parts
            .extensions
            .get::<Self>()
            .cloned()
            .ok_or(crate::errors::ApiError::Unauthorized {
                reason: "authentication required".into(),
            })
    }
}
