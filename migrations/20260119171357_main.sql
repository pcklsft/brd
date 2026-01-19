-- Add migration script here
CREATE TABLE IF NOT EXISTS boards
(
    id          BIGSERIAL PRIMARY KEY,
    name        TEXT NOT NULL UNIQUE,
    description TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS posts
(
    id       BIGSERIAL PRIMARY KEY,
    body     TEXT NOT NULL,
    parent   BIGINT REFERENCES posts DEFAULT NULL,
    board_id BIGINT NOT NULL
        REFERENCES boards (id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS files
(
  id BIGSERIAL PRIMARY KEY,
  file_name TEXT NOT NULL,
  file_type TEXT NOT NULL,
  post_id BIGINT NOT NULL
    REFERENCES posts (id) ON DELETE CASCADE
);
