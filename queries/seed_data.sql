INSERT INTO boards (id, name, description)
VALUES (1, 'g', 'general board')
ON CONFLICT DO NOTHING;
