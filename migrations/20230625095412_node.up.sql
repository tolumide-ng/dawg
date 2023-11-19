-- Add up migration script here
CREATE TABLE node(
    id BIGSERIAL PRIMARY KEY,
    letter TEXT NOT NULL,
    count BIGINT NOT NULL DEFAULT 0,
    terminal BOOLEAN NOT NULL DEFAULT false
);