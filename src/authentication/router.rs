use axum::{
    Json, Router,
    extract::State,
    response::IntoResponse,
    routing::{get, post},
};
use axum_extra::extract::{
    CookieJar,
    cookie::{self, Cookie},
};
use time::{Duration, OffsetDateTime};

use crate::{
    authentication::{AuthenticatedUser, LoginRequest},
    errors::ApiError,
    services::ServiceContainer,
};

pub fn router() -> Router<ServiceContainer> {
    Router::new()
        .route("/me", get(refresh))
        .route("/login", post(login))
        .route("/logout", post(logout))
}

pub async fn refresh(auth: AuthenticatedUser) -> Result<impl IntoResponse, ApiError> {
    let cookie = Cookie::build(("session_id", auth.session.id.to_string()))
        .path("/")
        .http_only(true)
        .secure(true)
        .same_site(cookie::SameSite::Strict)
        .expires(auth.session.expires_at)
        .build();

    let jar = CookieJar::new().add(cookie);

    Ok((jar, Json(auth.user)))
}

pub async fn login(
    State(container): State<ServiceContainer>,
    Json(login): Json<LoginRequest>,
) -> Result<impl IntoResponse, ApiError> {
    let AuthenticatedUser { user, session } = container.auth_service().login(login).await?;

    // create cookie with session id
    let cookie = Cookie::build(("session_id", session.id.to_string()))
        .path("/")
        .http_only(true)
        .secure(true)
        .same_site(cookie::SameSite::Strict)
        .expires(session.expires_at)
        .build();

    let jar = CookieJar::new().add(cookie);

    Ok((jar, Json(user)))
}

pub async fn logout(
    auth: AuthenticatedUser,
    State(container): State<ServiceContainer>,
) -> Result<impl IntoResponse, ApiError> {
    container.auth_service().logout(auth.session.id).await?;

    let cookie = Cookie::build(("session_id", ""))
        .path("/")
        .http_only(true)
        .secure(true)
        .same_site(cookie::SameSite::Strict)
        .expires(OffsetDateTime::now_utc().saturating_sub(Duration::hours(1)))
        .build();

    let cookie_jar = CookieJar::new().add(cookie);

    Ok((
        cookie_jar,
        Json(serde_json::json!({"message": "Logged out successfully"})),
    ))
}
