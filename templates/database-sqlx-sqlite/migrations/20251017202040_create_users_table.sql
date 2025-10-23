-- Add migration script here
-- Create users table
-- +migrate Up
CREATE TABLE IF NOT EXISTS users (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    email TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT (CURRENT_TIMESTAMP)
);

INSERT INTO users (email, name)
VALUES
    ('alice@example.com', 'Alice'),
    ('bob@example.com', 'Bob');

