CREATE TABLE IF NOT EXISTS public.users (
    id SERIAL PRIMARY KEY,
    first_name TEXT NOT NULL,
    last_name TEXT NOT NULL,
    email TEXT NOT NULL UNIQUE,
    username TEXT NOT NULL UNIQUE,
    password TEXT NOT NULL,
    last_login TIMESTAMPTZ,
    is_admin BOOLEAN NOT NULL DEFAULT FALSE
);
