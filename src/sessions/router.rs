use axum::{Json, Router, extract::State, routing::get};

use crate::{
    errors::ApiError, extractors::AdminUser, services::ServiceContainer, sessions::SessionSummary,
};

pub fn router() -> Router<ServiceContainer> {
    Router::new().route("/", get(session_summary))
}

// Clippy lint triggered by utoipa macro expansion, not our code
#[allow(clippy::needless_for_each)]
#[derive(utoipa::OpenApi)]
#[openapi(
    paths(
        crate::sessions::session_summary,
    ),
    components(
        schemas(SessionSummary)
    ),
    tags(
        (
            name = "Sessions", 
            description = "Session management"
        )
    )
)]
pub struct SessionApiDoc;

#[utoipa::path(
    get,
    summary = "Get Session Summary Details",
    path = "/api/sessions",
    tag = "Sessions",
    responses(
        (status = 200, description = "A list of session summary details", body = Vec<SessionSummary>),
        (status = 401, description = "Unauthorized - invalid or expired session"),
        (status = 403, description = "Forbidden - requires an administrator role"),
    ),
    description = "Retrieves a list of users with active sessions and the the count."
)]
pub async fn session_summary(
    _: AdminUser,
    State(container): State<ServiceContainer>,
) -> Result<Json<Vec<SessionSummary>>, ApiError> {
    let summaries = container.session_service().get_session_summaries().await?;
    Ok(Json(summaries))
}
