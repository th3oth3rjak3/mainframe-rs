use std::sync::Arc;

use axum::{
    Json, Router,
    extract::{Path, State},
    routing::get,
};

use crate::{
    errors::ApiError,
    services::{ServiceContainer, UserModule},
    users::{CreateUserRequest, UpdateUserRequest, UserResponse},
};

pub fn router() -> Router<Arc<ServiceContainer>> {
    Router::new()
        .route("/", get(get_all_users).post(create_user))
        .route("/{id}", get(get_by_id).put(update_user).delete(delete_user))
}

#[axum::debug_handler]
pub async fn get_all_users(
    State(module): State<UserModule>,
) -> Result<Json<Vec<UserResponse>>, ApiError> {
    let users = module.user_service.get_all().await?;
    Ok(Json(users))
}

#[axum::debug_handler]
pub async fn get_by_id(
    Path(id): Path<i32>,
    State(module): State<UserModule>,
) -> Result<Json<UserResponse>, ApiError> {
    let user = module.user_service.get_by_id(id).await?;
    Ok(Json(user))
}

#[axum::debug_handler]
pub async fn create_user(
    State(module): State<UserModule>,
    Json(req): Json<CreateUserRequest>,
) -> Result<Json<UserResponse>, ApiError> {
    let user = module.user_service.create(req).await?;
    Ok(Json(user))
}

#[axum::debug_handler]
pub async fn update_user(
    Path(id): Path<i32>,
    State(module): State<UserModule>,
    Json(req): Json<UpdateUserRequest>,
) -> Result<Json<UserResponse>, ApiError> {
    let user = module.user_service.update(id, req).await?;
    Ok(Json(user))
}

#[axum::debug_handler]
pub async fn delete_user(
    Path(id): Path<i32>,
    State(module): State<UserModule>,
) -> Result<(), ApiError> {
    module.user_service.delete(id).await?;
    Ok(())
}
