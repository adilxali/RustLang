-- Your SQL goes here
CREATE TABLE todos (
    id SERIAL PRIMARY KEY ,
    task VARCHAR NOT NULL ,
    completed BOOLEAN NOT NULL DEFAULT FALSE
);