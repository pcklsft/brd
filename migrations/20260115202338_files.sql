-- Add migration script here
CREATE TABLE IF NOT EXISTS files
(
  id BIGSERIAL PRIMARY KEY,
  file_name TEXT NOT NULL,
  file_type TEXT NOT NULL,
  
  board_id BIGINT NOT NULL
    REFERENCES boards (id) ON DELETE CASCADE,
     
  post_id BIGINT NOT NULL
    REFERENCES posts (id) ON DELETE CASCADE
);

ALTER TABLE posts ADD COLUMN
file_id BIGINT
    REFERENCES files (id);
