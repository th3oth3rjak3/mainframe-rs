use std::collections::HashMap;

use rocket::Route;
use rocket::serde::json::Json;
use rocket::*;
use rocket_db_pools::Connection;

use crate::database::*;
use crate::errors::*;
use crate::recipes::Ingredient;
use crate::recipes::Instruction;
use crate::recipes::Recipe;
use crate::recipes::RecipeBase;

pub fn routes() -> Vec<Route> {
    routes![get_all_recipes]
}

#[get("/")]
pub async fn get_all_recipes(
    mut db: Connection<AppCentralDb>,
) -> Result<Json<Vec<Recipe>>, ApiError> {
    let recipe_bases = sqlx::query_as!(RecipeBase, "select * from recipes.recipes")
        .fetch_all(db.connection())
        .await
        .map_err(ApiError::from)?;

    let ingredients = sqlx::query_as!(Ingredient, "select * from recipes.ingredients")
        .fetch_all(db.connection())
        .await
        .map_err(ApiError::from)?;

    let instructions = sqlx::query_as!(Instruction, "select * from recipes.instructions")
        .fetch_all(db.connection())
        .await
        .map_err(ApiError::from)?;

    let mut recipe_map: HashMap<i32, Recipe> =
        recipe_bases.into_iter().map(|r| (r.id, r.into())).collect();

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

    let recipes: Vec<Recipe> = recipe_map.into_values().collect();
    Ok(recipes.into())
}
