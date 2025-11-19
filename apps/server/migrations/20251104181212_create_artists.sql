-- Create artists table
CREATE TABLE IF NOT EXISTS artists (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name TEXT NOT NULL,
    mbid TEXT, -- MusicBrainz ID if available
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Index for name searches
CREATE INDEX idx_artists_name ON artists(name);

-- Index for MusicBrainz lookups
CREATE INDEX idx_artists_mbid ON artists(mbid) WHERE mbid IS NOT NULL;

-- Trigger to update updated_at
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ language 'plpgsql';

CREATE TRIGGER update_artists_updated_at BEFORE UPDATE
    ON artists FOR EACH ROW EXECUTE PROCEDURE 
    update_updated_at_column();