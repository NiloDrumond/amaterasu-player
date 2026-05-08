ALTER TABLE playlists
ADD COLUMN type text NOT NULL DEFAULT 'manual'
CHECK (type IN ('manual', 'dynamic')),
ADD COLUMN filter_definition jsonb,
ADD COLUMN cached_track_count integer NOT NULL DEFAULT 0,
ADD COLUMN cached_total_duration_ms bigint NOT NULL DEFAULT 0;

CREATE TABLE album_collections (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id uuid NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    name text NOT NULL,
    filter_definition jsonb NOT NULL,
    created_at timestamptz NOT NULL DEFAULT now(),
    updated_at timestamptz NOT NULL DEFAULT now()
);
CREATE INDEX idx_album_collections_user_id ON album_collections (user_id);
CREATE TRIGGER update_album_collections_updated_at BEFORE UPDATE
ON album_collections FOR EACH ROW EXECUTE PROCEDURE update_updated_at_column();
