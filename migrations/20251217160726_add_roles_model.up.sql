-- Add up migration script here
CREATE TABLE roles (
    id BLOB PRIMARY KEY NOT NULL,
    name TEXT NOT NULL UNIQUE
);