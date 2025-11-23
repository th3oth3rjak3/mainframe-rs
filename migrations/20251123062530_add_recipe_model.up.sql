CREATE SCHEMA IF NOT EXISTS recipes;

CREATE TABLE IF NOT EXISTS recipes.recipes (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    author TEXT,
    description TEXT,
    difficulty TEXT,
    estimated_duration TEXT
);

CREATE TABLE IF NOT EXISTS recipes.ingredients (
    id SERIAL PRIMARY KEY,
    recipe_id INTEGER NOT NULL REFERENCES recipes.recipes(id) ON DELETE CASCADE,
    position INTEGER NOT NULL,
    description TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS recipes.instructions (
    id SERIAL PRIMARY KEY,
    recipe_id INTEGER NOT NULL REFERENCES recipes.recipes(id) ON DELETE CASCADE,
    position INTEGER NOT NULL,
    description TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_ingredients_recipe ON recipes.ingredients(recipe_id);
CREATE INDEX IF NOT EXISTS idx_instructions_recipe ON recipes.instructions(recipe_id);
