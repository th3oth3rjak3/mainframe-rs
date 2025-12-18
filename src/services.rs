use crate::{
    authentication::{
        AuthenticationService, IAuthenticationRepository, IAuthenticationService,
        SqlxAuthenticationRepository,
    },
    recipes::{
        IIngredientRepository, IInstructionRepository, IRecipeRepository, IRecipeService,
        RecipeService, SqlxIngredientRepository, SqlxInstructionRepository, SqlxRecipeRepository,
    },
    roles::{IRoleRepository, IRoleService, RoleService, SqlxRoleRepository},
    sessions::{ISessionRepository, ISessionService, SessionService, SqlxSessionRepository},
    users::{IUserRepository, IUserService, SqlxUserRepository, UserService},
};
use sqlx::SqlitePool;
use std::sync::Arc;

/// A container holding all shared repositories and services for the app
#[derive(Clone)]
pub struct ServiceContainer {
    // Repositories (shared across services)
    auth_repo: Arc<dyn IAuthenticationRepository>,
    user_repo: Arc<dyn IUserRepository>,
    role_repo: Arc<dyn IRoleRepository>,
    session_repo: Arc<dyn ISessionRepository>,
    recipe_repo: Arc<dyn IRecipeRepository>,
    ingredient_repo: Arc<dyn IIngredientRepository>,
    instruction_repo: Arc<dyn IInstructionRepository>,

    // Services
    recipes: Arc<dyn IRecipeService>,
    users: Arc<dyn IUserService>,
    sessions: Arc<dyn ISessionService>,
    roles: Arc<dyn IRoleService>,
    auth: Arc<dyn IAuthenticationService>,
}

impl ServiceContainer {
    pub fn new(pool: SqlitePool) -> Self {
        // Create all repositories once
        let auth_repo = Arc::new(SqlxAuthenticationRepository::new(pool.clone()));
        let user_repo = Arc::new(SqlxUserRepository::new(pool.clone()));
        let role_repo = Arc::new(SqlxRoleRepository::new(pool.clone()));
        let session_repo = Arc::new(SqlxSessionRepository::new(pool.clone()));
        let recipe_repo = Arc::new(SqlxRecipeRepository::new(pool.clone()));
        let ingredient_repo = Arc::new(SqlxIngredientRepository::new(pool.clone()));
        let instruction_repo = Arc::new(SqlxInstructionRepository::new(pool));

        // Create services using shared repositories
        let recipes = Arc::new(RecipeService::new(
            recipe_repo.clone(),
            ingredient_repo.clone(),
            instruction_repo.clone(),
        ));

        let users = Arc::new(UserService::new(user_repo.clone(), role_repo.clone()));

        let sessions = Arc::new(SessionService::new(session_repo.clone()));

        let roles = Arc::new(RoleService::new(role_repo.clone()));

        let auth = Arc::new(AuthenticationService::new(
            auth_repo.clone(),
            user_repo.clone(),
            role_repo.clone(),
            session_repo.clone(),
        ));

        Self {
            auth_repo,
            user_repo,
            role_repo,
            session_repo,
            recipe_repo,
            ingredient_repo,
            instruction_repo,
            recipes,
            users,
            sessions,
            roles,
            auth,
        }
    }

    // Repository accessors
    #[allow(unused)]
    pub fn auth_repo(&self) -> Arc<dyn IAuthenticationRepository> {
        self.auth_repo.clone()
    }

    #[allow(unused)]
    pub fn user_repo(&self) -> Arc<dyn IUserRepository> {
        self.user_repo.clone()
    }

    #[allow(unused)]
    pub fn role_repo(&self) -> Arc<dyn IRoleRepository> {
        self.role_repo.clone()
    }

    #[allow(unused)]
    pub fn session_repo(&self) -> Arc<dyn ISessionRepository> {
        self.session_repo.clone()
    }

    #[allow(unused)]
    pub fn recipe_repo(&self) -> Arc<dyn IRecipeRepository> {
        self.recipe_repo.clone()
    }

    #[allow(unused)]
    pub fn ingredient_repo(&self) -> Arc<dyn IIngredientRepository> {
        self.ingredient_repo.clone()
    }

    #[allow(unused)]
    pub fn instruction_repo(&self) -> Arc<dyn IInstructionRepository> {
        self.instruction_repo.clone()
    }

    // Service accessors
    #[allow(unused)]
    pub fn recipe_service(&self) -> Arc<dyn IRecipeService> {
        self.recipes.clone()
    }

    #[allow(unused)]
    pub fn user_service(&self) -> Arc<dyn IUserService> {
        self.users.clone()
    }

    #[allow(unused)]
    pub fn session_service(&self) -> Arc<dyn ISessionService> {
        self.sessions.clone()
    }

    #[allow(unused)]
    pub fn role_service(&self) -> Arc<dyn IRoleService> {
        self.roles.clone()
    }

    #[allow(unused)]
    pub fn auth_service(&self) -> Arc<dyn IAuthenticationService> {
        self.auth.clone()
    }
}
