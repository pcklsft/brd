WITH inserted_boards_cte AS (
    INSERT INTO boards (id, name, description)
    VALUES (1, 'g', 'general board')
    ON CONFLICT DO NOTHING
)
INSERT INTO posts (id, body, parent, board_id)
VALUES (1, 'This is an example thread.', NULL, 1),
        (2, 'This is an example post.', 1, 1)
ON CONFLICT DO NOTHING;
