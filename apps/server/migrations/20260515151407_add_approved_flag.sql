ALTER TABLE tracks
ADD COLUMN approved boolean NOT NULL DEFAULT FALSE;

ALTER TABLE albums
ADD COLUMN approved boolean NOT NULL DEFAULT FALSE;

ALTER TABLE artists
ADD COLUMN approved boolean NOT NULL DEFAULT FALSE;

CREATE INDEX idx_tracks_pending
ON tracks (album_id)
WHERE approved = FALSE AND deleted_at IS NULL;

CREATE INDEX idx_albums_pending
ON albums (created_at DESC)
WHERE approved = FALSE;

CREATE INDEX idx_artists_pending
ON artists (created_at DESC)
WHERE approved = FALSE;
