CREATE TABLE IF NOT EXISTS sessions (
    id text PRIMARY KEY,
    user_id uuid NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    ip_address inet,
    metadata jsonb,

    expires_at timestamptz NOT NULL,

    created_at timestamptz NOT NULL DEFAULT now(),
    updated_at timestamptz NOT NULL DEFAULT now()
);

CREATE INDEX idx_sessions_expires_at ON sessions (expires_at);

CREATE TRIGGER update_sessions_updated_at
BEFORE UPDATE ON sessions
FOR EACH ROW
EXECUTE PROCEDURE update_updated_at_column();
