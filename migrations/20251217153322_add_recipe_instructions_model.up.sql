-- Add up migration script here
CREATE TABLE recipe_instructions (
    id BLOB PRIMARY KEY NOT NULL,
    recipe_id BLOB NOT NULL REFERENCES recipes(id) ON DELETE CASCADE,
    position INTEGER NOT NULL,
    description TEXT NOT NULL
);