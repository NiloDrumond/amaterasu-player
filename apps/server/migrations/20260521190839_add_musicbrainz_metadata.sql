-- Per-entity MusicBrainz lookup state. Lets the bulk-retry endpoint find
-- entities that were never tried or whose lookup failed.
ALTER TABLE artists
ADD COLUMN mb_lookup_status text,
ADD COLUMN mb_lookup_attempted_at timestamptz;

ALTER TABLE albums
ADD COLUMN mb_lookup_status text,
ADD COLUMN mb_lookup_attempted_at timestamptz;

ALTER TABLE tracks
ADD COLUMN mb_lookup_status text,
ADD COLUMN mb_lookup_attempted_at timestamptz;

CREATE INDEX idx_albums_mb_lookup_pending
ON albums (created_at DESC)
WHERE mb_lookup_status IS NULL OR mb_lookup_status = 'failed';

CREATE INDEX idx_artists_mb_lookup_pending
ON artists (created_at DESC)
WHERE mb_lookup_status IS NULL OR mb_lookup_status = 'failed';

CREATE INDEX idx_tracks_mb_lookup_pending
ON tracks (created_at DESC)
WHERE mb_lookup_status IS NULL OR mb_lookup_status = 'failed';

-- Candidate metadata suggestions from external sources. Polymorphic via
-- (entity_type, entity_id) -- FK enforcement is at the app layer because we
-- want a single table for all entity kinds and JSONB proposed payloads.
CREATE TABLE metadata_suggestions (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    entity_type text NOT NULL,
    entity_id uuid NOT NULL,
    source text NOT NULL DEFAULT 'musicbrainz',
    mbid text NOT NULL,
    score smallint NOT NULL,
    rank smallint NOT NULL,
    proposed jsonb NOT NULL,
    raw jsonb,
    status text NOT NULL DEFAULT 'pending',
    created_at timestamptz NOT NULL DEFAULT now(),
    updated_at timestamptz NOT NULL DEFAULT now(),
    CHECK (entity_type IN ('album', 'artist', 'track')),
    CHECK (status IN ('pending', 'accepted', 'rejected', 'superseded'))
);

CREATE INDEX idx_metadata_suggestions_entity
ON metadata_suggestions (entity_type, entity_id, rank);

CREATE INDEX idx_metadata_suggestions_pending
ON metadata_suggestions (entity_type, entity_id)
WHERE status = 'pending';

CREATE INDEX idx_metadata_suggestions_mbid
ON metadata_suggestions (mbid);

-- A previously-rejected (entity, mbid) pair. Future lookups for the same
-- entity skip these MBIDs so the admin doesn't have to re-reject the same
-- wrong match.
CREATE TABLE metadata_rejections (
    entity_type text NOT NULL,
    entity_id uuid NOT NULL,
    mbid text NOT NULL,
    rejected_at timestamptz NOT NULL DEFAULT now(),
    PRIMARY KEY (entity_type, entity_id, mbid),
    CHECK (entity_type IN ('album', 'artist', 'track'))
);
