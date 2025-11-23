use crate::{
    recipes::{
        IRecipeService, RecipeService, SqlxIngredientRepository, SqlxInstructionRepository,
        SqlxRecipeRepository,
    },
    sessions::{ISessionService, SessionService, SqlxSessionRepository},
    users::{IUserService, SqlxUserRepository, UserService},
};

use sqlx::PgPool;
use std::sync::Arc;

/// A container holding all shared services and resources for the app
#[derive(Clone)]
pub struct ServiceContainer {
    recipe_service: Arc<dyn IRecipeService>,
    user_service: Arc<dyn IUserService>,
    session_service: Arc<dyn ISessionService>,
}

impl ServiceContainer {
    pub fn new(pool: PgPool) -> Self {
        Self {
            recipe_service: Arc::new(ServiceContainer::make_recipe_service(pool.clone())),
            user_service: Arc::new(ServiceContainer::make_user_service(pool.clone())),
            session_service: Arc::new(ServiceContainer::make_session_service(pool.clone())),
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

    fn make_session_service(pool: PgPool) -> impl ISessionService {
        let session_repo = Arc::new(SqlxSessionRepository::new(pool.clone()));
        SessionService::new(session_repo)
    }

    pub fn session_service(&self) -> Arc<dyn ISessionService> {
        self.session_service.clone()
    }
}
