# Amaterasu Music Server - Plan V2

## Core Vision & Unique Features
**Vision:** A personal music server where YOU control the metadata and organization, not the files.

### Key Differentiators from Navidrome/Others
1. **Metadata Override System:** Edit any song/album metadata without touching files - DB acts as source of truth
2. **Advanced Tagging:** Multi-dimensional tags (genre, vibe, instruments, mood, etc.) with dynamic playlist generation
3. **Clean Separation:** Files are just audio streams; all metadata lives in DB and can be freely customized

### Non-goals (for now)
- DLNA/UPnP, podcasting, video
- Native mobile apps (ensure APIs support them later)
- Real-time features (WebSocket/SSE)
- Advanced audio analysis (AcoustID, chromaprint)

## Architecture Decisions (Updated)

### Backend Stack
- **Framework:** axum (async Rust, tower middleware, good for learning)
- **Database:** Postgres + SQLx (compile-time checked queries, migrations)
- **Background Jobs:** tokio-cron (simpler than apalis, well-maintained, pure Rust)
- **Search:** Tantivy embedded (sufficient for personal use, can scale later)
- **Media:** 
  - symphonia for metadata/duration (where supported)
  - FFmpeg for transcoding (OPUS support requirement)
  - lofty as fallback for tags
- **Auth:** Simple session tokens to start (argon2id passwords)

### Frontend Stack
- Next.js 15 (App Router, RSC)
- Tailwind + shadcn/ui
- TanStack Query + generated OpenAPI client
- Rich metadata editor components (crucial feature)

### Development & Quality
- **Migrations:** SQLx migrate (learn SQL properly)
- **Testing:** Start simple - integration tests for API endpoints, unit tests for business logic
- **API Contract:** OpenAPI via utoipa → TypeScript codegen

## Enhanced Domain Model

```sql
-- Core media structure (from file scanning)
CREATE TABLE artists (
    id UUID PRIMARY KEY,
    original_name TEXT NOT NULL,  -- From file tags
    file_mbid TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE albums (
    id UUID PRIMARY KEY,
    artist_id UUID REFERENCES artists(id),
    original_title TEXT NOT NULL,  -- From file tags
    original_year INT,
    file_mbid TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE tracks (
    id UUID PRIMARY KEY,
    album_id UUID REFERENCES albums(id),
    file_path TEXT NOT NULL UNIQUE,  -- Immutable reference to actual file
    
    -- Original metadata from file (never changes unless file is replaced)
    original_title TEXT NOT NULL,
    original_artist TEXT,
    original_album TEXT,
    original_disc INT,
    original_track_no INT,
    
    -- Technical metadata (from file, updated on rescan)
    duration_ms INT NOT NULL,
    format TEXT NOT NULL,
    bitrate INT,
    sample_rate INT,
    channels INT,
    file_size_bytes BIGINT,
    file_modified_at TIMESTAMPTZ,
    
    -- Analysis results
    replaygain_track_gain REAL,
    replaygain_album_gain REAL,
    
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- User-editable metadata overrides (the killer feature!)
CREATE TABLE track_metadata (
    track_id UUID PRIMARY KEY REFERENCES tracks(id),
    
    -- User can override any of these
    custom_title TEXT,           -- e.g., translated title
    custom_artist TEXT,
    custom_album TEXT,
    custom_album_artist TEXT,
    custom_disc INT,
    custom_track_no INT,
    custom_year INT,
    custom_composer TEXT,
    custom_comment TEXT,
    
    -- Additional user-only fields
    personal_rating INT CHECK (personal_rating >= 1 AND personal_rating <= 5),
    play_count INT DEFAULT 0,
    skip_count INT DEFAULT 0,
    loved BOOLEAN DEFAULT FALSE,
    
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Flexible tagging system
CREATE TABLE tags (
    id UUID PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    category TEXT,  -- 'genre', 'vibe', 'instrument', 'mood', 'custom', etc.
    color TEXT,      -- For UI
    description TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE track_tags (
    track_id UUID REFERENCES tracks(id),
    tag_id UUID REFERENCES tags(id),
    confidence REAL DEFAULT 1.0,  -- For auto-tagged vs manual (1.0 = manual)
    added_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (track_id, tag_id)
);

-- Smart/Dynamic playlists
CREATE TABLE playlists (
    id UUID PRIMARY KEY,
    user_id UUID REFERENCES users(id),
    name TEXT NOT NULL,
    description TEXT,
    
    -- If true, uses rule_definition instead of track list
    is_smart BOOLEAN DEFAULT FALSE,
    rule_definition JSONB,  -- e.g., {"tags": {"all": ["piano", "relaxing"], "any": ["video game", "anime"]}}
    
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Simple auth (can enhance later)
CREATE TABLE users (
    id UUID PRIMARY KEY,
    username TEXT NOT NULL UNIQUE,
    password_hash TEXT NOT NULL,
    is_admin BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE sessions (
    token TEXT PRIMARY KEY,
    user_id UUID REFERENCES users(id),
    expires_at TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
```

## API Design (Native, not Subsonic)

### Metadata Endpoints (Core Feature)
- `GET /api/tracks/:id` - Returns merged metadata (custom overrides original)
- `PATCH /api/tracks/:id/metadata` - Update custom metadata
- `DELETE /api/tracks/:id/metadata` - Reset to original
- `POST /api/tracks/:id/tags` - Add tags
- `DELETE /api/tracks/:id/tags/:tagId` - Remove tag

### Tag Management
- `GET /api/tags` - List all tags with categories
- `POST /api/tags` - Create new tag
- `GET /api/tags/suggest?track=:id` - AI/rule-based tag suggestions

### Dynamic Playlists
- `POST /api/playlists/generate` - Generate playlist from tag query
  ```json
  {
    "all_tags": ["piano", "relaxing"],
    "any_tags": ["video game soundtrack", "anime"],
    "exclude_tags": ["vocal"],
    "limit": 50,
    "order": "random"
  }
  ```

### Streaming
- `GET /api/tracks/:id/stream` - Range-enabled streaming
- `GET /api/tracks/:id/transcode?format=opus&bitrate=128` - On-the-fly transcode

## Simplified Phases

### Phase 0: Foundation (Week 1)
- Monorepo setup, SQLx migrations, basic axum server
- Docker-compose with Postgres
- Health endpoints, logging
- ✅ **Done when:** `docker-compose up` works, migrations run

### Phase 1: Core Scanning & Storage (Week 2-3)
- Library scanning with symphonia/lofty
- Dual metadata system (original + custom)
- Basic CRUD for tracks/albums/artists
- ✅ **Done when:** Can scan a folder and see tracks in DB with correct schema

### Phase 2: Streaming & Basic UI (Week 4-5)
- Range-based streaming endpoint
- Simple Next.js UI with track listing
- Basic playback (HTML5 audio)
- ✅ **Done when:** Can play a song with seek support

### Phase 3: THE KILLER FEATURES (Week 6-8)
- **Metadata editing UI** - Rich form for editing track/album metadata
- **Tag system** - Create, assign, manage tags
- **Dynamic playlists** - Generate from tag combinations
- Tantivy search integration
- ✅ **Done when:** Can edit metadata, tag songs, and generate playlists from tags

### Phase 4: Auth & Polish (Week 9-10)
- User auth (simple session tokens)
- Settings/preferences
- Transcode profiles (FFmpeg)
- Cover art management
- ✅ **Done when:** Multi-user works, can configure transcode settings

### Phase 5: Subsonic Compatibility (Optional, Week 11-12)
- OpenSubsonic adapter at `/rest/*`
- Only if you want to use existing apps
- ✅ **Done when:** Can use a Subsonic client

## Testing Strategy (Learning Opportunity)

```rust
// Start with integration tests for your API
#[cfg(test)]
mod tests {
    use axum_test::TestServer;
    
    #[tokio::test]
    async fn test_track_metadata_override() {
        // Test that custom metadata overrides original
    }
    
    #[tokio::test]
    async fn test_dynamic_playlist_generation() {
        // Test tag-based playlist generation
    }
}
```

Focus on testing the unique features (metadata override, tag queries) rather than basic CRUD.

## Development Workflow

```bash
# Development commands (using just)
just db-reset        # Reset DB and run migrations
just scan ~/music    # Scan music folder
just test           # Run tests
just api-docs       # Generate OpenAPI schema

# For Raspberry Pi optimization
just build-release  # Build with --release for ARM
```

## Performance Considerations (Raspberry Pi 4)

1. **Transcoding:** Cache aggressively, limit concurrent jobs
2. **Scanning:** Use inotify for changes instead of polling
3. **Database:** Connection pooling, careful with N+1 queries
4. **Search:** Tantivy index on SD card might be slow - consider RAM disk
5. **Images:** Thumbnail generation should be async/queued

## Configuration (via environment)

```env
# .env.example
DATABASE_URL=postgres://user:pass@localhost/amaterasu
LIBRARY_PATH=/music
CACHE_DIR=/var/cache/amaterasu
TRANSCODE_CACHE_SIZE_GB=10
RUST_LOG=amaterasu=debug,tower_http=debug
SESSION_SECRET=change-me-in-production
ENABLE_REGISTRATION=false  # After first user
```

## Why This Approach Will Succeed

1. **Clear unique value:** Metadata editing + tags solve real problems Navidrome doesn't
2. **Pragmatic choices:** FFmpeg for OPUS, simple auth, SQLx for learning
3. **Focused scope:** No podcasts, no video, no DLNA - just music done right
4. **Learning-friendly:** Good Rust patterns, SQL practice, but not overengineered
5. **Raspberry Pi ready:** Considered performance from the start

## Next Steps

1. Set up the monorepo structure
2. Create initial SQLx migrations with the dual-metadata schema
3. Implement basic library scanning
4. Build the metadata override system (your main differentiator)
5. Create a great tag management UI

The key insight: **Files are immutable audio sources; the database is where YOUR music library actually lives.**
