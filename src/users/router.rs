use axum::{
    Json, Router,
    extract::{Path, State},
    http::HeaderValue,
    response::IntoResponse,
    routing::{get, put},
};

use hyper::{HeaderMap, StatusCode, header};
use uuid::Uuid;

use crate::{
    auth::AdminUser,
    authentication::AuthenticatedUser,
    errors::ApiError,
    services::ServiceContainer,
    users::{CreateUserRequest, UpdateUserRequest, UserBaseResponse, UserResponse},
};

pub fn router() -> Router<ServiceContainer> {
    Router::new()
        .route("/", get(get_all_users).post(create_user))
        .route("/{id}", get(get_by_id).put(update_user).delete(delete_user))
        .route("/self", put(update_self))
}

pub async fn get_all_users(
    _: AdminUser,
    State(container): State<ServiceContainer>,
) -> Result<Json<Vec<UserBaseResponse>>, ApiError> {
    let users = container.user_service().get_all().await?;
    Ok(Json(users))
}

pub async fn get_by_id(
    _: AdminUser,
    Path(id): Path<Uuid>,
    State(container): State<ServiceContainer>,
) -> Result<Json<UserResponse>, ApiError> {
    let user = container.user_service().get_by_id(id).await?;
    Ok(Json(user))
}

pub async fn create_user(
    _: AdminUser,
    State(container): State<ServiceContainer>,
    Json(req): Json<CreateUserRequest>,
) -> Result<impl IntoResponse, ApiError> {
    let user_id = container.user_service().create(req).await?;
    let location_str = format!("/users/{}", user_id);
    let location = HeaderValue::from_str(&location_str).map_err(|err| anyhow::anyhow!(err))?;
    let mut headers = HeaderMap::new();
    headers.insert(header::LOCATION, location);
    Ok((StatusCode::CREATED, headers))
}

pub async fn update_user(
    _: AdminUser,
    Path(id): Path<Uuid>,
    State(container): State<ServiceContainer>,
    Json(req): Json<UpdateUserRequest>,
) -> Result<impl IntoResponse, ApiError> {
    container.user_service().update(id, req).await?;
    Ok(StatusCode::NO_CONTENT)
}

pub async fn update_self(
    auth: AuthenticatedUser,
    State(container): State<ServiceContainer>,
    Json(req): Json<UpdateUserRequest>,
) -> Result<impl IntoResponse, ApiError> {
    container.user_service().update(auth.user.id, req).await?;
    Ok(StatusCode::NO_CONTENT)
}

pub async fn delete_user(
    _: AdminUser,
    Path(id): Path<Uuid>,
    State(container): State<ServiceContainer>,
) -> Result<impl IntoResponse, ApiError> {
    container.user_service().delete(id).await?;
    Ok(StatusCode::NO_CONTENT)
}
