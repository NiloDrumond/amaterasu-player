# Stage 1: Build Rust binary
FROM rust:1-bookworm AS rust-builder

RUN apt-get update && apt-get install -y \
    clang \
    libavcodec-dev \
    libavformat-dev \
    libavutil-dev \
    libavfilter-dev \
    libavdevice-dev \
    libswscale-dev \
    libswresample-dev \
    pkg-config \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Dependency caching: copy manifests, build deps with dummy sources
COPY Cargo.toml Cargo.lock ./
COPY apps/server/Cargo.toml apps/server/Cargo.toml
COPY libs/macros/Cargo.toml libs/macros/Cargo.toml
COPY tools/dto-lint/Cargo.toml tools/dto-lint/Cargo.toml

RUN mkdir -p apps/server/src libs/macros/src tools/dto-lint/src \
    && echo "fn main() {}" > apps/server/src/main.rs \
    && touch libs/macros/src/lib.rs \
    && echo "fn main() {}" > tools/dto-lint/src/main.rs \
    && cargo build --release -p amaterasu-server 2>/dev/null || true \
    && rm -rf apps/ libs/ tools/

# Build the actual server
COPY .sqlx .sqlx
COPY apps/server apps/server
COPY libs libs
COPY tools tools

ENV SQLX_OFFLINE=true
RUN touch apps/server/src/main.rs libs/macros/src/lib.rs && cargo build --release -p amaterasu-server

# Stage 2: Build SvelteKit app
FROM oven/bun:1 AS web-builder

WORKDIR /app
COPY apps/web/package.json apps/web/bun.lock ./
RUN bun install --frozen-lockfile

COPY apps/web .
RUN bun run build

# Stage 3: Runtime
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    curl \
    nginx \
    libavcodec59 \
    libavdevice59 \
    libavformat59 \
    libavutil57 \
    libswresample4 \
    libswscale6 \
    && rm -rf /var/lib/apt/lists/*

COPY --from=web-builder /usr/local/bin/bun /usr/local/bin/bun
RUN ln -s /usr/local/bin/bun /usr/local/bin/bunx

RUN mkdir -p /data /music /run/nginx

COPY --from=rust-builder /app/target/release/amaterasu-server /usr/local/bin/amaterasu-server
COPY --from=web-builder /app/build /app/web

COPY infra/docker/nginx.conf /etc/nginx/nginx.conf
COPY infra/docker/entrypoint.sh /entrypoint.sh
RUN chmod +x /entrypoint.sh

ENV SERVER_HOST=127.0.0.1
ENV SERVER_PORT=8080
ENV DATA_DIR=/data
ENV LOG_DIR=/data/logs
ENV LIBRARY_PATH=/music
ENV TRUST_PROXY_HEADERS=true
ENV MUSICBRAINZ_ENABLED=false
ENV ORIGIN=http://127.0.0.1:3000

EXPOSE 3000
VOLUME ["/data", "/music"]

HEALTHCHECK --interval=30s --timeout=5s --start-period=60s --retries=3 \
    CMD curl -sf http://127.0.0.1:3000/health || exit 1

ENTRYPOINT ["/entrypoint.sh"]
