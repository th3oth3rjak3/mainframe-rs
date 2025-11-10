// src/services.rs
use crate::recipes::{
    RecipeService, SqlxIngredientRepository, SqlxInstructionRepository, SqlxRecipeRepository,
};
use axum::extract::FromRef;
use sqlx::PgPool;
use std::sync::Arc;

/// A container holding all shared services and resources for the app
#[derive(Clone)]
pub struct ServiceContainer {
    pub pool: PgPool,
    pub recipe_service: RecipeService,
}

impl ServiceContainer {
    pub fn new(pool: PgPool) -> Self {
        Self {
            pool: pool.clone(),
            recipe_service: ServiceContainer::make_recipe_service(pool.clone()),
        }
    }

    fn make_recipe_service(pool: PgPool) -> RecipeService {
        let recipe_repo = Arc::new(SqlxRecipeRepository::new(pool.clone()));
        let ingredient_repo = Arc::new(SqlxIngredientRepository::new(pool.clone()));
        let instruction_repo = Arc::new(SqlxInstructionRepository::new(pool.clone()));

        RecipeService::new(recipe_repo, ingredient_repo, instruction_repo)
    }

    /// Handy method to get a RecipeService reference for handlers
    pub fn recipe_service(&self) -> RecipeService {
        self.recipe_service.clone()
    }
}

impl FromRef<Arc<ServiceContainer>> for RecipeService {
    fn from_ref(input: &Arc<ServiceContainer>) -> Self {
        input.recipe_service()
    }
}
