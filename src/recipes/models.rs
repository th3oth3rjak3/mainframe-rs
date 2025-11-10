use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Recipe {
    pub id: i32,
    pub name: String,
    pub author: Option<String>,
    pub description: Option<String>,
    pub difficulty: Option<String>,
    pub estimated_duration: Option<String>,
    pub ingredients: Vec<Ingredient>,
    pub instructions: Vec<Instruction>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecipeBase {
    pub id: i32,
    pub name: String,
    pub author: Option<String>,
    pub description: Option<String>,
    pub difficulty: Option<String>,
    pub estimated_duration: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Ingredient {
    pub id: i32,
    pub recipe_id: i32,
    pub position: i32,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Instruction {
    pub id: i32,
    pub recipe_id: i32,
    pub position: i32,
    pub description: String,
}

impl From<RecipeBase> for Recipe {
    fn from(base: RecipeBase) -> Self {
        Recipe {
            id: base.id,
            name: base.name,
            author: base.author,
            description: base.description,
            difficulty: base.difficulty,
            estimated_duration: base.estimated_duration,
            ingredients: vec![],
            instructions: vec![],
        }
    }
}
