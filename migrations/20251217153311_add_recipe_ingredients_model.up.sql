-- Add up migration script here
CREATE TABLE recipe_ingredients (
    id TEXT NOT NULL PRIMARY KEY,
    recipe_id TEXT NOT NULL REFERENCES recipes(id) ON DELETE CASCADE,
    position INTEGER NOT NULL,
    description TEXT NOT NULL
);