use crate::{
    errors::RepositoryError,
    recipes::{Ingredient, Instruction, Recipe, RecipeBase, RecipeRequest},
};
use async_trait::async_trait;
use sqlx::SqlitePool;
use uuid::Uuid;

#[async_trait::async_trait]
pub trait IRecipeRepository: Send + Sync {
    async fn get_user_and_public_recipes(
        &self,
        user_id: Uuid,
        page: i64,
        page_size: i64,
        name_query: Option<&str>,
    ) -> Result<(Vec<RecipeBase>, i64), RepositoryError>;

    async fn get_by_id(&self, recipe_id: Uuid) -> Result<Recipe, RepositoryError>;

    async fn create(
        &self,
        user_id: Uuid,
        recipe_id: Uuid,
        request: RecipeRequest,
    ) -> Result<(), RepositoryError>;

    async fn update(
        &self,
        recipe_id: Uuid,
        request: RecipeRequest,
    ) -> Result<(), RepositoryError>;

    async fn delete(&self, recipe_id: Uuid) -> Result<(), RepositoryError>;
}

#[async_trait]
pub trait IIngredientRepository: Send + Sync {
    async fn get_all_by_recipe_ids(
        &self,
        recipe_ids: &[Uuid],
    ) -> Result<Vec<Ingredient>, RepositoryError>;
}

#[async_trait]
pub trait IInstructionRepository: Send + Sync {
    async fn get_all_by_recipe_ids(
        &self,
        recipe_ids: &[Uuid],
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
        user_id: Uuid,
        page: i64,
        page_size: i64,
        name_query: Option<&str>,
    ) -> Result<(Vec<RecipeBase>, i64), RepositoryError> {
        let offset = (page - 1) * page_size;

        let total: i64 = sqlx::query_scalar!(
            "SELECT COUNT(*) FROM recipes
             WHERE (user_id = ? OR is_public = true)
               AND (? IS NULL OR name LIKE '%' || ? || '%')",
            user_id,
            name_query,
            name_query
        )
        .fetch_one(&self.pool)
        .await?;

        let recipes = sqlx::query_as!(
            RecipeBase,
            r#"SELECT 
                id as "id: uuid::Uuid",
                user_id as "user_id: uuid::Uuid",
                author,
                name,
                description,
                difficulty,
                estimated_duration,
                is_public
            FROM recipes
            WHERE (user_id = ? OR is_public = true)
            AND (? IS NULL OR name LIKE '%' || ? || '%')
            ORDER BY name ASC
            LIMIT ? OFFSET ?"#,
            user_id,
            name_query,
            name_query,
            page_size,
            offset
        )
        .fetch_all(&self.pool)
        .await?;

        Ok((recipes, total))
    }

    async fn get_by_id(&self, recipe_id: Uuid) -> Result<Recipe, RepositoryError> {
        let base = sqlx::query_as!(
            RecipeBase,
            r#"SELECT 
                id as "id: uuid::Uuid",
                user_id as "user_id: uuid::Uuid",
                author,
                name,
                description,
                difficulty,
                estimated_duration,
                is_public
            FROM recipes 
            WHERE id = ?"#,
            recipe_id
        )
        .fetch_optional(&self.pool)
        .await?
        .ok_or(RepositoryError::NotFound {
            entity: "recipe",
            property: "id",
            value: recipe_id.to_string(),
        })?;

        let ingredients = sqlx::query_as!(
            Ingredient,
            r#"SELECT
                id as "id: uuid::Uuid",
                recipe_id as "recipe_id: uuid::Uuid",
                position,
                description
            FROM recipe_ingredients 
            WHERE recipe_id = ?"#,
            recipe_id
        )
        .fetch_all(&self.pool)
        .await?;

        let instructions = sqlx::query_as!(
            Instruction,
            r#"SELECT
                id as "id: uuid::Uuid",
                recipe_id as "recipe_id: uuid::Uuid",
                position,
                description
            FROM recipe_instructions 
            WHERE recipe_id = ?"#,
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
        user_id: Uuid,
        recipe_id: Uuid,
        request: RecipeRequest,
    ) -> Result<(), RepositoryError> {
        let mut tx = self.pool.begin().await?;

        sqlx::query!(
            r#"
            INSERT INTO recipes (id, name, author, description, difficulty, estimated_duration, is_public, user_id)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?)
            "#,
            recipe_id,
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
            let ingredient_id = Uuid::now_v7();

            sqlx::query!(
                "INSERT INTO recipe_ingredients (id, recipe_id, position, description) VALUES (?, ?, ?, ?)",
                ingredient_id,
                recipe_id,
                ingredient.position,
                ingredient.description)
            .execute(&mut *tx).await?;
        }

        for instruction in &request.instructions {
            let instruction_id = Uuid::now_v7();

            sqlx::query!(
                "INSERT INTO recipe_instructions (id, recipe_id, position, description) VALUES (?, ?, ?, ?)",
                instruction_id,
                recipe_id,
                instruction.position,
                instruction.description)
            .execute(&mut *tx).await?;
        }

        tx.commit().await?;

        Ok(())
    }

    async fn update(
        &self,
        recipe_id: Uuid,
        request: RecipeRequest,
    ) -> Result<(), RepositoryError> {
        let mut tx = self.pool.begin().await?;

        // Update the recipe header
        sqlx::query!(
            r#"
            UPDATE recipes
            SET name = ?, 
                author = ?, 
                description = ?, 
                difficulty = ?,
                estimated_duration = ?, 
                is_public = ?
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
            "DELETE FROM recipe_ingredients WHERE recipe_id = ?",
            recipe_id
        )
        .execute(&mut *tx)
        .await?;

        sqlx::query!(
            "DELETE FROM recipe_instructions WHERE recipe_id = ?",
            recipe_id
        )
        .execute(&mut *tx)
        .await?;

        // Insert new ingredients
        for ingredient in &request.ingredients {
            let id = Uuid::now_v7();
            sqlx::query!(
                "INSERT INTO recipe_ingredients (id, recipe_id, position, description) VALUES (?, ?, ?, ?)",
                id,
                recipe_id,
                ingredient.position,
                ingredient.description
            )
            .execute(&mut *tx)
            .await?;
        }

        // Insert new instructions
        for instruction in &request.instructions {
            let id = Uuid::now_v7();
            sqlx::query!(
                "INSERT INTO recipe_instructions (id, recipe_id, position, description) VALUES (?, ?, ?, ?)",
                id,
                recipe_id,
                instruction.position,
                instruction.description
            )
            .execute(&mut *tx)
            .await?;
        }

        tx.commit().await?;
        Ok(())
    }

    async fn delete(&self, recipe_id: Uuid) -> Result<(), RepositoryError> {
        sqlx::query!("DELETE FROM recipes WHERE id = ?", recipe_id)
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
        recipe_ids: &[Uuid],
    ) -> Result<Vec<Ingredient>, RepositoryError> {
        if recipe_ids.is_empty() {
            return Ok(Vec::new());
        }

        let params = vec!["?"; recipe_ids.len()].join(", ");
        let query_string = format!(
            r#"SELECT 
                id as "id: uuid::Uuid",
                recipe_id as "recipe_id: uuid::Uuid",
                position,
                description
            FROM recipe_ingredients
            WHERE recipe_id IN ({})"#,
            params
        );

        let mut query = sqlx::query_as::<_, Ingredient>(&query_string);

        for id in recipe_ids {
            query = query.bind(id);
        }

        let ingredients: Vec<Ingredient> = query.fetch_all(&self.pool).await?;

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
        recipe_ids: &[Uuid],
    ) -> Result<Vec<Instruction>, RepositoryError> {
        if recipe_ids.is_empty() {
            return Ok(Vec::new());
        }

        let params = vec!["?"; recipe_ids.len()].join(", ");
        let query_string = format!(
            r#"SELECT 
                id as "id: uuid::Uuid",
                recipe_id as "recipe_id: uuid::Uuid",
                position,
                description
            FROM recipe_instructions 
            WHERE recipe_id IN ({})"#,
            params
        );

        let mut query = sqlx::query_as::<_, Instruction>(&query_string);

        for id in recipe_ids {
            query = query.bind(id);
        }

        let instructions: Vec<Instruction> = query.fetch_all(&self.pool).await?;

        Ok(instructions)
    }
}
