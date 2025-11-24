-- Add user_id and is_public columns to recipes
ALTER TABLE recipes.recipes
ADD COLUMN user_id INTEGER NOT NULL REFERENCES public.users(id) ON DELETE CASCADE,
ADD COLUMN is_public BOOLEAN NOT NULL DEFAULT false;

-- Add index for faster queries by user_id
CREATE INDEX IF NOT EXISTS idx_recipes_user ON recipes.recipes(user_id);

-- Add index for public recipes (useful for fetching all public recipes)
CREATE INDEX IF NOT EXISTS idx_recipes_public ON recipes.recipes(is_public) WHERE is_public = true;
