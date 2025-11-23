use axum::Json;
use axum::Router;
use axum::extract::State;
use axum::routing::get;

use crate::auth::AuthUser;
use crate::errors::ApiError;
use crate::recipes::Recipe;
use crate::services::ServiceContainer;

pub fn router() -> Router<ServiceContainer> {
    Router::new().route("/", get(get_all_recipes))
}

pub async fn get_all_recipes(
    _: AuthUser,
    State(module): State<ServiceContainer>,
) -> Result<Json<Vec<Recipe>>, ApiError> {
    // TODO: get recipes for the current user plus public ones
    let service = module.recipe_service;
    let recipes = service.get_all().await?;
    Ok(Json(recipes))
}
