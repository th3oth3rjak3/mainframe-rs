use async_trait::async_trait;

use crate::errors::ApiError;
use crate::recipes::{
    IIngredientRepository, IInstructionRepository, IRecipeRepository, Ingredient, Instruction,
    Recipe, RecipeRequest,
};
use crate::shared_models::PaginatedResponse;
use std::sync::Arc;

#[async_trait]
pub trait IRecipeService: Send + Sync {
    /// Get all recipes that belong to the current user and any recipes that are public.
    async fn get_user_and_public_recipes(
        &self,
        user_id: i32,
        page: i64,
        page_size: i64,
        name_query: Option<&str>,
    ) -> Result<PaginatedResponse<Recipe>, ApiError>;

    /// Get a recipe by its id.
    async fn get_by_id(&self, recipe_id: i32, user_id: i32) -> Result<Recipe, ApiError>;

    /// Create a new recipe for the user with the given `user_id`.
    async fn create_recipe(&self, user_id: i32, request: RecipeRequest)
    -> Result<Recipe, ApiError>;

    /// Update an existing recipe when the user owns it.
    async fn update_recipe(
        &self,
        recipe_id: i32,
        user_id: i32,
        request: RecipeRequest,
    ) -> Result<Recipe, ApiError>;

    /// Delete a recipe only when the user owns it.
    async fn delete_recipe(&self, recipe_id: i32, user_id: i32) -> Result<(), ApiError>;
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

#[async_trait]
impl IRecipeService for RecipeService {
    async fn get_user_and_public_recipes(
        &self,
        user_id: i32,
        page: i64,
        page_size: i64,
        name_query: Option<&str>,
    ) -> Result<PaginatedResponse<Recipe>, ApiError> {
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
        let recipe_ids: Vec<i32> = recipe_bases.iter().map(|r| r.id).collect();

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

    async fn get_by_id(&self, recipe_id: i32, user_id: i32) -> Result<Recipe, ApiError> {
        let recipe = self.recipes.get_by_id(recipe_id).await?;

        if recipe.is_public || recipe.user_id == user_id {
            Ok(recipe)
        } else {
            // If a recipe exists but a user doesn't own it or it isn't public, don't give away that information.
            // Just tell the user it wasn't found to prevent traversal attacks.
            Err(ApiError::not_found())
        }
    }

    async fn create_recipe(
        &self,
        user_id: i32,
        request: RecipeRequest,
    ) -> Result<Recipe, ApiError> {
        let recipe = self.recipes.create(user_id, request).await?;
        Ok(recipe)
    }

    async fn update_recipe(
        &self,
        recipe_id: i32,
        user_id: i32,
        request: RecipeRequest,
    ) -> Result<Recipe, ApiError> {
        let recipe = self.recipes.get_by_id(recipe_id).await?;
        if recipe.user_id != user_id {
            return Err(ApiError::not_found());
        }

        let updated = self.recipes.update(recipe_id, request).await?;
        Ok(updated)
    }

    async fn delete_recipe(&self, recipe_id: i32, user_id: i32) -> Result<(), ApiError> {
        let recipe = self.recipes.get_by_id(recipe_id).await?;

        if recipe.user_id != user_id {
            // If a recipe exists but a user doesn't own it, don't give away that information.
            // Just tell the user it wasn't found to prevent traversal attacks.
            return Err(ApiError::not_found());
        }

        self.recipes.delete(recipe_id).await?;
        Ok(())
    }
}
