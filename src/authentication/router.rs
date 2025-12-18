use crate::{
    authentication::{AuthenticatedUser, LoginRequest},
    errors::ApiError,
    services::ServiceContainer,
    users::UserResponse,
};
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

#[derive(utoipa::OpenApi)]
#[openapi(
    paths(
        crate::authentication::refresh,
        crate::authentication::login,
        crate::authentication::logout
    ),
    components(
        schemas(LoginRequest, UserResponse)
    ),
    tags(
        (
            name = "Authentication", 
            description = "User authentication and session management endpoints"
        )
    )
)]
pub struct AuthApiDoc;

pub fn router() -> Router<ServiceContainer> {
    Router::new()
        .route("/me", get(refresh))
        .route("/login", post(login))
        .route("/logout", post(logout))
}

#[utoipa::path(
    get,
    summary = "Get Current User",
    path = "/api/auth/me",
    tag = "Authentication",
    responses(
        (status = 200, description = "Current user details retrieved successfully", body = UserResponse),
        (status = 401, description = "Unauthorized - invalid or expired session"),
    ),
    description = "Retrieves the currently authenticated user's details and refreshes \
                  their session cookie. This endpoint is typically called when the \
                  client application loads or refreshes to restore user state from \
                  the session cookie."
)]
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

#[utoipa::path(
    post,
    summary = "Login",
    path = "/api/auth/login",
    tag = "Authentication",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "Login successful", body = UserResponse),
        (status = 400, description = "Invalid request body"),
        (status = 401, description = "Invalid username or password"),
    ),
    description = "Authenticates a user with their username and password. On successful \
                  authentication, creates a new session and returns a secure, HTTP-only \
                  session cookie along with the user's details. The session cookie is \
                  used for all subsequent authenticated requests."
)]
pub async fn login(
    State(container): State<ServiceContainer>,
    Json(login): Json<LoginRequest>,
) -> Result<impl IntoResponse, ApiError> {
    let AuthenticatedUser { user, session } = container.auth_service().login(login).await?;

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

#[utoipa::path(
    post,
    summary = "Logout",
    path = "/api/auth/logout",
    tag = "Authentication",
    responses(
        (status = 200, description = "Logout successful"),
        (status = 401, description = "Unauthorized - no valid session"),
    ),
    description = "Logs out the currently authenticated user by invalidating their session \
                  and clearing the session cookie. The session is removed from the server \
                  and the cookie is expired, ensuring the user must log in again to access \
                  protected resources."
)]
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
