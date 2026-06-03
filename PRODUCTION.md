# Production deployment

## Quick start

1. Copy the `docker-compose.yml` from the [README](README.md#using-docker-compose-recommended) into a directory (no clone needed)
2. Edit it: music library path, admin credentials, and database password (in both services)
3. `docker compose up -d`
4. Open http://localhost:4534

## Configuration

All configuration is done through environment variables on the `amaterasu` service. Key settings:

| Variable | Description |
|----------|-------------|
| `DATABASE_URL` | PostgreSQL connection string (match the password with the `db` service) |
| `ADMIN_EMAIL` | Bootstrap admin email (only used on first startup) |
| `ADMIN_PASSWORD` | Bootstrap admin password (only used on first startup) |
| `MUSICBRAINZ_ENABLED` | Enable MusicBrainz metadata enrichment (default: `false`) |
| `MUSICBRAINZ_USER_AGENT` | Required by MusicBrainz API policy when enabled |

The compose file pulls `nilodrumond/amaterasu-player:latest`. Pin a release by changing the
tag, e.g. `nilodrumond/amaterasu-player:1.0.0`.

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

- Set `GF_SECURITY_ADMIN_PASSWORD` on the `grafana` service in `docker-compose.monitoring.yml` (defaults to `admin`)
- Set `GF_SERVER_ROOT_URL` to your public URL, e.g. `https://music.example.com/admin/logs/`
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

## Releasing

Images are built and published to Docker Hub (`nilodrumond/amaterasu-player`) by GitHub
Actions (`.github/workflows/docker-publish.yml`). The source lives on Codeberg and is
push-mirrored to GitHub, which is what triggers the workflow.

- **Latest:** every push to `main` builds and pushes `:latest` (multi-arch: amd64 + arm64).
- **Versioned release:** create and push a semver tag — the mirror forwards it to GitHub and
  the workflow publishes `:1.0.0`, `:1.0`, `:1`, and `:latest`:
  ```bash
  git tag v1.0.0
  git push origin v1.0.0
  ```

One-time setup:

- Create a **public** GitHub repo and configure a **Push Mirror** on Codeberg
  (Settings → Repository → Push Mirror) targeting it with a GitHub PAT. A public repo gives
  free GitHub-hosted arm64 runners (`ubuntu-24.04-arm`); without them the workflow would fall
  back to slow QEMU emulation for arm64.
- Add repo secrets on GitHub: `DOCKERHUB_USERNAME` and `DOCKERHUB_TOKEN` (a Docker Hub access
  token, not the account password).
