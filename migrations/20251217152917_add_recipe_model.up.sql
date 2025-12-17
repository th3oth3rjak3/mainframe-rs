-- Add up migration script here
CREATE TABLE recipes (
    id TEXT NOT NULL PRIMARY KEY,
    user_id TEXT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    author TEXT,
    description TEXT,
    difficulty TEXT,
    estimated_duration TEXT,
    is_public BOOLEAN NOT NULL DEFAULT false
);