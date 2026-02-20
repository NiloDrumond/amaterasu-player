CREATE TABLE IF NOT EXISTS albums (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    artist_id uuid REFERENCES artists (id) ON DELETE SET NULL,
    title text NOT NULL,
    sort_title text NOT NULL,
    date date,
    mbid text,
    replaygain_album_gain real,
    replaygain_album_peak real,
    created_at timestamptz NOT NULL DEFAULT now(),
    updated_at timestamptz NOT NULL DEFAULT now()
);

CREATE INDEX idx_albums_sort_title ON albums (sort_title);

CREATE INDEX idx_albums_artist_id ON albums (artist_id);

CREATE INDEX idx_albums_date ON albums (date)
WHERE
date IS NOT NULL;

CREATE INDEX idx_albums_mbid ON albums (mbid)
WHERE
mbid IS NOT NULL;

CREATE TRIGGER update_albums_updated_at
BEFORE UPDATE ON albums
FOR EACH ROW
EXECUTE PROCEDURE update_updated_at_column();
