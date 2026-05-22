CREATE TABLE pinned_playlists (
    user_id uuid NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    playlist_id uuid NOT NULL REFERENCES playlists (id) ON DELETE CASCADE,
    position integer NOT NULL,
    pinned_at timestamptz NOT NULL DEFAULT NOW(),
    PRIMARY KEY (user_id, playlist_id)
);

CREATE INDEX idx_pinned_playlists_user_position
ON pinned_playlists (user_id, position);
