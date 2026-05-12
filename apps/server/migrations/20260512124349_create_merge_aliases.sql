CREATE TABLE artist_aliases (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    source_name text NOT NULL,
    artist_id uuid NOT NULL REFERENCES artists (id) ON DELETE CASCADE,
    created_at timestamptz NOT NULL DEFAULT now()
);

CREATE UNIQUE INDEX idx_artist_aliases_source_name
ON artist_aliases (lower(source_name));

CREATE INDEX idx_artist_aliases_artist_id ON artist_aliases (artist_id);

CREATE TABLE album_aliases (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    source_title text NOT NULL,
    source_album_artist_id uuid REFERENCES artists (id) ON DELETE CASCADE,
    album_id uuid NOT NULL REFERENCES albums (id) ON DELETE CASCADE,
    created_at timestamptz NOT NULL DEFAULT now()
);

CREATE UNIQUE INDEX idx_album_aliases_source_keys
ON album_aliases (source_album_artist_id, lower(source_title))
WHERE source_album_artist_id IS NOT NULL;

CREATE UNIQUE INDEX idx_album_aliases_source_title_no_artist
ON album_aliases (lower(source_title))
WHERE source_album_artist_id IS NULL;

CREATE INDEX idx_album_aliases_album_id ON album_aliases (album_id);
