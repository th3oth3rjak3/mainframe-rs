use crate::{
    authentication::{AuthenticationService, IAuthenticationService, SqlxAuthenticationRepository}, recipes::{
        IRecipeService, RecipeService, SqlxIngredientRepository, SqlxInstructionRepository,
        SqlxRecipeRepository,
    }, roles::{IRoleService, RoleService, SqlxRoleRepository}, sessions::{ISessionService, SessionService, SqlxSessionRepository}, users::{IUserService, SqlxUserRepository, UserService}
};

use sqlx::SqlitePool;
use std::sync::Arc;

/// A container holding all shared services and resources for the app
#[derive(Clone)]
pub struct ServiceContainer {
    recipes: Arc<dyn IRecipeService>,
    users: Arc<dyn IUserService>,
    sessions: Arc<dyn ISessionService>,
    roles: Arc<dyn IRoleService>,
    auth: Arc<dyn IAuthenticationService>,
}

impl ServiceContainer {
    pub fn new(pool: SqlitePool) -> Self {
        Self {
            recipes: Arc::new(Self::make_recipe_service(pool.clone())),
            users: Arc::new(Self::make_user_service(pool.clone())),
            sessions: Arc::new(Self::make_session_service(pool.clone())),
            roles: Arc::new(Self::make_role_service(pool.clone())),
            auth: Arc::new(Self::make_authentication_service(pool))
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
        let user_repo = Arc::new(SqlxUserRepository::new(pool.clone()));
        let role_repo = Arc::new(SqlxRoleRepository::new(pool));

        UserService::new(user_repo, role_repo)
    }

    fn make_role_service(pool: SqlitePool) -> impl IRoleService {
        let role_repo = Arc::new(SqlxRoleRepository::new(pool));
        
        RoleService::new(role_repo)
    }

    pub fn role_service(&self) -> Arc<dyn IRoleService> {
        self.roles.clone()
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

    fn make_authentication_service(pool: SqlitePool) -> impl IAuthenticationService {
        let auth_repo = Arc::new(SqlxAuthenticationRepository::new(pool.clone()));
        let user_repo = Arc::new(SqlxUserRepository::new(pool.clone()));
        let role_repo = Arc::new(SqlxRoleRepository::new(pool.clone()));
        let session_repo = Arc::new(SqlxSessionRepository::new(pool));

        AuthenticationService::new(auth_repo, user_repo, role_repo, session_repo)
    }

    pub fn auth_service(&self) -> Arc<dyn IAuthenticationService> {
        self.auth.clone()
    }
}
