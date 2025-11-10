use std::collections::HashMap;
use std::sync::Arc;

use axum::Json;
use axum::Router;
use axum::extract::State;
use axum::routing::get;

use crate::errors::ApiError;
use crate::recipes::Recipe;
use crate::services::ServiceContainer;

pub fn router() -> Router<Arc<ServiceContainer>> {
    Router::new().route("/", get(get_all_recipes))
}

pub async fn get_all_recipes(
    State(container): State<Arc<ServiceContainer>>,
) -> Result<Json<Vec<Recipe>>, ApiError> {
    let recipes = container.recipe_service().get_all().await?;
    Ok(Json(recipes))
}
