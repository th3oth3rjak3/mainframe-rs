use uuid::Uuid;

use crate::errors::ServiceError;
use crate::recipes::{
    IIngredientRepository, IInstructionRepository, IRecipeRepository, Ingredient, Instruction,
    Recipe, RecipeRequest,
};
use crate::shared_models::PaginatedResponse;
use std::sync::Arc;

#[async_trait::async_trait]
pub trait IRecipeService: Send + Sync {
    /// Get all recipes that belong to the current user and any recipes that are public.
    async fn get_user_and_public_recipes(
        &self,
        user_id: Uuid,
        page: i64,
        page_size: i64,
        name_query: Option<&str>,
    ) -> Result<PaginatedResponse<Recipe>, ServiceError>;

    /// Get a recipe by its id.
    async fn get_by_id(&self, recipe_id: Uuid, user_id: Uuid) -> Result<Recipe, ServiceError>;

    /// Create a new recipe for the user with the given `user_id`.
    async fn create_recipe(
        &self,
        user_id: Uuid,
        request: RecipeRequest,
    ) -> Result<Uuid, ServiceError>;

    /// Update an existing recipe when the user owns it.
    async fn update_recipe(
        &self,
        recipe_id: Uuid,
        user_id: Uuid,
        request: RecipeRequest,
    ) -> Result<(), ServiceError>;

    /// Delete a recipe only when the user owns it.
    async fn delete_recipe(&self, recipe_id: Uuid, user_id: Uuid) -> Result<(), ServiceError>;
}

#[derive(Clone)]
pub struct RecipeService {
    recipes: Arc<dyn IRecipeRepository>,
    ingredients: Arc<dyn IIngredientRepository>,
    instructions: Arc<dyn IInstructionRepository>,
}

impl RecipeService {
    pub fn new(
        recipe_repo: Arc<dyn IRecipeRepository>,
        ingredient_repo: Arc<dyn IIngredientRepository>,
        instruction_repo: Arc<dyn IInstructionRepository>,
    ) -> Self {
        Self {
            recipes: recipe_repo,
            ingredients: ingredient_repo,
            instructions: instruction_repo,
        }
    }
}

#[async_trait::async_trait]
impl IRecipeService for RecipeService {
    async fn get_user_and_public_recipes(
        &self,
        user_id: Uuid,
        page: i64,
        page_size: i64,
        name_query: Option<&str>,
    ) -> Result<PaginatedResponse<Recipe>, ServiceError> {
        if page_size <= 0 {
            return Err(ServiceError::BadRequest("invalid page size".into()));
        }

        // Get paginated recipe bases and total count from repository
        let (recipe_bases, total) = self
            .recipes
            .get_user_and_public_recipes(user_id, page, page_size, name_query)
            .await?;

        // If no recipes, return early
        if recipe_bases.is_empty() {
            return Ok(PaginatedResponse {
                data: vec![],
                page,
                page_size,
                total,
                total_pages: 0,
            });
        }

        // Collect recipe IDs
        let recipe_ids: Vec<Uuid> = recipe_bases.iter().map(|r| r.id).collect();

        // Fetch ingredients and instructions in parallel
        let (ingredients, instructions) = tokio::try_join!(
            self.ingredients.get_all_by_recipe_ids(&recipe_ids),
            self.instructions.get_all_by_recipe_ids(&recipe_ids)
        )?;

        // Build full Recipe objects by combining base + ingredients + instructions
        let recipes = recipe_bases
            .into_iter()
            .map(|base| {
                let recipe_ingredients: Vec<Ingredient> = ingredients
                    .iter()
                    .filter(|i| i.recipe_id == base.id)
                    .cloned()
                    .collect();

                let recipe_instructions: Vec<Instruction> = instructions
                    .iter()
                    .filter(|i| i.recipe_id == base.id)
                    .cloned()
                    .collect();

                Recipe {
                    id: base.id,
                    name: base.name,
                    author: base.author,
                    description: base.description,
                    difficulty: base.difficulty,
                    estimated_duration: base.estimated_duration,
                    user_id: base.user_id,
                    is_public: base.is_public,
                    ingredients: recipe_ingredients,
                    instructions: recipe_instructions,
                }
            })
            .collect();

        let total_pages = (total + page_size - 1) / page_size;

        Ok(PaginatedResponse {
            data: recipes,
            page,
            page_size,
            total,
            total_pages,
        })
    }

    async fn get_by_id(&self, recipe_id: Uuid, user_id: Uuid) -> Result<Recipe, ServiceError> {
        let recipe = self.recipes.get_by_id(recipe_id).await?;

        if recipe.is_public || recipe.user_id == user_id {
            Ok(recipe)
        } else {
            // If a recipe exists but a user doesn't own it or it isn't public, don't give away that information.
            // Just tell the user it wasn't found to prevent traversal attacks.
            Err(ServiceError::NotFound {
                entity: "recipe",
                property: "id",
                value: recipe_id.to_string(),
            })
        }
    }

    async fn create_recipe(
        &self,
        user_id: Uuid,
        request: RecipeRequest,
    ) -> Result<Uuid, ServiceError> {
        let recipe_id = Uuid::now_v7();
        self.recipes.create(user_id, recipe_id, request).await?;
        Ok(recipe_id)
    }

    async fn update_recipe(
        &self,
        recipe_id: Uuid,
        user_id: Uuid,
        request: RecipeRequest,
    ) -> Result<(), ServiceError> {
        let recipe = self.recipes.get_by_id(recipe_id).await?;
        if recipe.user_id != user_id {
            return Err(ServiceError::NotFound {
                entity: "recipe",
                property: "id",
                value: recipe_id.to_string(),
            });
        }

        self.recipes.update(recipe_id, request).await?;
        
        Ok(())
    }

    async fn delete_recipe(&self, recipe_id: Uuid, user_id: Uuid) -> Result<(), ServiceError> {
        let recipe = self.recipes.get_by_id(recipe_id).await?;

        if recipe.user_id != user_id {
            // If a recipe exists but a user doesn't own it, don't give away that information.
            // Just tell the user it wasn't found to prevent traversal attacks.
            return Err(ServiceError::NotFound {
                entity: "recipe",
                property: "id",
                value: recipe_id.to_string(),
            });
        }

        self.recipes.delete(recipe_id).await?;
        Ok(())
    }
}
