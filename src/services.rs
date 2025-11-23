use crate::{
    recipes::{
        IRecipeService, RecipeService, SqlxIngredientRepository, SqlxInstructionRepository,
        SqlxRecipeRepository,
    },
    users::{IUserService, SqlxUserRepository, UserService},
};
use axum::extract::FromRef;
use sqlx::PgPool;
use std::sync::Arc;

#[derive(Clone)]
pub struct RecipeModule {
    pub recipe_service: Arc<dyn IRecipeService>,
}

impl RecipeModule {
    pub fn new(recipe_service: Arc<dyn IRecipeService>) -> Self {
        Self { recipe_service }
    }
}

#[derive(Clone)]
pub struct UserModule {
    pub user_service: Arc<dyn IUserService>,
}

impl UserModule {
    pub fn new(user_service: Arc<dyn IUserService>) -> Self {
        Self { user_service }
    }
}

/// A container holding all shared services and resources for the app
#[derive(Clone)]
pub struct ServiceContainer {
    pub recipe_service: Arc<dyn IRecipeService>,
    pub user_service: Arc<dyn IUserService>,
}

impl ServiceContainer {
    pub fn new(pool: PgPool) -> Self {
        Self {
            recipe_service: Arc::new(ServiceContainer::make_recipe_service(pool.clone())),
            user_service: Arc::new(ServiceContainer::make_user_service(pool.clone())),
        }
    }

    fn make_recipe_service(pool: PgPool) -> impl IRecipeService {
        let recipe_repo = Arc::new(SqlxRecipeRepository::new(pool.clone()));
        let ingredient_repo = Arc::new(SqlxIngredientRepository::new(pool.clone()));
        let instruction_repo = Arc::new(SqlxInstructionRepository::new(pool.clone()));

        RecipeService::new(recipe_repo, ingredient_repo, instruction_repo)
    }

    /// Handy method to get a RecipeService reference for handlers
    pub fn recipe_service(&self) -> Arc<dyn IRecipeService> {
        self.recipe_service.clone()
    }

    fn make_user_service(pool: PgPool) -> impl IUserService {
        let user_repo = Arc::new(SqlxUserRepository::new(pool.clone()));

        UserService::new(user_repo)
    }

    pub fn user_service(&self) -> Arc<dyn IUserService> {
        self.user_service.clone()
    }
}

impl FromRef<Arc<ServiceContainer>> for RecipeModule {
    fn from_ref(input: &Arc<ServiceContainer>) -> Self {
        RecipeModule::new(input.recipe_service())
    }
}

impl FromRef<Arc<ServiceContainer>> for UserModule {
    fn from_ref(input: &Arc<ServiceContainer>) -> Self {
        UserModule::new(input.user_service())
    }
}
