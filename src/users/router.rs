use axum::{
    Json, Router,
    extract::{Path, State},
    response::IntoResponse,
    routing::{get, post, put},
};
use axum_extra::extract::{
    CookieJar,
    cookie::{self, Cookie},
};
use time::Duration;

use crate::{
    auth::{AdminUser, AuthUser},
    errors::ApiError,
    services::ServiceContainer,
    users::{CreateUserRequest, LoginRequest, UpdateUserRequest, UserResponse},
};

pub fn router() -> Router<ServiceContainer> {
    Router::new()
        .route("/", get(get_all_users).post(create_user))
        .route("/{id}", get(get_by_id).put(update_user).delete(delete_user))
        .route("/self", put(update_self))
        .route("/login", post(login))
        .route("/logout", post(logout))
}

#[axum::debug_handler]
pub async fn get_all_users(
    _: AdminUser,
    State(container): State<ServiceContainer>,
) -> Result<Json<Vec<UserResponse>>, ApiError> {
    let users = container.user_service().get_all().await?;
    Ok(Json(users))
}

#[axum::debug_handler]
pub async fn get_by_id(
    _: AdminUser,
    Path(id): Path<i32>,
    State(container): State<ServiceContainer>,
) -> Result<Json<UserResponse>, ApiError> {
    let user = container.user_service().get_by_id(id).await?;
    Ok(Json(user))
}

#[axum::debug_handler]
pub async fn create_user(
    _: AdminUser,
    State(container): State<ServiceContainer>,
    Json(req): Json<CreateUserRequest>,
) -> Result<Json<UserResponse>, ApiError> {
    let user = container.user_service().create(req).await?;
    Ok(Json(user))
}

#[axum::debug_handler]
pub async fn update_user(
    _: AdminUser,
    Path(id): Path<i32>,
    State(container): State<ServiceContainer>,
    Json(req): Json<UpdateUserRequest>,
) -> Result<Json<UserResponse>, ApiError> {
    let user = container.user_service().update(id, req).await?;
    Ok(Json(user))
}

#[axum::debug_handler]
pub async fn update_self(
    auth: AuthUser,
    State(container): State<ServiceContainer>,
    Json(req): Json<UpdateUserRequest>,
) -> Result<Json<UserResponse>, ApiError> {
    let user = container.user_service().update(auth.user.id, req).await?;
    Ok(Json(user))
}

#[axum::debug_handler]
pub async fn delete_user(
    _: AdminUser,
    Path(id): Path<i32>,
    State(container): State<ServiceContainer>,
) -> Result<(), ApiError> {
    container.user_service().delete(id).await?;
    Ok(())
}

#[axum::debug_handler]
pub async fn login(
    State(container): State<ServiceContainer>,
    Json(login): Json<LoginRequest>,
) -> Result<impl IntoResponse, ApiError> {
    // lazily delete expired sessions when someone tries to login
    container
        .session_service()
        .cleanup_expired_sessions()
        .await?;

    // verify login credentials and update last login timestamp
    let user = container.user_service().login(login).await?;

    // create login session
    let session = container.session_service().create_session(user.id).await?;

    // create cookie with session id
    let cookie = Cookie::build(("session_id", session.id.to_string()))
        .path("/")
        .http_only(true)
        .secure(true)
        .same_site(cookie::SameSite::Strict)
        .max_age(Duration::days(1))
        .build();

    let jar = CookieJar::new().add(cookie);

    Ok((jar, Json(user)))
}

#[axum::debug_handler]
pub async fn logout(
    auth: AuthUser,
    State(container): State<ServiceContainer>,
) -> Result<impl IntoResponse, ApiError> {
    container
        .session_service()
        .delete_session(auth.session.id)
        .await?;

    let cookie = Cookie::build(("session_id", ""))
        .path("/")
        .http_only(true)
        .secure(true)
        .same_site(cookie::SameSite::Strict)
        .max_age(Duration::seconds(-1))
        .build();

    let cookie_jar = CookieJar::new().add(cookie);

    Ok((
        cookie_jar,
        Json(serde_json::json!({"message": "Logged out successfully"})),
    ))
}
