CREATE TABLE playlists (
    id         uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id    uuid NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    name       text NOT NULL,
    created_at timestamptz NOT NULL DEFAULT now(),
    updated_at timestamptz NOT NULL DEFAULT now()
);
CREATE INDEX idx_playlists_user_id ON playlists(user_id);
CREATE TRIGGER update_playlists_updated_at BEFORE UPDATE
    ON playlists FOR EACH ROW EXECUTE PROCEDURE update_updated_at_column();

CREATE TABLE playlist_tracks (
    id          uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    playlist_id uuid NOT NULL REFERENCES playlists(id)  ON DELETE CASCADE,
    track_id    uuid NOT NULL REFERENCES tracks(id)     ON DELETE CASCADE,
    position    double precision NOT NULL,
    added_at    timestamptz NOT NULL DEFAULT now(),
    UNIQUE (playlist_id, track_id)
);
CREATE INDEX idx_playlist_tracks_playlist ON playlist_tracks(playlist_id, position);
