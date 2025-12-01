use axum::Json;
use axum::Router;
use axum::extract::Path;
use axum::extract::Query;
use axum::extract::State;
use axum::routing::get;
use serde::{self, Deserialize};

use crate::auth::AuthUser;
use crate::errors::ApiError;
use crate::recipes::Recipe;
use crate::recipes::RecipeRequest;
use crate::services::ServiceContainer;
use crate::shared_models::PaginatedResponse;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecipeFilters {
    #[serde(default = "default_page")]
    pub page: i64,
    #[serde(default = "default_page_size")]
    pub page_size: i64,
    pub q: Option<String>, // Filter by name
}

const fn default_page() -> i64 {
    1
}
const fn default_page_size() -> i64 {
    20
}

pub fn router() -> Router<ServiceContainer> {
    Router::new()
        .route("/", get(get_all_recipes).post(create_recipe))
        .route(
            "/{id}",
            get(get_by_id).put(update_recipe).delete(delete_recipe),
        )
}

pub async fn get_all_recipes(
    auth: AuthUser,
    State(container): State<ServiceContainer>,
    Query(filters): Query<RecipeFilters>,
) -> Result<Json<PaginatedResponse<Recipe>>, ApiError> {
    let recipes = container
        .recipe_service()
        .get_user_and_public_recipes(
            auth.user.id,
            filters.page,
            filters.page_size,
            filters.q.as_deref(),
        )
        .await?;

    Ok(Json(recipes))
}

pub async fn get_by_id(
    auth: AuthUser,
    Path(id): Path<i32>,
    State(container): State<ServiceContainer>,
) -> Result<Json<Recipe>, ApiError> {
    let recipe = container
        .recipe_service()
        .get_by_id(id, auth.user.id)
        .await?;

    Ok(Json(recipe))
}

pub async fn create_recipe(
    auth: AuthUser,
    State(container): State<ServiceContainer>,
    Json(request): Json<RecipeRequest>,
) -> Result<Json<Recipe>, ApiError> {
    let recipe = container
        .recipe_service()
        .create_recipe(auth.user.id, request)
        .await?;

    Ok(Json(recipe))
}

pub async fn update_recipe(
    auth: AuthUser,
    State(container): State<ServiceContainer>,
    Path(id): Path<i32>,
    Json(request): Json<RecipeRequest>,
) -> Result<Json<Recipe>, ApiError> {
    let recipe = container
        .recipe_service()
        .update_recipe(id, auth.user.id, request)
        .await?;

    Ok(Json(recipe))
}

pub async fn delete_recipe(
    auth: AuthUser,
    State(container): State<ServiceContainer>,
    Path(id): Path<i32>,
) -> Result<(), ApiError> {
    container
        .recipe_service()
        .delete_recipe(id, auth.user.id)
        .await?;

    Ok(())
}
