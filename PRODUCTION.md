# Production checklist

Items below must be addressed before deploying. They are no-ops in local dev but become security/correctness issues in production.

## Logs / Grafana

- **Drop the `3001:3000` port mapping on the `grafana` service** in `infra/docker-compose.yml` so Grafana is only reachable through the amaterasu server's `/admin/logs` reverse proxy. If Grafana stays directly exposed, anyone who can reach it can forge the `X-WEBAUTH-USER` header and walk in as admin.
- **Set `GRAFANA_ROOT_URL`** in the deploy environment to the public URL, e.g. `https://music.example.com/admin/logs/`. This is read by docker-compose into `GF_SERVER_ROOT_URL`.
- **Set `GRAFANA_URL`** in the server's environment to the internal address Grafana is reachable at from the server process. Once the server is also containerized, this is `http://grafana:3000`; while the server still runs on the host, keep `http://localhost:3001`.
- **Optional hardening** (recommended once direct login is no longer needed):
  - `GF_AUTH_DISABLE_LOGIN_FORM=true`
  - `GF_AUTH_BASIC_ENABLED=false`
  - `GF_AUTH_PROXY_WHITELIST=<server container IP / CIDR>` — restricts which upstreams Grafana will trust for the auth header.
- **Set a real `GF_SECURITY_ADMIN_PASSWORD`** in the deploy environment. The compose file falls back to `admin` if unset.

## Known limitations

- The `/admin/logs` reverse proxy in `apps/server` forwards HTTP only. **Grafana Live (websockets) will not work through it.** Basic Explore, dashboards, and ad-hoc queries are unaffected. Add a websocket upgrade path if/when live-tail is needed.
