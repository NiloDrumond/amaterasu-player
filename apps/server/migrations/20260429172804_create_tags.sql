CREATE TABLE tag_categories (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id uuid NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    name text NOT NULL,
    color text,
    position integer NOT NULL DEFAULT 0,
    created_at timestamptz NOT NULL DEFAULT now(),
    updated_at timestamptz NOT NULL DEFAULT now(),
    UNIQUE (user_id, name)
);
CREATE INDEX idx_tag_categories_user ON tag_categories (user_id, position);
CREATE TRIGGER update_tag_categories_updated_at BEFORE UPDATE
ON tag_categories FOR EACH ROW EXECUTE PROCEDURE update_updated_at_column();

CREATE TABLE tags (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id uuid NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    category_id uuid REFERENCES tag_categories (id) ON DELETE SET NULL,
    name text NOT NULL,
    color text,
    created_at timestamptz NOT NULL DEFAULT now(),
    updated_at timestamptz NOT NULL DEFAULT now(),
    UNIQUE (user_id, name)
);
CREATE INDEX idx_tags_user_id ON tags (user_id);
CREATE INDEX idx_tags_user_category ON tags (user_id, category_id);
CREATE TRIGGER update_tags_updated_at BEFORE UPDATE
ON tags FOR EACH ROW EXECUTE PROCEDURE update_updated_at_column();

CREATE TABLE track_tags (
    track_id uuid NOT NULL REFERENCES tracks (id) ON DELETE CASCADE,
    tag_id uuid NOT NULL REFERENCES tags (id) ON DELETE CASCADE,
    added_at timestamptz NOT NULL DEFAULT now(),
    PRIMARY KEY (track_id, tag_id)
);
CREATE INDEX idx_track_tags_tag ON track_tags (tag_id);

CREATE TABLE album_tags (
    album_id uuid NOT NULL REFERENCES albums (id) ON DELETE CASCADE,
    tag_id uuid NOT NULL REFERENCES tags (id) ON DELETE CASCADE,
    added_at timestamptz NOT NULL DEFAULT now(),
    PRIMARY KEY (album_id, tag_id)
);
CREATE INDEX idx_album_tags_tag ON album_tags (tag_id);
