# Production deployment

## Quick start

1. Edit `docker-compose.yml`: set your music library path, admin credentials, and database password
2. `docker compose up -d`
3. Open http://localhost:4534

## Configuration

Edit `docker-compose.yml` directly. Key settings in the `amaterasu` service:

| Variable | Description |
|----------|-------------|
| `DATABASE_URL` | PostgreSQL connection string (match password with `db` service) |
| `ADMIN_EMAIL` | Bootstrap admin email (only used on first startup) |
| `ADMIN_PASSWORD` | Bootstrap admin password (only used on first startup) |
| `MUSICBRAINZ_ENABLED` | Enable MusicBrainz metadata enrichment (default: `false`) |
| `MUSICBRAINZ_USER_AGENT` | Required by MusicBrainz API policy when enabled |

Under `volumes`, change `/path/to/your/music` to your music library directory.

## Volumes

| Container path | Purpose |
|----------------|---------|
| `/data` | Covers, search index, logs — persisted across restarts |
| `/music` | Music library (mounted read-only) |

## Monitoring (optional)

Add Loki + Grafana for structured logging and dashboards:

```bash
docker compose -f docker-compose.yml -f docker-compose.monitoring.yml up -d
```

Grafana is accessible at `http://localhost:4534/admin/logs` (proxied through the app, requires admin login).

### Monitoring hardening

- Set `GF_SECURITY_ADMIN_PASSWORD` in `.env` (defaults to `admin`)
- Set `GRAFANA_ROOT_URL` to your public URL, e.g. `https://music.example.com/admin/logs/`
- Grafana is not directly exposed — it's only reachable via the app's reverse proxy

## Reverse proxy

If running behind a reverse proxy (nginx, Caddy, Traefik), the app trusts `X-Forwarded-*` headers by default in Docker (`TRUST_PROXY_HEADERS=true`).

## Database

- PostgreSQL 16 runs as a separate container
- Migrations run automatically on server startup
- Database data is persisted in a Docker volume (`db_data`)

## Architecture

The `amaterasu` container runs three processes managed by an entrypoint script:

- **nginx** (port 3000): Routes `/api/*` and `/health` to the Rust server, everything else to SvelteKit
- **Rust server** (internal port 8080): API, audio streaming, Grafana proxy
- **SvelteKit/Bun** (internal port 3001): Server-side rendered frontend

## Known limitations

- The `/admin/logs` reverse proxy forwards HTTP only. Grafana Live (websockets) will not work through it.
