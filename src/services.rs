// src/services.rs
use crate::recipes::RecipeService;
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
        let recipe_service = RecipeService::new(pool.clone());
        Self {
            pool,
            recipe_service,
        }
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
