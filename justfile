# Development commands for Amaterasu Player

default:
    @just --list

dev:
    docker compose -f infra/docker-compose.yml up -d
    cd apps/server && watchexec -r -e rs,toml -- cargo run

backend:
    docker compose -f infra/docker-compose.yml up -d postgres
    cd apps/server && watchexec -r -e rs,toml -- cargo run

mobile:
    docker compose -f infra/docker-compose.yml up -d
    cd apps/server && cargo run --release
    # Add mobile-specific commands here

test-backend:
    cd apps/server && cargo test

fmt:
    cd apps/server && cargo fmt

check:
    cd apps/server && cargo check
    cd apps/server && cargo clippy

docker:
    docker compose -f infra/docker-compose.yml up

infra:
    docker compose -f infra/docker-compose.yml up -d

stop:
    docker compose -f infra/docker-compose.yml down

clean:
    docker compose -f infra/docker-compose.yml down -v
    cd apps/server && cargo clean

# Database migrations (example)
migrate:
    cd apps/server && cargo run --bin migrate

# Build for production
build:
    cd apps/server && cargo build --release
    # Add frontend build when ready
