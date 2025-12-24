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
    errors::ApiError,
    extractors::{AdminUser, ValidatedJson, authenticated_user::AuthenticatedUser},
    services::ServiceContainer,
    users::{
        CreateUserRequest, UpdatePasswordRequest, UpdateUserRequest, UserBaseResponse, UserResponse,
    },
};

pub fn router() -> Router<ServiceContainer> {
    Router::new()
        .route("/", get(get_all_users).post(create_user))
        .route("/{id}", get(get_by_id).put(update_user).delete(delete_user))
        .route("/{id}/password", put(update_password_for_user))
        .route("/self", put(update_self))
        .route("/self/password", put(update_own_password))
}

// Clippy lint triggered by utoipa macro expansion, not our code
#[allow(clippy::needless_for_each)]
#[derive(utoipa::OpenApi)]
#[openapi(
    paths(
        crate::users::get_all_users,
        crate::users::get_by_id,
        crate::users::create_user,
        crate::users::update_user,
        crate::users::update_own_password,
        crate::users::update_password_for_user,
        crate::users::update_self,
        crate::users::delete_user
    ),
    components(
        schemas(UserBaseResponse, UserResponse, CreateUserRequest, UpdateUserRequest)
    ),
    tags(
        (name = "Users", description = "User and account management endpoints")
    )
)]
pub struct UsersApiDoc;

#[utoipa::path(
    get,
    summary = "List Users",
    path = "/api/users",
    tag = "Users",
    responses(
        (status = 200, description = "List all users", body = Vec<UserBaseResponse>),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden - requires admin role"),
    ),
    description = "Retrieves a complete list of all users in the system. \
        Requires a valid session_id cookie and administrator role to access this endpoint."
)]
pub async fn get_all_users(
    _: AdminUser,
    State(container): State<ServiceContainer>,
) -> Result<Json<Vec<UserBaseResponse>>, ApiError> {
    let users = container.user_service().get_all().await?;
    Ok(Json(users))
}

#[utoipa::path(
    get,
    summary = "Get User by ID",
    path = "/api/users/{id}",
    tag = "Users",
    params(
        ("id" = Uuid, Path, description = "Unique identifier of the user")
    ),
    responses(
        (status = 200, description = "User found", body = UserResponse),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden - requires admin role"),
        (status = 404, description = "User not found"),
    ),
    description = "Retrieves detailed information about a specific user by their unique ID. \
        Requires a valid session_id cookie and administrator role to access this endpoint."
)]
pub async fn get_by_id(
    _: AdminUser,
    Path(id): Path<Uuid>,
    State(container): State<ServiceContainer>,
) -> Result<Json<UserResponse>, ApiError> {
    let user = container.user_service().get_by_id(id).await?;
    Ok(Json(user))
}

#[utoipa::path(
    post,
    summary = "Create User",
    path = "/api/users",
    tag = "Users",
    request_body = CreateUserRequest,
    responses(
        (status = 201, description = "User created successfully", headers(
            ("Location" = String, description = "URI of the newly created user")
        )),
        (status = 400, description = "Invalid request body"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden - requires admin role"),
        (status = 409, description = "User already exists"),
    ),
    description = "Creates a new user in the system with the provided details. \
        Returns a 201 status code with a Location header pointing to the newly created user resource. \
        Requires a valid session_id cookie and administrator role to access this endpoint."
)]

pub async fn create_user(
    _: AdminUser,
    State(container): State<ServiceContainer>,
    ValidatedJson(req): ValidatedJson<CreateUserRequest>,
) -> Result<impl IntoResponse, ApiError> {
    let user_id = container.user_service().create(req).await?;
    let location_str = format!("/users/{user_id}");
    let location = HeaderValue::from_str(&location_str).map_err(|err| anyhow::anyhow!(err))?;
    let mut headers = HeaderMap::new();
    headers.insert(header::LOCATION, location);
    Ok((StatusCode::CREATED, headers))
}

#[utoipa::path(
    put,
    summary = "Update User",
    path = "/api/users/{id}",
    tag = "Users",
    params(
        ("id" = Uuid, Path, description = "Unique identifier of the user to update")
    ),
    request_body = UpdateUserRequest,
    responses(
        (status = 204, description = "User updated successfully"),
        (status = 400, description = "Invalid request body"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden - requires admin role"),
        (status = 404, description = "User not found"),
    ),
    description = "Updates an existing user's information by their unique ID. \
        Only the fields provided in the request body will be updated. \
        Returns a 204 No Content status on success. \
        Requires a valid session_id cookie and administrator role to access this endpoint."
)]
pub async fn update_user(
    _: AdminUser,
    Path(id): Path<Uuid>,
    State(container): State<ServiceContainer>,
    ValidatedJson(req): ValidatedJson<UpdateUserRequest>,
) -> Result<impl IntoResponse, ApiError> {
    container.user_service().update(id, req).await?;
    Ok(StatusCode::NO_CONTENT)
}

#[utoipa::path(
    put,
    summary = "Update Password For User",
    path = "/api/users/{id}/password",
    tag = "Users",
    params(
        ("id" = Uuid, Path, description = "Unique identifier of the user")
    ),
    request_body = UpdatePasswordRequest,
    responses(
        (status = 204, description = "Password updated successfully for other user"),
        (status = 400, description = "Invalid request body"),
        (status = 401, description = "Unauthorized"),
    ),
    description = "Allows an administrator to update a password for another user. \
        Only the password field provided in the request body will be updated. \
        Returns a 204 No Content status on success. Requires a valid session_id cookie."
)]
pub async fn update_password_for_user(
    _: AdminUser,
    Path(id): Path<Uuid>,
    State(container): State<ServiceContainer>,
    ValidatedJson(req): ValidatedJson<UpdatePasswordRequest>,
) -> Result<impl IntoResponse, ApiError> {
    container
        .user_service()
        .update_password_for_user(id, req)
        .await?;
    Ok(StatusCode::NO_CONTENT)
}

#[utoipa::path(
    put,
    summary = "Update Current User",
    path = "/api/users/self",
    tag = "Users",
    request_body = UpdateUserRequest,
    responses(
        (status = 204, description = "User updated successfully"),
        (status = 400, description = "Invalid request body"),
        (status = 401, description = "Unauthorized"),
    ),
    description = "Allows an authenticated user to update their own profile information. \
        The user ID is automatically determined from the authentication session. \
        Only the fields provided in the request body will be updated. \
        Returns a 204 No Content status on success. Requires a valid session_id cookie."
)]
pub async fn update_self(
    auth: AuthenticatedUser,
    State(container): State<ServiceContainer>,
    ValidatedJson(req): ValidatedJson<UpdateUserRequest>,
) -> Result<impl IntoResponse, ApiError> {
    container.user_service().update(auth.user.id, req).await?;
    Ok(StatusCode::NO_CONTENT)
}

#[utoipa::path(
    put,
    summary = "Update Current User's Password",
    path = "/api/users/self/password",
    tag = "Users",
    request_body = UpdatePasswordRequest,
    responses(
        (status = 204, description = "Current User's Password updated successfully"),
        (status = 400, description = "Invalid request body"),
        (status = 401, description = "Unauthorized"),
    ),
    description = "Allows an authenticated user to update their own password. \
        The user ID is automatically determined from the authentication session. \
        Only the password field provided in the request body will be updated. \
        Returns a 204 No Content status on success. Requires a valid session_id cookie."
)]
pub async fn update_own_password(
    auth: AuthenticatedUser,
    State(container): State<ServiceContainer>,
    ValidatedJson(req): ValidatedJson<UpdatePasswordRequest>,
) -> Result<impl IntoResponse, ApiError> {
    container
        .user_service()
        .update_password_for_user(auth.user.id, req)
        .await?;
    Ok(StatusCode::NO_CONTENT)
}

#[utoipa::path(
    delete,
    summary = "Delete User",
    path = "/api/users/{id}",
    tag = "Users",
    params(
        ("id" = Uuid, Path, description = "Unique identifier of the user to delete")
    ),
    responses(
        (status = 204, description = "User deleted successfully"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden - requires admin role"),
        (status = 404, description = "User not found"),
    ),
    description = "Permanently deletes a user from the system by their unique ID. \
        This action cannot be undone. \
        Returns a 204 No Content status on success. \
        Requires a valid session_id cookie and administrator role to access this endpoint."
)]
pub async fn delete_user(
    _: AdminUser,
    Path(id): Path<Uuid>,
    State(container): State<ServiceContainer>,
) -> Result<impl IntoResponse, ApiError> {
    container.user_service().delete(id).await?;
    Ok(StatusCode::NO_CONTENT)
}
