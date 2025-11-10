// src/recipes/service.rs
use crate::errors::ApiError;
use crate::recipes::{Ingredient, Instruction, Recipe, RecipeBase};
use sqlx::PgPool;
use std::collections::HashMap;

#[derive(Clone)]
pub struct RecipeService {
    pool: PgPool,
}

impl RecipeService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Get all recipes with ingredients and instructions
    pub async fn get_all(&self) -> Result<Vec<Recipe>, ApiError> {
        // Fetch base recipes
        let recipe_bases = sqlx::query_as!(
            RecipeBase,
            "SELECT id, name, author, description, difficulty, estimated_duration FROM recipes.recipes")
        .fetch_all(&self.pool)
        .await
        .map_err(ApiError::from)?;

        // Fetch ingredients
        let ingredients = sqlx::query_as!(Ingredient, "SELECT * FROM recipes.ingredients")
            .fetch_all(&self.pool)
            .await
            .map_err(ApiError::from)?;

        // Fetch instructions
        let instructions = sqlx::query_as!(Instruction, "SELECT * FROM recipes.instructions")
            .fetch_all(&self.pool)
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
