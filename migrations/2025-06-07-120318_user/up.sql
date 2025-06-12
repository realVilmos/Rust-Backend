-- Your SQL goes here
CREATE TYPE token_type AS ENUM ('access', 'refresh');

CREATE TABLE users (
    user_id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    password VARCHAR NOT NULL,
    email VARCHAR NOT NULL UNIQUE
);

CREATE TABLE jwt_secrets (
    valid_for VARCHAR PRIMARY KEY,
    secret VARCHAR NOT NULL
)

CREATE TABLE active_tokens (
    token VARCHAR PRIMARY KEY,
    valid_for VARCHAR NOT NULL REFERENCES jwt_secrets(valid_for),
    type token_type NOT NULL,
    exp TIMESTAMP NOT NULL
)
