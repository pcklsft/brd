-- Add migration script here
DROP TABLE IF EXISTS threads;

CREATE TABLE IF NOT EXISTS posts
(
    id       BIGSERIAL PRIMARY KEY,
    body     TEXT NOT NULL,
    parent   BIGINT REFERENCES posts DEFAULT NULL,
    board_id BIGINT NOT NULL
        REFERENCES boards (id) ON DELETE CASCADE
);
