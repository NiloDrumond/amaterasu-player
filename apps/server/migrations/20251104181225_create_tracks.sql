-- Create tracks table
CREATE TABLE IF NOT EXISTS tracks (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    album_id UUID REFERENCES albums(id) ON DELETE SET NULL,
    file_path TEXT NOT NULL UNIQUE, -- The only immutable reference to the file
    
    -- All metadata - initially from file, fully editable by user
    title TEXT NOT NULL,
    artist TEXT,
    album TEXT,
    album_artist TEXT,
    disc INT,
    track_no INT,
    year INT,
    composer TEXT,
    comment TEXT,
    
    -- Technical metadata (read-only, from file)
    duration_ms INT NOT NULL,
    format TEXT NOT NULL,
    bitrate INT,
    sample_rate INT,
    channels INT,
    file_size_bytes BIGINT,
    file_modified_at TIMESTAMPTZ,
    
    -- Audio analysis (optional, populated by scanner)
    replaygain_track_gain REAL,
    replaygain_album_gain REAL,
    
    -- Tracking user modifications
    metadata_modified_at TIMESTAMPTZ, -- NULL = never edited by user, Some = user has edited
    
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Index for file path lookups
CREATE UNIQUE INDEX idx_tracks_file_path ON tracks(file_path);

-- Index for title searches
CREATE INDEX idx_tracks_title ON tracks(title);

-- Index for artist searches
CREATE INDEX idx_tracks_artist ON tracks(artist) WHERE artist IS NOT NULL;

-- Index for album lookups
CREATE INDEX idx_tracks_album_id ON tracks(album_id);

-- Index for album grouping
CREATE INDEX idx_tracks_album ON tracks(album) WHERE album IS NOT NULL;

-- Index for track ordering within album
CREATE INDEX idx_tracks_album_disc_track ON tracks(album_id, disc, track_no);

-- Index for year filtering
CREATE INDEX idx_tracks_year ON tracks(year) WHERE year IS NOT NULL;

-- Trigger to update updated_at
CREATE TRIGGER update_tracks_updated_at BEFORE UPDATE
    ON tracks FOR EACH ROW EXECUTE PROCEDURE 
    update_updated_at_column();