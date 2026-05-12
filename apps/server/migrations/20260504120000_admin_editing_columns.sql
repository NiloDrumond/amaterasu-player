-- Artists: add source_name (snapshot of original tag) and locked_at.
-- source_name is the stable scan-side key; name is what admins edit.
ALTER TABLE artists
ADD COLUMN source_name text NOT NULL,
ADD COLUMN locked_at timestamptz;

CREATE UNIQUE INDEX idx_artists_source_name ON artists (LOWER(source_name));

-- Albums: add source_title + source_album_artist_id (FK), and locked_at.
-- Scanner matches by (source_album_artist_id, source_title);
-- admin edits leave both untouched.
ALTER TABLE albums
ADD COLUMN source_title text NOT NULL,
ADD COLUMN source_album_artist_id uuid REFERENCES artists (
    id
) ON DELETE SET NULL,
ADD COLUMN locked_at timestamptz;

CREATE UNIQUE INDEX idx_albums_source_keys
ON albums (source_album_artist_id, LOWER(source_title))
WHERE source_album_artist_id IS NOT NULL;

CREATE UNIQUE INDEX idx_albums_source_title_no_artist
ON albums (LOWER(source_title))
WHERE source_album_artist_id IS NULL;

-- Tracks: soft delete + lock.
ALTER TABLE tracks
ADD COLUMN deleted_at timestamptz,
ADD COLUMN locked_at timestamptz;

CREATE INDEX idx_tracks_deleted_at ON tracks (
    deleted_at
) WHERE deleted_at IS NOT NULL;
