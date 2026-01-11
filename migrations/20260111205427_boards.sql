-- Add migration script here
CREATE TABLE IF NOT EXISTS boards
(
    id          BIGSERIAL PRIMARY KEY,
    name        TEXT NOT NULL,
    description TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS threads
(
    id       BIGSERIAL PRIMARY KEY,
    body     TEXT NOT NULL,
    board_id BIGINT NOT NULL
        REFERENCES boards (id) ON DELETE CASCADE
);
