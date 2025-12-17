-- Add up migration script here
CREATE TABLE roles (
    id TEXT NOT NULL PRIMARY KEY,
    name TEXT NOT NULL UNIQUE
);