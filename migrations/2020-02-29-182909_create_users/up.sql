-- Your SQL goes here

CREATE TABLE users
(
    id         VARCHAR(36) NOT NULL PRIMARY KEY,
    created_at TIMESTAMP   NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP   NOT NULL DEFAULT NOW()
);