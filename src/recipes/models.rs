use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Recipe {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub author: Option<String>,
    pub description: Option<String>,
    pub difficulty: Option<String>,
    pub estimated_duration: Option<String>,
    pub is_public: bool,
    pub ingredients: Vec<Ingredient>,
    pub instructions: Vec<Instruction>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecipeBase {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub author: Option<String>,
    pub description: Option<String>,
    pub difficulty: Option<String>,
    pub estimated_duration: Option<String>,
    pub is_public: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Ingredient {
    pub id: Uuid,
    pub recipe_id: Uuid,
    pub position: i64,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Instruction {
    pub id: Uuid,
    pub recipe_id: Uuid,
    pub position: i64,
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
    pub position: i64,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InstructionRequest {
    pub position: i64,
    pub description: String,
}
