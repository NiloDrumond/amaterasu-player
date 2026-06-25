CREATE TABLE track_favorites (
    user_id uuid NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    track_id uuid NOT NULL REFERENCES tracks (id) ON DELETE CASCADE,
    created_at timestamptz NOT NULL DEFAULT now(),
    PRIMARY KEY (user_id, track_id)
);
CREATE INDEX idx_track_favorites_user ON track_favorites (user_id);
