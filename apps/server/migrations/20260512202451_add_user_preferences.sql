ALTER TABLE users ADD COLUMN preferences jsonb NOT NULL DEFAULT '{}'::jsonb;
