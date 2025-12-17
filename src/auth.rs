use axum::{
    RequestPartsExt,
    extract::{FromRef, FromRequestParts},
};
use axum_extra::extract::CookieJar;
use uuid::Uuid;

use crate::{
    authentication::AuthenticatedUser, errors::ApiError, services::ServiceContainer,
    sessions::Session, users::UserResponse,
};

impl<S> FromRequestParts<S> for AuthenticatedUser
where
    S: Send + Sync,
    ServiceContainer: axum::extract::FromRef<S>,
{
    type Rejection = ApiError;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        let container = ServiceContainer::from_ref(state);
        let cookie_jar =
            parts
                .extract::<CookieJar>()
                .await
                .map_err(|_| ApiError::Unauthorized {
                    reason: "could not extract cookie from the cookie jar".into(),
                })?;

        let session_id = cookie_jar
            .get("session_id")
            .and_then(|cookie| Uuid::parse_str(cookie.value()).ok())
            .ok_or(ApiError::Unauthorized {
                reason: "could not parse session id from cookie".into(),
            })?;

        let auth_user = container.auth_service().refresh(session_id).await?;

        Ok(auth_user)
    }
}

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
