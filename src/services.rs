use crate::{
    recipes::{
        IRecipeService, RecipeService, SqlxIngredientRepository, SqlxInstructionRepository,
        SqlxRecipeRepository,
    },
    sessions::{ISessionService, SessionService, SqlxSessionRepository},
    users::{IUserService, SqlxUserRepository, UserService},
};

use sqlx::SqlitePool;
use std::sync::Arc;

/// A container holding all shared services and resources for the app
#[derive(Clone)]
pub struct ServiceContainer {
    recipes: Arc<dyn IRecipeService>,
    users: Arc<dyn IUserService>,
    sessions: Arc<dyn ISessionService>,
}

impl ServiceContainer {
    pub fn new(pool: SqlitePool) -> Self {
        Self {
            recipes: Arc::new(Self::make_recipe_service(pool.clone())),
            users: Arc::new(Self::make_user_service(pool.clone())),
            sessions: Arc::new(Self::make_session_service(pool)),
        }
    }

    fn make_recipe_service(pool: SqlitePool) -> impl IRecipeService {
        let recipe_repo = Arc::new(SqlxRecipeRepository::new(pool.clone()));
        let ingredient_repo = Arc::new(SqlxIngredientRepository::new(pool.clone()));
        let instruction_repo = Arc::new(SqlxInstructionRepository::new(pool));

        RecipeService::new(recipe_repo, ingredient_repo, instruction_repo)
    }

    /// Handy method to get a `RecipeService` reference for handlers
    pub fn recipe_service(&self) -> Arc<dyn IRecipeService> {
        self.recipes.clone()
    }

    fn make_user_service(pool: SqlitePool) -> impl IUserService {
        let user_repo = Arc::new(SqlxUserRepository::new(pool));

        UserService::new(user_repo)
    }

    pub fn user_service(&self) -> Arc<dyn IUserService> {
        self.users.clone()
    }

    fn make_session_service(pool: SqlitePool) -> impl ISessionService {
        let session_repo = Arc::new(SqlxSessionRepository::new(pool));
        SessionService::new(session_repo)
    }

    pub fn session_service(&self) -> Arc<dyn ISessionService> {
        self.sessions.clone()
    }
}
