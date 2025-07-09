-- Your SQL goes here
CREATE TABLE patients (
                          id UUID PRIMARY KEY,
                          name VARCHAR NOT NULL,
                          dob DATE NOT NULL,
                          created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);