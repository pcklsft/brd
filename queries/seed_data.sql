INSERT INTO boards (name, description)
VALUES ('g', 'general board')
ON CONFLICT DO NOTHING;
