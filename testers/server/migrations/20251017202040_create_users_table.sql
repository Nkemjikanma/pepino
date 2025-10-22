-- Add migration script here
-- Create users table
CREATE TABLE IF NOT EXISTS users (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    email TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    created_at TEXT NOT NULL DEFAULT (datetime ('now'))
);

-- Insert sample data
INSERT INTO
    users (email, name)
VALUES
    ('alice@example.com', 'Alice'),
    ('bob@example.com', 'Bob');
