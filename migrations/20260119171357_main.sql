-- NOTE: unless we've actually deployed to prod, don't add any new
-- migrations and just edit this file until release
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
        REFERENCES boards (id) ON DELETE CASCADE,

    file_name TEXT,
    file_type TEXT
);
