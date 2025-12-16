use crate::{
    errors::RepositoryError,
    recipes::{Ingredient, Instruction, Recipe, RecipeBase, RecipeRequest},
};
use async_trait::async_trait;
use sqlx::SqlitePool;

#[async_trait::async_trait]
pub trait IRecipeRepository: Send + Sync {
    async fn get_user_and_public_recipes(
        &self,
        user_id: i32,
        page: i64,
        page_size: i64,
        name_query: Option<&str>,
    ) -> Result<(Vec<RecipeBase>, i64), RepositoryError>;

    async fn get_by_id(&self, recipe_id: i32) -> Result<Recipe, RepositoryError>;

    async fn create(&self, user_id: i32, request: RecipeRequest)
    -> Result<Recipe, RepositoryError>;

    async fn update(
        &self,
        recipe_id: i32,
        request: RecipeRequest,
    ) -> Result<Recipe, RepositoryError>;

    async fn delete(&self, recipe_id: i32) -> Result<(), RepositoryError>;
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
    pub pool: SqlitePool,
}

impl SqlxRecipeRepository {
    pub const fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl IRecipeRepository for SqlxRecipeRepository {
    async fn get_user_and_public_recipes(
        &self,
        user_id: i32,
        page: i64,
        page_size: i64,
        name_query: Option<&str>,
    ) -> Result<(Vec<RecipeBase>, i64), RepositoryError> {
        let offset = page.saturating_sub(1).saturating_mul(page_size);

        let total: i64 = sqlx::query_scalar!(
            "SELECT COUNT(*) FROM recipes
             WHERE (user_id = ? OR is_public = true)
               AND ($2::TEXT IS NULL OR name ILIKE '%' || $2 || '%')",
            user_id,
            name_query
        )
        .fetch_one(&self.pool)
        .await?
        .unwrap_or(0);

        let recipes = sqlx::query_as!(
            RecipeBase,
            "SELECT * FROM recipes
             WHERE (user_id = ? OR is_public = true)
               AND (? IS NULL OR name LIKE '%' || ? || '%')
             ORDER BY name ASC
             LIMIT ? OFFSET ?",
            user_id,
            page_size.into(),
            offset.into(),
            name_query
        )
        .fetch_all(&self.pool)
        .await?;

        Ok((recipes, total))
    }

    async fn get_by_id(&self, recipe_id: i32) -> Result<Recipe, RepositoryError> {
        let base = sqlx::query_as!(
            RecipeBase,
            "SELECT * FROM recipes WHERE id = ?",
            recipe_id
        )
        .fetch_optional(&self.pool)
        .await?
        .ok_or(RepositoryError::NotFound)?;

        let ingredients = sqlx::query_as!(
            Ingredient,
            "SELECT * FROM ingredients WHERE recipe_id = ?",
            recipe_id
        )
        .fetch_all(&self.pool)
        .await?;

        let instructions = sqlx::query_as!(
            Instruction,
            "SELECT * FROM instructions WHERE recipe_id = ?",
            recipe_id
        )
        .fetch_all(&self.pool)
        .await?;

        let mut recipe: Recipe = base.into();

        recipe.ingredients = ingredients;
        recipe.instructions = instructions;

        Ok(recipe)
    }

    async fn create(
        &self,
        user_id: i32,
        request: RecipeRequest,
    ) -> Result<Recipe, RepositoryError> {
        let mut tx = self.pool.begin().await?;

        let id = sqlx::query_scalar!(
            r#"
            INSERT INTO recipes (name, author, description, difficulty, estimated_duration, is_public, user_id)
            VALUES (?, ?, ?, ?, ?, ?, ?)
            RETURNING id
            "#,
            request.name,
            request.author,
            request.description,
            request.difficulty,
            request.estimated_duration,
            request.is_public,
            user_id
        ).fetch_one(&mut *tx)
        .await?;

        for ingredient in &request.ingredients {
            sqlx::query!(
                "INSERT INTO ingredients (recipe_id, position, description) VALUES (?, ?, ?)",
                id,
                ingredient.position,
                ingredient.description)
            .execute(&mut *tx).await?;
        }

        for instruction in &request.instructions {
            sqlx::query!(
                "INSERT INTO instructions (recipe_id, position, description) VALUES (?, ?, ?)",
                id,
                instruction.position,
                instruction.description)
            .execute(&mut *tx).await?;
        }

        tx.commit().await?;

        self.get_by_id(id).await
    }

    async fn update(
        &self,
        recipe_id: i32,
        request: RecipeRequest,
    ) -> Result<Recipe, RepositoryError> {
        let mut tx = self.pool.begin().await?;

        // Update the recipe header
        sqlx::query!(
            r#"
            UPDATE recipes
            SET name = ?, author = ?, description = ?, difficulty = ?,
                estimated_duration = ?, is_public = ?
            WHERE id = ?
            "#,
            request.name,
            request.author,
            request.description,
            request.difficulty,
            request.estimated_duration,
            request.is_public,
            recipe_id
        )
        .execute(&mut *tx)
        .await?;

        // Delete old ingredients and instructions
        sqlx::query!(
            "DELETE FROM ingredients WHERE recipe_id = ?",
            recipe_id
        )
        .execute(&mut *tx)
        .await?;

        sqlx::query!(
            "DELETE FROM instructions WHERE recipe_id = ?",
            recipe_id
        )
        .execute(&mut *tx)
        .await?;

        // Insert new ingredients
        for ingredient in &request.ingredients {
            sqlx::query!(
                "INSERT INTO ingredients (recipe_id, position, description) VALUES (?, ?, ?)",
                recipe_id,
                ingredient.position,
                ingredient.description
            )
            .execute(&mut *tx)
            .await?;
        }

        // Insert new instructions
        for instruction in &request.instructions {
            sqlx::query!(
                "INSERT INTO instructions (recipe_id, position, description) VALUES (?, ?, ?)",
                recipe_id,
                instruction.position,
                instruction.description
            )
            .execute(&mut *tx)
            .await?;
        }

        tx.commit().await?;

        // Fetch and return the complete recipe
        self.get_by_id(recipe_id).await
    }

    async fn delete(&self, recipe_id: i32) -> Result<(), RepositoryError> {
        sqlx::query!("DELETE FROM recipes.recipes WHERE id = ?", recipe_id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}

pub struct SqlxIngredientRepository {
    pub pool: SqlitePool,
}

impl SqlxIngredientRepository {
    pub const fn new(pool: SqlitePool) -> Self {
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
    pub pool: SqlitePool,
}

impl SqlxInstructionRepository {
    pub const fn new(pool: SqlitePool) -> Self {
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
