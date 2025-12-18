-- Add up migration script here
CREATE TABLE user_roles (
    user_id BLOB NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    role_id BLOB NOT NULL REFERENCES roles(id) ON DELETE CASCADE
);