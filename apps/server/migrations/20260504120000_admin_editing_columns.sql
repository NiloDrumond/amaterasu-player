-- Artists: add locked_at so admins can pin a row against scan-side overwrites.
ALTER TABLE artists ADD COLUMN locked_at timestamptz;

-- Albums: same lock column.
ALTER TABLE albums ADD COLUMN locked_at timestamptz;

-- Tracks: soft delete + lock.
ALTER TABLE tracks
ADD COLUMN deleted_at timestamptz,
ADD COLUMN locked_at timestamptz;

CREATE INDEX idx_tracks_deleted_at ON tracks (
    deleted_at
) WHERE deleted_at IS NOT NULL;
