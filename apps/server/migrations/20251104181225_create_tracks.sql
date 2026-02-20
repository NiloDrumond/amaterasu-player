CREATE TABLE IF NOT EXISTS tracks (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    audio_hash bytea NOT NULL UNIQUE,
    album_id uuid REFERENCES albums (id) ON DELETE SET NULL,
    file_path text NOT NULL UNIQUE,
    title text NOT NULL,
    sort_title text NOT NULL,
    artist_id uuid REFERENCES artists (id) ON DELETE SET NULL,
    disc int CHECK (disc >= 0),
    track_no int CHECK (track_no >= 0),
    date date,
    composer text,
    comment text,
    duration_ms int NOT NULL CHECK (duration_ms >= 0),
    bitrate int CHECK (bitrate >= 0),
    sample_rate bigint CHECK (sample_rate >= 0),
    channels int CHECK (channels >= 0),
    file_size_bytes bigint CHECK (file_size_bytes >= 0),
    file_modified_at timestamptz,
    replaygain_track_gain real,
    replaygain_track_peak real,
    metadata_modified_at timestamptz,
    created_at timestamptz NOT NULL DEFAULT now(),
    updated_at timestamptz NOT NULL DEFAULT now()
);

CREATE UNIQUE INDEX idx_tracks_file_path ON tracks (file_path);

CREATE INDEX idx_tracks_sort_title ON tracks (sort_title);

CREATE INDEX idx_tracks_artist_id ON tracks (artist_id);

CREATE INDEX idx_tracks_album_id ON tracks (album_id);

CREATE INDEX idx_tracks_album_disc_track ON tracks (album_id, disc, track_no);

CREATE INDEX idx_tracks_date ON tracks (date)
WHERE
date IS NOT NULL;

CREATE TRIGGER update_tracks_updated_at
BEFORE UPDATE ON tracks
FOR EACH ROW
EXECUTE PROCEDURE update_updated_at_column();
