CREATE TABLE track_plays (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id uuid NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    track_id uuid NOT NULL REFERENCES tracks (id) ON DELETE CASCADE,
    context_album_id uuid REFERENCES albums (id) ON DELETE SET NULL,
    context_playlist_id uuid REFERENCES playlists (id) ON DELETE SET NULL,
    played_at timestamptz NOT NULL DEFAULT now()
);

CREATE INDEX idx_track_plays_user_track ON track_plays (user_id, track_id);
CREATE INDEX idx_track_plays_user_album ON track_plays (
    user_id, context_album_id
)
WHERE context_album_id IS NOT NULL;
CREATE INDEX idx_track_plays_user_playlist ON track_plays (
    user_id, context_playlist_id
)
WHERE context_playlist_id IS NOT NULL;
