// src/recipes/service.rs
use crate::errors::ApiError;
use crate::recipes::{IngredientRepository, InstructionRepository, Recipe, RecipeRepository};
use std::collections::HashMap;
use std::sync::Arc;

type RecipeRepoImpl = Arc<dyn RecipeRepository + Send + Sync>;
type IngredientRepoImpl = Arc<dyn IngredientRepository + Send + Sync>;
type InstructionRepoImpl = Arc<dyn InstructionRepository + Send + Sync>;

#[derive(Clone)]
pub struct RecipeService {
    recipe_repo: RecipeRepoImpl,
    ingredient_repo: IngredientRepoImpl,
    instruction_repo: InstructionRepoImpl,
}

impl RecipeService {
    pub fn new(
        recipe_repo: RecipeRepoImpl,
        ingredient_repo: IngredientRepoImpl,
        instruction_repo: InstructionRepoImpl,
    ) -> Self {
        Self {
            recipe_repo,
            ingredient_repo,
            instruction_repo,
        }
    }

    /// Get all recipes with ingredients and instructions
    pub async fn get_all(&self) -> Result<Vec<Recipe>, ApiError> {
        // Fetch base recipes
        let recipe_bases = self
            .recipe_repo
            .get_all_bases()
            .await
            .map_err(ApiError::from)?;

        let ids = recipe_bases.iter().map(|r| r.id).collect::<Vec<_>>();
        let ingredients = self
            .ingredient_repo
            .get_all_by_recipe_ids(&ids)
            .await
            .map_err(ApiError::from)?;

        let instructions = self
            .instruction_repo
            .get_all_by_recipe_ids(&ids)
            .await
            .map_err(ApiError::from)?;

        // Assemble recipes
        let mut recipe_map: HashMap<i32, Recipe> = recipe_bases
            .into_iter()
            .map(|r| {
                (
                    r.id,
                    Recipe {
                        id: r.id,
                        name: r.name,
                        description: r.description,
                        author: r.author,
                        difficulty: r.difficulty,
                        estimated_duration: r.estimated_duration,
                        ingredients: vec![],
                        instructions: vec![],
                    },
                )
            })
            .collect();

        for ing in ingredients {
            if let Some(recipe) = recipe_map.get_mut(&ing.recipe_id) {
                recipe.ingredients.push(ing);
            }
        }

        for instr in instructions {
            if let Some(recipe) = recipe_map.get_mut(&instr.recipe_id) {
                recipe.instructions.push(instr);
            }
        }

        Ok(recipe_map.into_values().collect())
    }
}
