use axum::{
    RequestPartsExt,
    extract::{FromRef, FromRequestParts},
};
use axum_extra::extract::CookieJar;
use uuid::Uuid;

use crate::{errors::ApiError, services::ServiceContainer, sessions::Session, users::UserResponse};

#[derive(Clone)]
pub struct AuthUser {
    pub user: UserResponse,
    pub session: Session,
}

impl<S> FromRequestParts<S> for AuthUser
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
        let cookie_jar = parts
            .extract::<CookieJar>()
            .await
            .map_err(|_| ApiError::unauthorized())?;

        let session_id = cookie_jar
            .get("session_id")
            .and_then(|cookie| Uuid::parse_str(cookie.value()).ok())
            .ok_or_else(ApiError::unauthorized)?;

        let (session, user) = container
            .session_service()
            .get_session_with_user(session_id)
            .await
            .map_err(|_| ApiError::unauthorized())?
            .ok_or_else(ApiError::unauthorized)?;

        Ok(Self { user, session })
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
        let auth_user = AuthUser::from_request_parts(parts, state).await?;

        if !auth_user.user.is_admin {
            return Err(ApiError::forbidden());
        }

        Ok(Self {
            user: auth_user.user,
            session: auth_user.session,
        })
    }
}
