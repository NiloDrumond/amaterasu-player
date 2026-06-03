# Amaterasu Player

A self-hosted music player and library manager. Scans your local music collection, enriches metadata via MusicBrainz, and serves a web UI for browsing and playback.

## Installation with Docker

The image is published to Docker Hub as [`nilodrumond/amaterasu-player`](https://hub.docker.com/r/nilodrumond/amaterasu-player). Amaterasu needs a PostgreSQL database; the compose file below runs one alongside the app, so this is all you need — no need to clone the repository.

### Using docker compose (recommended)

Create a `docker-compose.yml` file:

```yaml
services:
  amaterasu:
    image: nilodrumond/amaterasu-player:latest
    ports:
      - "4534:3000"
    restart: unless-stopped
    environment:
      # Keep this password in sync with POSTGRES_PASSWORD on the db service below
      DATABASE_URL: postgres://amaterasu:CHANGE_ME_DB_PASSWORD@db:5432/amaterasu
      # Initial admin user — created on first start only
      ADMIN_EMAIL: admin@example.com
      ADMIN_PASSWORD: CHANGE_ME_ADMIN_PASSWORD
      # Optional: MusicBrainz metadata enrichment (a real contact is required)
      # MUSICBRAINZ_ENABLED: "true"
      # MUSICBRAINZ_USER_AGENT: "amaterasu-player/1.0 ( mailto:you@example.com )"
    volumes:
      - data:/data
      - /path/to/your/music:/music:ro # change to your music library
    depends_on:
      db:
        condition: service_healthy

  db:
    image: postgres:16-alpine
    restart: unless-stopped
    environment:
      POSTGRES_DB: amaterasu
      POSTGRES_USER: amaterasu
      POSTGRES_PASSWORD: CHANGE_ME_DB_PASSWORD
    volumes:
      - db_data:/var/lib/postgresql/data
    healthcheck:
      test: ["CMD-SHELL", "pg_isready -U amaterasu"]
      interval: 5s
      timeout: 5s
      retries: 10

volumes:
  data:
  db_data:
```

Before starting, edit:

- the **music library path** — replace `/path/to/your/music`
- the **admin credentials** — `ADMIN_EMAIL` and `ADMIN_PASSWORD`
- the **database password** — replace `CHANGE_ME_DB_PASSWORD` in **both** the `amaterasu` and `db` services

Then start the stack and open <http://localhost:4534>:

```bash
docker compose up -d
```

> To pin a specific release instead of tracking the latest build, replace `:latest` with a version tag, e.g. `nilodrumond/amaterasu-player:1.0.0`.

### Using the docker CLI

`docker compose` is recommended because Amaterasu needs a PostgreSQL database. If you already run your own Postgres, you can start just the app container and point it at your existing database:

```bash
docker run -d --name amaterasu \
  -p 4534:3000 \
  -e DATABASE_URL="postgres://user:password@host:5432/amaterasu" \
  -e ADMIN_EMAIL="admin@example.com" \
  -e ADMIN_PASSWORD="changeme" \
  -v amaterasu_data:/data \
  -v /path/to/your/music:/music:ro \
  --restart unless-stopped \
  nilodrumond/amaterasu-player:latest
```

### Building from source

To build the image locally instead of pulling it, clone the repository and run:

```bash
docker compose -f docker-compose.yml -f docker-compose.build.yml up -d --build
```

## Customization

Configuration is done through environment variables on the `amaterasu` service.

### Environment variables

| Variable | Description | Default |
|----------|-------------|---------|
| `DATABASE_URL` | PostgreSQL connection string | — (required) |
| `ADMIN_EMAIL` | Bootstrap admin email (first start only) | `admin@example.com` |
| `ADMIN_PASSWORD` | Bootstrap admin password (first start only) | `changeme` |
| `MUSICBRAINZ_ENABLED` | Enable MusicBrainz metadata enrichment | `false` |
| `MUSICBRAINZ_USER_AGENT` | Required by MusicBrainz API policy when enabled | — |
| `TRUST_PROXY_HEADERS` | Trust `X-Forwarded-*` headers (for reverse proxies) | `true` |

### Volumes

| Container path | Purpose |
|----------------|---------|
| `/data` | Covers, search index, logs — persisted across restarts |
| `/music` | Your music library — mount it read-only (`:ro`) |

### Ports

The container listens on port **3000**. The compose example maps it to **4534** on the host (`"4534:3000"`); change the left-hand number to expose it on a different port.

## MusicBrainz integration

To enable automatic metadata enrichment from MusicBrainz, uncomment these in the `amaterasu` service `environment`:

```yaml
MUSICBRAINZ_ENABLED: "true"
MUSICBRAINZ_USER_AGENT: "amaterasu-player/1.0 ( mailto:you@example.com )"
```

The user agent string is required by [MusicBrainz API policy](https://musicbrainz.org/doc/MusicBrainz_API#User_agent). Use a real contact email.

## Reverse proxy

When running behind nginx, Caddy, Traefik, or similar, the app trusts `X-Forwarded-*` headers by default (`TRUST_PROXY_HEADERS=true`).

## Monitoring (optional)

Add Loki + Grafana for structured logging and dashboards. This overlay mounts config from the repo's `infra/` directory, so it requires a clone:

```bash
docker compose -f docker-compose.yml -f docker-compose.monitoring.yml up -d
```

Grafana is accessible at <http://localhost:4534/admin/logs> (proxied through the app, requires admin login). It is not directly exposed — only reachable through the app's reverse proxy.

| Variable | Where | Description |
|----------|-------|-------------|
| `GF_SECURITY_ADMIN_PASSWORD` | `docker-compose.monitoring.yml` (grafana service) | Grafana admin password (defaults to `admin`) |
| `GF_SERVER_ROOT_URL` | `docker-compose.monitoring.yml` (grafana service) | Set to your public URL if not localhost |

## Supported audio formats

MP3, FLAC, WAV, OGG, OGA, Opus, M4A, M4B, AAC, WMA, AIFF, AIF, ALAC

## Architecture

The container runs three processes behind an internal nginx reverse proxy:

- **nginx** (port 3000, exposed as 4534) — routes `/api/*` and `/health` to the Rust server, everything else to SvelteKit
- **Rust server** (internal port 8080) — API, audio streaming, library scanning, Grafana proxy
- **SvelteKit + Bun** (internal port 3001) — server-side rendered frontend

## Database

- PostgreSQL 16 runs as a separate container
- Migrations run automatically on server startup
- Data is persisted in a Docker volume (`db_data`)
