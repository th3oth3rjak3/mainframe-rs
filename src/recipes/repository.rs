use crate::{
    errors::RepositoryError,
    recipes::{Ingredient, Instruction, RecipeBase},
};
use async_trait::async_trait;
use sqlx::PgPool;

#[async_trait::async_trait]
pub trait IRecipeRepository: Send + Sync {
    async fn get_all_bases(&self) -> Result<Vec<RecipeBase>, RepositoryError>;
    // async fn get_shared_with(&self, recipe_id: i32) -> Result<Vec<RecipeShare>, sqlx::Error>;
}

#[async_trait]
pub trait IIngredientRepository: Send + Sync {
    async fn get_all_by_recipe_ids(
        &self,
        recipe_ids: &[i32],
    ) -> Result<Vec<Ingredient>, RepositoryError>;
}

#[async_trait]
pub trait IInstructionRepository: Send + Sync {
    async fn get_all_by_recipe_ids(
        &self,
        recipe_ids: &[i32],
    ) -> Result<Vec<Instruction>, RepositoryError>;
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
impl IRecipeRepository for SqlxRecipeRepository {
    async fn get_all_bases(&self) -> Result<Vec<RecipeBase>, RepositoryError> {
        let recipes = sqlx::query_as!(RecipeBase, "SELECT * FROM recipes.recipes")
            .fetch_all(&self.pool)
            .await?;

        Ok(recipes)
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
impl IIngredientRepository for SqlxIngredientRepository {
    async fn get_all_by_recipe_ids(
        &self,
        recipe_ids: &[i32],
    ) -> Result<Vec<Ingredient>, RepositoryError> {
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
impl IInstructionRepository for SqlxInstructionRepository {
    async fn get_all_by_recipe_ids(
        &self,
        recipe_ids: &[i32],
    ) -> Result<Vec<Instruction>, RepositoryError> {
        let instructions: Vec<Instruction> =
            sqlx::query_as("SELECT * FROM recipes.instructions WHERE recipe_id = ANY($1)")
                .bind(recipe_ids)
                .fetch_all(&self.pool)
                .await?;

        Ok(instructions)
    }
}
