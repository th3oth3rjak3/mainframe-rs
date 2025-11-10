use std::sync::Arc;

use axum::Json;
use axum::Router;
use axum::extract::State;
use axum::routing::get;

use crate::errors::ApiError;
use crate::recipes::Recipe;
use crate::recipes::RecipeService;
use crate::services::ServiceContainer;

pub fn router() -> Router<Arc<ServiceContainer>> {
    Router::new().route("/", get(get_all_recipes))
}

pub async fn get_all_recipes(
    State(service): State<RecipeService>,
) -> Result<Json<Vec<Recipe>>, ApiError> {
    let recipes = service.get_all().await?;
    Ok(Json(recipes))
}
