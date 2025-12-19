use axum::{
    Json, Router,
    extract::{Path, State},
    routing::get,
};
use uuid::Uuid;

use crate::{auth::AdminUser, errors::ApiError, roles::Role, services::ServiceContainer};

// Clippy lint triggered by utoipa macro expansion, not our code
#[allow(clippy::needless_for_each)]
#[derive(utoipa::OpenApi)]
#[openapi(
    paths(
        crate::roles::get_all,
        crate::roles::get_by_id,
    ),
    components(
        schemas(Role)
    ),
    tags(
        (
            name = "Sessions", 
            description = "Session management"
        )
    )
)]
pub struct RolesApiDoc;

pub fn router() -> Router<ServiceContainer> {
    Router::new()
        .route("/", get(get_all))
        .route("/{id}", get(get_by_id))
}

#[utoipa::path(
    get,
    summary = "Get All Roles",
    path = "/api/roles",
    tag = "Roles",
    responses(
        (status = 200, description = "A list of roles", body = Vec<Role>),
        (status = 401, description = "Unauthorized - invalid or expired session"),
        (status = 403, description = "Forbidden - requires an administrator role"),
    ),
    description = "Retrieves a list of roles."
)]
pub async fn get_all(
    _: AdminUser,
    State(container): State<ServiceContainer>,
) -> Result<Json<Vec<Role>>, ApiError> {
    let roles = container.role_service().get_all().await?;
    Ok(Json(roles))
}

#[utoipa::path(
    get,
    summary = "Get Role By ID",
    path = "/api/roles/{id}",
    tag = "Roles",
    responses(
        (status = 200, description = "The role with the given ID.", body = Vec<Role>),
        (status = 401, description = "Unauthorized - invalid or expired session"),
        (status = 403, description = "Forbidden - requires an administrator role"),
    ),
    description = "Retrieves a role by its id."
)]
pub async fn get_by_id(
    _: AdminUser,
    State(container): State<ServiceContainer>,
    Path(id): Path<Uuid>,
) -> Result<Json<Role>, ApiError> {
    let role = container.role_service().get_by_id(id).await?;
    Ok(Json(role))
}
