CREATE TABLE IF NOT EXISTS artists (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    name text NOT NULL,
    sort_name text NOT NULL,
    mbid text,
    created_at timestamptz NOT NULL DEFAULT now(),
    updated_at timestamptz NOT NULL DEFAULT now()
);

CREATE INDEX idx_artists_sort_name ON artists (name);

CREATE INDEX idx_artists_mbid ON artists (mbid) WHERE mbid IS NOT NULL;

CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS trigger AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE 'plpgsql';

CREATE TRIGGER update_artists_updated_at BEFORE UPDATE
ON artists FOR EACH ROW EXECUTE PROCEDURE
update_updated_at_column();
