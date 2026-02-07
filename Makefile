# ── Configuration ─────────────────────────────────────────────────────────────
ENV_FILE       := .env
ENV_EXAMPLE    := .env.example
SQLX_CACHE_DIR := .sqlx

COMPOSE ?= docker compose

# ── Local dev setup ──────────────────────────────────────────────────────────

## Full local dev setup: .env, Postgres, migrations, sqlx cache
setup: env-file db-up db-wait migrate sqlx-prepare
	@echo "Done — local dev environment is ready."

## Copy .env.example to .env (skips if .env already exists)
env-file:
	@test -f $(ENV_FILE) || cp $(ENV_EXAMPLE) $(ENV_FILE) && echo "Created $(ENV_FILE)"

# ── Database ─────────────────────────────────────────────────────────────────

## Start Postgres via $(COMPOSE)
db-up:
	$(COMPOSE) up -d db

## Stop Postgres
db-down:
	$(COMPOSE) down

## Wait for Postgres to accept connections
db-wait:
	@echo "Waiting for Postgres..."
	@until $(COMPOSE) exec db pg_isready -U dioxus -d dioxus_app > /dev/null 2>&1; do \
		sleep 1; \
	done
	@echo "Postgres is ready."

## Run sqlx migrations
migrate:
	cargo sqlx migrate run

## Generate the .sqlx offline query cache (commit this directory)
sqlx-prepare:
	cargo sqlx prepare --workspace -- --all-targets --all-features

## Reset the database (drop + recreate + migrate)
db-reset:
	cargo sqlx database drop -y
	cargo sqlx database create
	cargo sqlx migrate run

# ── Build & check ────────────────────────────────────────────────────────────

## Cargo check (workspace, no server features)
check:
	cargo check --workspace

## Cargo check with server features (requires DATABASE_URL)
check-server:
	cargo check -p server --features server

## Format all code
fmt:
	cargo fmt --all

## Run clippy
clippy:
	cargo clippy --workspace -- -D warnings

# ── Test ─────────────────────────────────────────────────────────────────────

## Run all tests
test:
	cargo test --workspace

# ── Dev server ───────────────────────────────────────────────────────────────

## Start the dioxus dev server
dev:
	dx serve --package app --platform web

## Build for release
build:
	dx bundle --package app --platform web --release

# ── Helpers ──────────────────────────────────────────────────────────────────

## Show available targets
help:
	@echo "Available targets:"
	@grep -E '^## ' Makefile | sed 's/## /  /'

.PHONY: setup env-file db-up db-down db-wait migrate sqlx-prepare db-reset \
        check check-server fmt clippy test dev build help
