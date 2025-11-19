-- Create albums table
CREATE TABLE IF NOT EXISTS albums (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    artist_id UUID REFERENCES artists(id) ON DELETE SET NULL,
    title TEXT NOT NULL,
    year INT,
    mbid TEXT, -- MusicBrainz ID if available
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Index for title searches
CREATE INDEX idx_albums_title ON albums(title);

-- Index for artist lookups
CREATE INDEX idx_albums_artist_id ON albums(artist_id);

-- Index for year filtering
CREATE INDEX idx_albums_year ON albums(year) WHERE year IS NOT NULL;

-- Index for MusicBrainz lookups
CREATE INDEX idx_albums_mbid ON albums(mbid) WHERE mbid IS NOT NULL;

-- Trigger to update updated_at
CREATE TRIGGER update_albums_updated_at BEFORE UPDATE
    ON albums FOR EACH ROW EXECUTE PROCEDURE 
    update_updated_at_column();