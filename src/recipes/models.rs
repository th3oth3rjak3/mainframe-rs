use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Recipe {
    pub id: i32,
    pub name: String,
    pub author: Option<String>,
    pub description: Option<String>,
    pub difficulty: Option<String>,
    pub estimated_duration: Option<String>,
    pub user_id: i32,
    pub is_public: bool,
    pub ingredients: Vec<Ingredient>,
    pub instructions: Vec<Instruction>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecipeBase {
    pub id: i32,
    pub name: String,
    pub author: Option<String>,
    pub description: Option<String>,
    pub difficulty: Option<String>,
    pub estimated_duration: Option<String>,
    pub user_id: i32,
    pub is_public: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Ingredient {
    pub id: i32,
    pub recipe_id: i32,
    pub position: i32,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Instruction {
    pub id: i32,
    pub recipe_id: i32,
    pub position: i32,
    pub description: String,
}

impl From<RecipeBase> for Recipe {
    fn from(base: RecipeBase) -> Self {
        Self {
            id: base.id,
            name: base.name,
            author: base.author,
            description: base.description,
            difficulty: base.difficulty,
            estimated_duration: base.estimated_duration,
            user_id: base.user_id,
            is_public: base.is_public,
            ingredients: vec![],
            instructions: vec![],
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecipeRequest {
    pub name: String,
    pub author: Option<String>,
    pub description: Option<String>,
    pub difficulty: Option<String>,
    pub estimated_duration: Option<String>,
    pub is_public: bool,
    pub ingredients: Vec<IngredientRequest>,
    pub instructions: Vec<InstructionRequest>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IngredientRequest {
    pub position: i32,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InstructionRequest {
    pub position: i32,
    pub description: String,
}
