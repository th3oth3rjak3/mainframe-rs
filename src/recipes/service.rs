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

// Unit tests
#[cfg(test)]
mod tests {

    use async_trait::async_trait;

    use crate::{
        errors::RepositoryError,
        recipes::{Ingredient, Instruction, RecipeBase},
    };

    use super::*;

    // ----------------------
    // Mock repositories
    // ----------------------
    pub struct MockRecipeRepository {
        recipes: Vec<RecipeBase>,
        fail: bool,
    }

    #[async_trait]
    impl RecipeRepository for MockRecipeRepository {
        async fn get_all_bases(&self) -> Result<Vec<RecipeBase>, RepositoryError> {
            if self.fail {
                Err(RepositoryError::Other("forced recipe failure".into()))
            } else {
                Ok(self.recipes.clone())
            }
        }
    }

    pub struct MockIngredientRepository {
        ingredients: Vec<Ingredient>,
        fail: bool,
    }

    #[async_trait]
    impl IngredientRepository for MockIngredientRepository {
        async fn get_all_by_recipe_ids(
            &self,
            _recipe_ids: &[i32],
        ) -> Result<Vec<Ingredient>, RepositoryError> {
            if self.fail {
                Err(RepositoryError::Other("forced ingredient failure".into()))
            } else {
                Ok(self.ingredients.clone())
            }
        }
    }

    pub struct MockInstructionRepository {
        instructions: Vec<Instruction>,
        fail: bool,
    }

    #[async_trait]
    impl InstructionRepository for MockInstructionRepository {
        async fn get_all_by_recipe_ids(
            &self,
            _recipe_ids: &[i32],
        ) -> Result<Vec<Instruction>, RepositoryError> {
            if self.fail {
                Err(RepositoryError::Other("forced instruction failure".into()))
            } else {
                Ok(self.instructions.clone())
            }
        }
    }

    // ----------------------
    // Builder for RecipeService
    // ----------------------
    pub struct MockRecipeServiceBuilder {
        recipes: Option<Vec<RecipeBase>>,
        ingredients: Option<Vec<Ingredient>>,
        instructions: Option<Vec<Instruction>>,
        fail_recipe_repo: bool,
        fail_ingredient_repo: bool,
        fail_instruction_repo: bool,
    }

    impl MockRecipeServiceBuilder {
        pub fn new() -> Self {
            Self {
                recipes: None,
                ingredients: None,
                instructions: None,
                fail_recipe_repo: false,
                fail_ingredient_repo: false,
                fail_instruction_repo: false,
            }
        }

        pub fn with_recipes(mut self, recipes: Vec<RecipeBase>) -> Self {
            self.recipes = Some(recipes);
            self
        }

        pub fn with_ingredients(mut self, ingredients: Vec<Ingredient>) -> Self {
            self.ingredients = Some(ingredients);
            self
        }

        pub fn with_instructions(mut self, instructions: Vec<Instruction>) -> Self {
            self.instructions = Some(instructions);
            self
        }

        // Failure triggers
        pub fn fail_recipe_repo(mut self) -> Self {
            self.fail_recipe_repo = true;
            self
        }

        pub fn fail_ingredient_repo(mut self) -> Self {
            self.fail_ingredient_repo = true;
            self
        }

        pub fn fail_instruction_repo(mut self) -> Self {
            self.fail_instruction_repo = true;
            self
        }

        pub fn build(self) -> RecipeService {
            let recipe_repo = Arc::new(MockRecipeRepository {
                recipes: self.recipes.unwrap_or_default(),
                fail: self.fail_recipe_repo,
            });
            let ingredient_repo = Arc::new(MockIngredientRepository {
                ingredients: self.ingredients.unwrap_or_default(),
                fail: self.fail_ingredient_repo,
            });
            let instruction_repo = Arc::new(MockInstructionRepository {
                instructions: self.instructions.unwrap_or_default(),
                fail: self.fail_instruction_repo,
            });

            RecipeService::new(recipe_repo, ingredient_repo, instruction_repo)
        }
    }

    #[tokio::test]
    async fn test_recipe_service_mock_builder() {
        let recipe_service = MockRecipeServiceBuilder::new()
            .with_recipes(vec![RecipeBase {
                id: 1,
                name: "Test Recipe".into(),
                author: Some("Tester".into()),
                description: Some("Tasty".into()),
                difficulty: Some("easy".into()),
                estimated_duration: Some("10 min".into()),
            }])
            .with_ingredients(vec![])
            .with_instructions(vec![])
            .build();

        let recipes = recipe_service.get_all().await.unwrap();
        assert_eq!(recipes.len(), 1);
        assert_eq!(recipes[0].name, "Test Recipe");
    }

    #[tokio::test]
    async fn test_recipe_service_recipe_repo_failure() {
        let service = MockRecipeServiceBuilder::new().fail_recipe_repo().build();

        let result = service.get_all().await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "forced recipe failure");
    }

    #[tokio::test]
    async fn test_recipe_service_ingredient_repo_failure() {
        let service = MockRecipeServiceBuilder::new()
            .fail_ingredient_repo()
            .build();

        let result = service.get_all().await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "forced ingredient failure");
    }

    #[tokio::test]
    async fn test_recipe_service_instruction_repo_failure() {
        let service = MockRecipeServiceBuilder::new()
            .fail_instruction_repo()
            .build();

        let result = service.get_all().await;
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "forced instruction failure"
        );
    }
}
