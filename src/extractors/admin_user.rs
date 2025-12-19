use axum::extract::FromRequestParts;

use crate::{
    errors::ApiError, extractors::authenticated_user::AuthenticatedUser,
    services::ServiceContainer, sessions::Session, users::UserResponse,
};

#[derive(Clone)]
// Primarily used as an extractor to ensure a user is an admin.
#[allow(unused)]
pub struct AdminUser {
    pub user: UserResponse,
    pub session: Session,
}

impl<S> FromRequestParts<S> for AdminUser
where
    S: Send + Sync,
    ServiceContainer: axum::extract::FromRef<S>,
{
    type Rejection = ApiError;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        let AuthenticatedUser { user, session } =
            AuthenticatedUser::from_request_parts(parts, state).await?;

        if !user.is_admin() {
            return Err(ApiError::Forbidden {
                reason: "user is not an administrator".into(),
            });
        }

        Ok(Self { user, session })
    }
}
