-- Add migration script here
ALTER TABLE boards ADD CONSTRAINT unique_name UNIQUE (name);
