use crate::recipes::{Ingredient, Instruction, RecipeBase};
use async_trait::async_trait;
use sqlx::PgPool;

#[async_trait::async_trait]
pub trait RecipeRepository {
    async fn get_all_bases(&self) -> Result<Vec<RecipeBase>, sqlx::Error>;
    // async fn get_shared_with(&self, recipe_id: i32) -> Result<Vec<RecipeShare>, sqlx::Error>;
}

#[async_trait]
pub trait IngredientRepository {
    async fn get_all_by_recipe_ids(
        &self,
        recipe_ids: &[i32],
    ) -> Result<Vec<Ingredient>, sqlx::Error>;
}

#[async_trait]
pub trait InstructionRepository {
    async fn get_all_by_recipe_ids(
        &self,
        recipe_ids: &[i32],
    ) -> Result<Vec<Instruction>, sqlx::Error>;
}

pub struct SqlxRecipeRepository {
    pub pool: PgPool,
}

impl SqlxRecipeRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl RecipeRepository for SqlxRecipeRepository {
    async fn get_all_bases(&self) -> Result<Vec<RecipeBase>, sqlx::Error> {
        sqlx::query_as!(RecipeBase, "SELECT * FROM recipes.recipes")
            .fetch_all(&self.pool)
            .await
    }

    // async fn get_shared_with_for_recipe_ids(
    //     &self,
    //     recipe_ids: &[i32],
    // ) -> Result<Vec<SharedWith>, sqlx::Error> {
    //     let ids = recipe_ids
    //         .iter()
    //         .map(|id| id.to_string())
    //         .collect::<Vec<_>>()
    //         .join(",");
    //     let query = format!(
    //         ,
    //         ids
    //     );
    //     sqlx::query_as(
    //         SharedWith,
    //         "SELECT * FROM recipes.recipe_shares WHERE recipe_id = ANY($1)",
    //     )
    //     .bind(recipe_ids)
    //     .fetch_all(&self.pool)
    //     .await
    // }
}

pub struct SqlxIngredientRepository {
    pub pool: PgPool,
}

impl SqlxIngredientRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl IngredientRepository for SqlxIngredientRepository {
    async fn get_all_by_recipe_ids(
        &self,
        recipe_ids: &[i32],
    ) -> Result<Vec<Ingredient>, sqlx::Error> {
        let ingredients: Vec<Ingredient> =
            sqlx::query_as("SELECT * FROM recipes.ingredients WHERE recipe_id = ANY($1)")
                .bind(recipe_ids)
                .fetch_all(&self.pool)
                .await?;

        Ok(ingredients)
    }
}

pub struct SqlxInstructionRepository {
    pub pool: PgPool,
}

impl SqlxInstructionRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl InstructionRepository for SqlxInstructionRepository {
    async fn get_all_by_recipe_ids(
        &self,
        recipe_ids: &[i32],
    ) -> Result<Vec<Instruction>, sqlx::Error> {
        let instructions: Vec<Instruction> =
            sqlx::query_as("SELECT * FROM recipes.instructions WHERE recipe_id = ANY($1)")
                .bind(recipe_ids)
                .fetch_all(&self.pool)
                .await?;

        Ok(instructions)
    }
}
