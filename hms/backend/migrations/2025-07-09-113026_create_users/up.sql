-- Your SQL goes here
CREATE TABLE users (
                       id UUID PRIMARY KEY,
                       email VARCHAR NOT NULL UNIQUE,
                       password_hash VARCHAR NOT NULL,
                       role VARCHAR NOT NULL,
                       created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);