# Backend Implementation Todo

## Project Structure (Rust/Axum Standard)

```
apps/server/
├── Cargo.toml
├── .env.example
├── sqlx-data.json         # SQLx offline mode cache (generated)
├── migrations/            # SQL migrations
│   ├── 001_create_users.sql
│   ├── 002_create_media_tables.sql
│   └── ...
├── src/
│   ├── main.rs           # App entry point, server setup
│   ├── lib.rs            # Library root (optional, for testing)
│   ├── config.rs         # Configuration management
│   ├── error.rs          # Custom error types
│   ├── state.rs          # Application state (Arc<AppState>)
│   │
│   ├── db/               # Database layer
│   │   ├── mod.rs        # Pool setup, migrations
│   │   └── entities/     # Pure data structs matching DB
│   │       ├── mod.rs
│   │       ├── track.rs  # Track, TrackRow structs
│   │       ├── album.rs
│   │       ├── artist.rs
│   │       ├── tag.rs
│   │       └── user.rs
│   │
│   ├── models/           # Domain models (business logic)
│   │   ├── mod.rs
│   │   ├── track.rs      # TrackWithMetadata (merged view)
│   │   ├── playlist.rs   # SmartPlaylist logic
│   │   └── ...
│   │
│   ├── repositories/     # Data access layer
│   │   ├── mod.rs
│   │   ├── track_repository.rs    # SQL queries for tracks
│   │   ├── album_repository.rs
│   │   ├── artist_repository.rs
│   │   ├── tag_repository.rs
│   │   └── user_repository.rs
│   │
│   ├── services/         # Business logic layer
│   │   ├── mod.rs
│   │   ├── library_service.rs     # Orchestrates multiple repos
│   │   ├── metadata_service.rs    # Metadata override logic
│   │   ├── scanner_service.rs     # File scanning
│   │   ├── streaming_service.rs   # File streaming
│   │   ├── search_service.rs      # Tantivy integration
│   │   ├── auth_service.rs        # Authentication
│   │   └── transcode_service.rs   # FFmpeg integration
│   │
│   ├── handlers/         # HTTP handlers (controllers in MVC)
│   │   ├── mod.rs
│   │   ├── tracks.rs     # Track-related endpoints
│   │   ├── albums.rs
│   │   ├── artists.rs
│   │   ├── tags.rs
│   │   ├── playlists.rs
│   │   ├── auth.rs
│   │   ├── admin.rs      # Admin endpoints (scanning, etc.)
│   │   └── stream.rs     # Streaming endpoint
│   │
│   ├── routes/           # Route definitions
│   │   ├── mod.rs        # Combines all routes
│   │   └── api.rs        # API route builder
│   │
│   ├── middleware/       # Custom middleware
│   │   ├── mod.rs
│   │   ├── auth.rs       # Auth validation middleware
│   │   ├── logging.rs    # Request/response logging
│   │   └── cors.rs       # CORS setup
│   │
│   ├── dto/              # Data Transfer Objects
│   │   ├── mod.rs
│   │   ├── request/      # Request DTOs
│   │   │   ├── mod.rs
│   │   │   ├── track_update.rs
│   │   │   └── playlist_generate.rs
│   │   └── response/     # Response DTOs
│   │       ├── mod.rs
│   │       ├── track_response.rs
│   │       └── paginated.rs
│   │
│   ├── extractors/       # Custom axum extractors
│   │   ├── mod.rs
│   │   ├── auth.rs       # CurrentUser extractor
│   │   └── pagination.rs # Pagination params
│   │
│   ├── scanner/          # Media file scanning
│   │   ├── mod.rs
│   │   ├── file_walker.rs
│   │   ├── metadata_extractor.rs
│   │   └── duration_analyzer.rs
│   │
│   ├── streaming/        # HTTP streaming
│   │   ├── mod.rs
│   │   └── range_parser.rs
│   │
│   ├── search/           # Tantivy search
│   │   ├── mod.rs
│   │   ├── index.rs      # Index management
│   │   └── query.rs      # Query builder
│   │
│   ├── jobs/             # Background jobs
│   │   ├── mod.rs
│   │   ├── scanner_job.rs
│   │   └── cleanup_job.rs
│   │
│   └── utils/            # Utility functions
│       ├── mod.rs
│       ├── hash.rs       # File hashing
│       └── paths.rs      # Path manipulation
│
├── tests/                # Integration tests
│   ├── common/
│   │   └── mod.rs        # Test helpers
│   ├── api/
│   │   ├── tracks_test.rs
│   │   └── auth_test.rs
│   └── streaming_test.rs
│
└── benches/              # Performance benchmarks
    └── scan_bench.rs
```

## Key Differences from NestJS Structure

### Rust/Axum Conventions:
1. **Layer-based, not feature-based**: Group by technical role (handlers, services, repositories) rather than by domain
2. **Separation of concerns**:
   - `db/entities/` - Pure structs that map 1:1 with database tables
   - `models/` - Domain models with business logic
   - `dto/` - HTTP request/response shapes
3. **Explicit data flow**: handler → service → repository → database
4. **No decorators**: Route definitions are explicit in `routes/`
5. **Extractors instead of pipes**: Custom extractors for request parsing

### Module Organization Example:

```rust
// src/handlers/tracks.rs
use axum::{extract::{Path, Query, State}, Json};
use crate::{
    dto::{request::TrackUpdate, response::TrackResponse},
    services::LibraryService,
    extractors::CurrentUser,
    error::AppError,
    state::AppState,
};

pub async fn get_track(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    user: CurrentUser,  // Custom extractor
) -> Result<Json<TrackResponse>, AppError> {
    let service = LibraryService::new(state.db.clone());
    let track = service.get_track_with_metadata(id, user.id).await?;
    Ok(Json(track.into()))
}

pub async fn update_track_metadata(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    user: CurrentUser,
    Json(update): Json<TrackUpdate>,
) -> Result<Json<TrackResponse>, AppError> {
    let service = LibraryService::new(state.db.clone());
    let track = service.update_metadata(id, update, user.id).await?;
    Ok(Json(track.into()))
}
```

```rust
// src/routes/api.rs
use axum::{routing::{get, patch}, Router};
use crate::handlers::tracks;
use crate::state::AppState;

pub fn tracks_routes() -> Router<AppState> {
    Router::new()
        .route("/tracks/:id", get(tracks::get_track))
        .route("/tracks/:id/metadata", patch(tracks::update_track_metadata))
}
```

```rust
// src/routes/mod.rs
use axum::Router;
use crate::state::AppState;

pub fn create_api_router() -> Router<AppState> {
    Router::new()
        .nest("/api", api_routes())
}

fn api_routes() -> Router<AppState> {
    Router::new()
        .merge(tracks_routes())
        .merge(albums_routes())
        .merge(auth_routes())
        // etc...
}
```

## Phase 0: Foundation Setup

### 0.1 Project Structure
- [x] Create `apps/server/` directory
- [x] Initialize Cargo project: `cargo init --name amaterasu-server`
- [x] Set up workspace in root `Cargo.toml`:
  ```toml
  [workspace]
  members = ["apps/server"]
  resolver = "2"
  ```
- [x] Create the folder structure above

### 0.2 Core Dependencies
- [x] Add to `Cargo.toml`:
  ```toml
  [dependencies]
  # Web framework
  axum = "0.7"
  tokio = { version = "1", features = ["full"] }
  tower = "0.4"
  tower-http = { version = "0.5", features = ["trace", "cors"] }
  
  # Database
  sqlx = { version = "0.8", features = ["runtime-tokio", "postgres", "uuid", "time"] }
  
  # Serialization
  serde = { version = "1", features = ["derive"] }
  serde_json = "1"
  
  # Logging
  tracing = "0.1"
  tracing-subscriber = { version = "0.3", features = ["env-filter"] }
  
  # Config
  dotenvy = "0.15"
  
  # Utils
  uuid = { version = "1", features = ["v4", "serde"] }
  anyhow = "1"
  thiserror = "1"
  ```

### 0.3 Basic Server Setup
- [x] Create `src/main.rs` with minimal axum server
- [x] Add health check endpoint at `/health`
- [x] Set up tracing/logging
- [x] Environment variable loading with dotenvy
- [x] Graceful shutdown handling

### 0.4 Project Structure
- [x] Create module structure:
  ```
  src/
  ├── main.rs
  ├── config.rs         # App configuration
  ├── error.rs          # Error types
  ├── db/
  │   └── mod.rs        # Database pool setup
  ├── routes/
  │   ├── mod.rs
  │   └── health.rs
  └── state.rs          # App state
  ```

### 0.5 Database Setup
- [x] Create `migrations/` directory
- [x] Install sqlx-cli: `cargo install sqlx-cli --no-default-features --features postgres`
- [x] Create `.env` file with `DATABASE_URL`
- [x] Create docker-compose.yml with Postgres
- [x] Initialize SQLx: `sqlx database create`
- [x] Test connection pool in main.rs

### 0.6 Error Handling
- [x] Define custom error type with `thiserror`
- [x] Implement `IntoResponse` for error type
- [x] Add error context with `anyhow`

**Learning Tips:**
- Study axum examples repo for patterns
- Use `cargo watch -x run` for auto-reload
- Enable `RUST_LOG=debug` for detailed logs
- Test each endpoint with `curl` or `httpie`

---

## Phase 1: Core Models & Migrations

### 1.1 Create Base Migrations
- [ ] Migration 001: Create users table
- [ ] Migration 002: Create artists, albums, tracks tables
- [ ] Migration 003: Create track_metadata table
- [ ] Migration 004: Create tags and track_tags tables
- [ ] Migration 005: Create playlists table
- [ ] Migration 006: Create sessions table
- [ ] Run migrations: `sqlx migrate run`

### 1.2 Domain Models
- [ ] Create `src/models/` directory
- [ ] Define structs matching DB schema:
  - [ ] `user.rs` - User, Session
  - [ ] `track.rs` - Track, TrackWithMetadata (merged view)
  - [ ] `album.rs` - Album
  - [ ] `artist.rs` - Artist
  - [ ] `tag.rs` - Tag, TrackTag
  - [ ] `playlist.rs` - Playlist
- [ ] Implement `FromRow` for SQLx
- [ ] Add builder patterns where useful

### 1.3 Repository Pattern
- [ ] Create `src/repositories/` directory
- [ ] Implement repositories:
  - [ ] `track_repo.rs` - CRUD + metadata merging logic
  - [ ] `album_repo.rs`
  - [ ] `artist_repo.rs`
  - [ ] `tag_repo.rs`
- [ ] Use SQLx query macros for compile-time checking
- [ ] Handle the dual metadata system in track_repo

### 1.4 Service Layer
- [ ] Create `src/services/` directory
- [ ] Implement business logic:
  - [ ] `library_service.rs` - Coordinate track/album/artist operations
  - [ ] `metadata_service.rs` - Handle override logic
- [ ] Add transaction support where needed

**Learning Tips:**
- Use `sqlx::query!` for compile-time SQL checking
- Study SQLx examples for complex queries
- Consider using `sqlx-cli` to generate schema cache

---

## Phase 2: File Scanning

### 2.1 Media Dependencies
- [ ] Add to Cargo.toml:
  ```toml
  # Media processing
  symphonia = { version = "0.5", features = ["all"] }
  lofty = "0.21"
  
  # File watching
  notify = "6"
  walkdir = "2"
  
  # Async
  tokio-stream = "0.1"
  ```

### 2.2 Scanner Module
- [ ] Create `src/scanner/` directory
- [ ] `file_walker.rs` - Recursive directory walking
  - [ ] Support include/exclude patterns
  - [ ] Filter by audio extensions
- [ ] `metadata_extractor.rs` - Extract tags
  - [ ] Try symphonia first
  - [ ] Fallback to lofty
  - [ ] Extract: title, artist, album, track_no, disc, year
- [ ] `duration_analyzer.rs` - Get accurate duration
  - [ ] Use symphonia to decode headers
  - [ ] Calculate duration from frames/sample rate

### 2.3 Scanner Service
- [ ] Create `src/services/scanner_service.rs`
- [ ] Implement scanning workflow:
  1. Walk directories
  2. Extract metadata
  3. Hash file for change detection
  4. Upsert to database
  5. Track scan progress
- [ ] Handle errors gracefully (corrupted files, etc.)
- [ ] Add progress reporting

### 2.4 Scanner API Endpoint
- [ ] `POST /api/admin/scan` - Trigger library scan
- [ ] `GET /api/admin/scan/status` - Get scan progress
- [ ] Add to router in main.rs

**Learning Tips:**
- Symphonia examples are excellent for audio processing
- Use `walkdir` with `.filter_entry()` for efficiency
- Consider chunking DB inserts for performance

---

## Phase 3: Streaming

### 3.1 Range Request Support
- [ ] Create `src/streaming/` directory
- [ ] `range_parser.rs` - Parse Range headers
  - [ ] Handle single ranges: `bytes=0-1023`
  - [ ] Handle suffix ranges: `bytes=-500`
  - [ ] Handle open ranges: `bytes=1024-`
- [ ] Return 416 for invalid ranges

### 3.2 Streaming Service
- [ ] `src/services/streaming_service.rs`
- [ ] Implement file streaming:
  - [ ] Open file with tokio::fs
  - [ ] Seek to range start
  - [ ] Stream chunks with correct headers
  - [ ] Set Content-Length, Content-Range, Accept-Ranges
  - [ ] Return 206 Partial Content for ranges
  - [ ] Return 200 OK for full file

### 3.3 Streaming Endpoint
- [ ] `GET /api/tracks/:id/stream`
- [ ] Look up file path from DB
- [ ] Check file exists
- [ ] Handle Range header if present
- [ ] Stream file with proper mime type

**Learning Tips:**
- Study HTTP range request RFC 7233
- Test with curl: `curl -H "Range: bytes=0-1023"`
- Use browser DevTools to verify 206 responses

---

## Phase 4: Background Jobs

### 4.1 Job Infrastructure
- [ ] Add tokio-cron to dependencies
- [ ] Create `src/jobs/` directory
- [ ] `job_runner.rs` - Main job scheduler
- [ ] Define job trait/interface

### 4.2 Implement Jobs
- [ ] `scan_job.rs` - Periodic library scan
- [ ] `cleanup_job.rs` - Remove orphaned entries
- [ ] Future: `transcode_job.rs`, `thumbnail_job.rs`

### 4.3 Job Scheduling
- [ ] Initialize scheduler in main.rs
- [ ] Configure cron expressions
- [ ] Add job status tracking

---

## Phase 5: API Development

### 5.1 OpenAPI Setup
- [ ] Add utoipa dependencies:
  ```toml
  utoipa = { version = "4", features = ["axum_extras"] }
  utoipa-swagger-ui = { version = "6", features = ["axum"] }
  ```
- [ ] Add OpenAPI annotations to structs
- [ ] Document endpoints with utoipa macros
- [ ] Serve OpenAPI spec at `/api/openapi.json`
- [ ] Serve Swagger UI at `/api/docs`

### 5.2 Core CRUD Endpoints
- [ ] Track endpoints:
  - [ ] `GET /api/tracks` - List with pagination
  - [ ] `GET /api/tracks/:id` - Get with merged metadata
  - [ ] `PATCH /api/tracks/:id/metadata` - Update overrides
  - [ ] `DELETE /api/tracks/:id/metadata` - Reset to original
  
- [ ] Tag endpoints:
  - [ ] `GET /api/tags` - List all tags
  - [ ] `POST /api/tags` - Create tag
  - [ ] `POST /api/tracks/:id/tags` - Add tag to track
  - [ ] `DELETE /api/tracks/:id/tags/:tagId` - Remove tag

- [ ] Album/Artist endpoints:
  - [ ] `GET /api/albums` - List albums
  - [ ] `GET /api/albums/:id` - Get album with tracks
  - [ ] `GET /api/artists` - List artists
  - [ ] `GET /api/artists/:id` - Get artist with albums

### 5.3 Advanced Features
- [ ] Search endpoint:
  - [ ] `GET /api/search?q=...` - Search tracks/albums/artists
  
- [ ] Dynamic playlist generation:
  - [ ] `POST /api/playlists/generate` - Generate from tags

### 5.4 Request/Response DTOs
- [ ] Create `src/dto/` directory
- [ ] Define separate DTOs from domain models
- [ ] Add validation with `validator` crate
- [ ] Implement conversion traits

---

## Phase 6: Search with Tantivy

### 6.1 Tantivy Setup
- [ ] Add tantivy to dependencies
- [ ] Create `src/search/` directory
- [ ] Design index schema (fields for artist, album, track, tags)
- [ ] Initialize index on startup

### 6.2 Indexing
- [ ] Index tracks after scanning
- [ ] Update index on metadata changes
- [ ] Handle index rebuilding

### 6.3 Search Implementation
- [ ] Query parser for user searches
- [ ] Support fuzzy matching
- [ ] Implement faceted search by tags

---

## Phase 7: Authentication

### 7.1 Password Hashing
- [ ] Add argon2 dependency
- [ ] Implement password hashing/verification
- [ ] Create registration endpoint
- [ ] Create login endpoint

### 7.2 Session Management
- [ ] Generate secure session tokens
- [ ] Store sessions in DB with expiry
- [ ] Create auth middleware
- [ ] Add logout endpoint

### 7.3 Authorization
- [ ] Add role checks (admin vs user)
- [ ] Protect admin endpoints
- [ ] Add per-user library filtering

---

## Phase 8: Transcoding (FFmpeg)

### 8.1 FFmpeg Integration
- [ ] Check FFmpeg availability on startup
- [ ] Create `src/transcoding/` directory
- [ ] Build FFmpeg command builder
- [ ] Handle process spawning with tokio

### 8.2 Transcode Profiles
- [ ] Define profile struct (format, bitrate, etc.)
- [ ] Support opus, mp3, aac outputs
- [ ] Add quality presets

### 8.3 Transcode Cache
- [ ] Cache transcoded files to disk
- [ ] Implement cache eviction (LRU)
- [ ] Serve from cache when available

---

## Testing Strategy

### Unit Tests
- [ ] Test metadata extraction
- [ ] Test range parsing
- [ ] Test tag query building
- [ ] Test auth logic

### Integration Tests
- [ ] Test full scan workflow
- [ ] Test streaming with various range requests
- [ ] Test metadata override system
- [ ] Test dynamic playlist generation

### E2E Tests
- [ ] Set up test database
- [ ] Test complete user flows
- [ ] Verify API contract

---

## Development Tools

### Makefile/Justfile Commands
```makefile
# Database
db-create:
    sqlx database create

db-migrate:
    sqlx migrate run

db-reset:
    sqlx database drop -y
    sqlx database create
    sqlx migrate run

# Development
dev:
    cargo watch -x run

test:
    cargo test

check:
    cargo check
    cargo clippy
    cargo fmt --check

# Docker
docker-up:
    docker-compose up -d

docker-down:
    docker-compose down
```

### Useful Cargo Commands
```bash
# Check for outdated dependencies
cargo outdated

# Security audit
cargo audit

# Generate SQLx offline schema
cargo sqlx prepare

# Format code
cargo fmt

# Lint code
cargo clippy -- -W clippy::pedantic
```

---

## Learning Resources

### Rust + Axum
- [Axum examples](https://github.com/tokio-rs/axum/tree/main/examples)
- [Zero to Production in Rust](https://www.zero2prod.com/) - Excellent book
- [Rust Async Book](https://rust-lang.github.io/async-book/)

### SQLx
- [SQLx GitHub examples](https://github.com/launchbadge/sqlx/tree/main/examples)
- [SQLx book](https://github.com/launchbadge/sqlx/blob/main/sqlx-cli/README.md)

### Audio Processing
- [Symphonia docs](https://docs.rs/symphonia/latest/symphonia/)
- [Lofty examples](https://github.com/Serial-ATA/lofty-rs/tree/main/examples)

### Best Practices
- Use `#[derive(Debug, Clone)]` liberally
- Prefer `Arc<T>` for shared state in async
- Use `tracing` spans for better debugging
- Write tests as you go
- Commit after each working feature

---

## Progress Tracking

Mark items as complete as you implement them. If you get stuck on any step, we can dive deeper into that specific area. Remember:

1. **Start small** - Get a basic endpoint working first
2. **Test often** - Use curl/httpie to verify each endpoint
3. **Read errors carefully** - Rust compiler is your friend
4. **Ask for help** - Come back here when you need clarification

The goal is to learn, not to rush. Take your time with each component and understand how it works!
