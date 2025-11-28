CREATE TABLE IF NOT EXISTS albums (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid (),
    artist_id uuid REFERENCES artists (id) ON DELETE SET NULL,
    title text NOT NULL,
    year int,
    mbid text,
    created_at timestamptz NOT NULL DEFAULT NOW(),
    updated_at timestamptz NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_albums_title ON albums (title);

CREATE INDEX idx_albums_artist_id ON albums (artist_id);

CREATE INDEX idx_albums_year ON albums (year)
WHERE
    year IS NOT NULL;

CREATE INDEX idx_albums_mbid ON albums (mbid)
WHERE
    mbid IS NOT NULL;

CREATE TRIGGER update_albums_updated_at
    BEFORE UPDATE ON albums
    FOR EACH ROW
    EXECUTE PROCEDURE update_updated_at_column ();

