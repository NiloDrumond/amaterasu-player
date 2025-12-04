CREATE TABLE IF NOT EXISTS tracks (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid (),
    audio_hash bytea NOT NULL UNIQUE,
    album_id uuid REFERENCES albums (id) ON DELETE SET NULL,
    file_path text NOT NULL UNIQUE,
    title text NOT NULL,
    artist text,
    album text,
    album_artist text,
    disc int CHECK (disc >= 0),
    track_no int CHECK (track_no >= 0),
    year int CHECK (year >= 0),
    composer text,
    comment text,
    codec int NOT NULL CHECK (codec >= 0),
    duration_ms int NOT NULL CHECK (duration_ms >= 0),
    bitrate int CHECK (bitrate >= 0),
    sample_rate int CHECK (sample_rate >= 0),
    channels int CHECK (channels >= 0),
    file_size_bytes bigint CHECK (file_size_bytes >= 0),
    file_modified_at timestamptz,
    replaygain_track_gain real,
    replaygain_album_gain real,
    metadata_modified_at timestamptz,
    created_at timestamptz NOT NULL DEFAULT NOW(),
    updated_at timestamptz NOT NULL DEFAULT NOW()
);

CREATE UNIQUE INDEX idx_tracks_file_path ON tracks (file_path);

CREATE INDEX idx_tracks_title ON tracks (title);

CREATE INDEX idx_tracks_artist ON tracks (artist)
WHERE
    artist IS NOT NULL;

CREATE INDEX idx_tracks_album_id ON tracks (album_id);

CREATE INDEX idx_tracks_album ON tracks (album)
WHERE
    album IS NOT NULL;

CREATE INDEX idx_tracks_album_disc_track ON tracks (album_id, disc, track_no);

CREATE INDEX idx_tracks_year ON tracks (year)
WHERE
    year IS NOT NULL;

CREATE TRIGGER update_tracks_updated_at
    BEFORE UPDATE ON tracks
    FOR EACH ROW
    EXECUTE PROCEDURE update_updated_at_column ();

