use async_trait::async_trait;

use crate::errors::ApiError;
use crate::recipes::{
    IIngredientRepository, IInstructionRepository, IRecipeRepository, Ingredient, Instruction,
    Recipe,
};
use crate::shared_models::PaginatedResponse;
use std::sync::Arc;

#[async_trait]
pub trait IRecipeService: Send + Sync {
    async fn get_user_and_public_recipes(
        &self,
        user_id: i32,
        page: i64,
        page_size: i64,
        name_query: Option<&str>,
    ) -> Result<PaginatedResponse<Recipe>, ApiError>;
}

#[derive(Clone)]
pub struct RecipeService {
    recipe_repo: Arc<dyn IRecipeRepository>,
    ingredient_repo: Arc<dyn IIngredientRepository>,
    instruction_repo: Arc<dyn IInstructionRepository>,
}

impl RecipeService {
    pub fn new(
        recipe_repo: Arc<dyn IRecipeRepository>,
        ingredient_repo: Arc<dyn IIngredientRepository>,
        instruction_repo: Arc<dyn IInstructionRepository>,
    ) -> Self {
        Self {
            recipe_repo,
            ingredient_repo,
            instruction_repo,
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
            .recipe_repo
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
            self.ingredient_repo.get_all_by_recipe_ids(&recipe_ids),
            self.instruction_repo.get_all_by_recipe_ids(&recipe_ids)
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

        let total_pages = (total as f64 / page_size as f64).ceil() as i64;

        Ok(PaginatedResponse {
            data: recipes,
            page,
            page_size,
            total,
            total_pages,
        })
    }
}
