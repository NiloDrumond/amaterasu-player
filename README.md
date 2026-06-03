# Amaterasu Player

A self-hosted music player and library manager. Scans your local music collection, enriches metadata via MusicBrainz, and serves a web UI for browsing and playback.

## Quick start

No need to clone the repo — the image is published to Docker Hub. In an empty directory:

1. Download the compose file and the example environment file:
   ```bash
   curl -O https://codeberg.org/NiloDrumond/amaterasu-player/raw/branch/main/docker-compose.yml
   curl -o .env https://codeberg.org/NiloDrumond/amaterasu-player/raw/branch/main/.env.example
   ```
2. Edit `.env`:
   - `MUSIC_PATH` — absolute path to your music library
   - `ADMIN_EMAIL` / `ADMIN_PASSWORD` — initial admin user
   - `POSTGRES_PASSWORD` — database password (used by both services)
3. Start the stack:
   ```bash
   docker compose up -d
   ```
4. Open http://localhost:4534

The compose file pulls `nilodrumond/amaterasu-player:latest`. To pin a specific release,
change the tag to a version, e.g. `nilodrumond/amaterasu-player:1.0.0`.

### Building from source

To build the image locally instead of pulling it:

```bash
docker compose -f docker-compose.yml -f docker-compose.build.yml up -d --build
```

## Configuration

All configuration is done through environment variables in `.env` (see `.env.example`).

### App settings

| Variable | Description | Default |
|----------|-------------|---------|
| `DATABASE_URL` | PostgreSQL connection string | `postgres://amaterasu:amaterasu@db:5432/amaterasu` |
| `ADMIN_EMAIL` | Bootstrap admin email (first startup only) | `admin@example.com` |
| `ADMIN_PASSWORD` | Bootstrap admin password (first startup only) | `changeme` |
| `MUSICBRAINZ_ENABLED` | Enable MusicBrainz metadata enrichment | `false` |
| `MUSICBRAINZ_USER_AGENT` | Required by MusicBrainz API policy when enabled | — |
| `TRUST_PROXY_HEADERS` | Trust `X-Forwarded-*` headers (for reverse proxies) | `true` |

### Volumes

| Container path | Purpose |
|----------------|---------|
| `/data` | Covers, search index, logs — persisted across restarts |
| `/music` | Music library (mounted read-only) |

## Supported audio formats

MP3, FLAC, WAV, OGG, OGA, Opus, M4A, M4B, AAC, WMA, AIFF, AIF, ALAC

## Architecture

The container runs three processes behind an internal nginx reverse proxy:

- **nginx** (port 3000, exposed as 4534) — routes `/api/*` and `/health` to the Rust server, everything else to SvelteKit
- **Rust server** (internal port 8080) — API, audio streaming, library scanning, Grafana proxy
- **SvelteKit + Bun** (internal port 3001) — server-side rendered frontend

## MusicBrainz integration

To enable automatic metadata enrichment from MusicBrainz, set these in `.env`:

```bash
MUSICBRAINZ_ENABLED=true
MUSICBRAINZ_USER_AGENT=amaterasu-player/1.0 ( mailto:you@example.com )
```

The user agent string is required by [MusicBrainz API policy](https://musicbrainz.org/doc/MusicBrainz_API#User_agent). Use a real contact email.

## Monitoring (optional)

Add Loki + Grafana for structured logging and dashboards:

```bash
docker compose -f docker-compose.yml -f docker-compose.monitoring.yml up -d
```

Grafana is accessible at http://localhost:4534/admin/logs (proxied through the app, requires admin login). It is not directly exposed — only reachable through the app's reverse proxy.

### Monitoring settings

| Variable | Where | Description |
|----------|-------|-------------|
| `GF_SECURITY_ADMIN_PASSWORD` | `.env` | Grafana admin password (defaults to `admin`) |
| `GRAFANA_ROOT_URL` | `docker-compose.monitoring.yml` | Set to your public URL if not localhost |

## Reverse proxy

When running behind nginx, Caddy, Traefik, or similar, the app trusts `X-Forwarded-*` headers by default (`TRUST_PROXY_HEADERS=true`).

## Database

- PostgreSQL 16 runs as a separate container
- Migrations run automatically on server startup
- Data is persisted in a Docker volume (`db_data`)
