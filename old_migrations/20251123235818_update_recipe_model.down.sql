-- Remove the indexes
DROP INDEX IF EXISTS recipes.idx_recipes_public;
DROP INDEX IF EXISTS recipes.idx_recipes_user;

-- Remove the columns
ALTER TABLE recipes.recipes
DROP COLUMN IF EXISTS is_public,
DROP COLUMN IF EXISTS user_id;
