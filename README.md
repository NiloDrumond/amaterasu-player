# Amaterasu Player

A self-hosted music player and library manager. Scans your local music collection, enriches metadata via MusicBrainz, and serves a web UI for browsing and playback.

## Quick start

1. Clone the repository
2. Edit `docker-compose.yml`:
   - Set the path to your music library (replace `/path/to/your/music`)
   - Change the admin credentials (`ADMIN_EMAIL`, `ADMIN_PASSWORD`)
   - Change the database password (update both the `amaterasu` service `DATABASE_URL` and the `db` service `POSTGRES_PASSWORD`)
3. Start the stack:
   ```bash
   docker compose up -d
   ```
4. Open http://localhost:4534

## Configuration

All configuration is done through environment variables in `docker-compose.yml`.

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

To enable automatic metadata enrichment from MusicBrainz, uncomment these lines in `docker-compose.yml`:

```yaml
MUSICBRAINZ_ENABLED: true
MUSICBRAINZ_USER_AGENT: "amaterasu/1.0 ( mailto:you@example.com )"
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
