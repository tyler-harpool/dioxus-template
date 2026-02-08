# ── Configuration ─────────────────────────────────────────────────────────────
ENV_FILE       := .env
ENV_EXAMPLE    := .env.example
SQLX_CACHE_DIR := .sqlx
SIGNOZ_DIR     := signoz

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

# ── Observability (SigNoz) ───────────────────────────────────────────────────

## Clone SigNoz and start containers (dashboard at http://localhost:3301)
signoz-up:
	@if [ ! -d $(SIGNOZ_DIR) ]; then \
		git clone -b main https://github.com/SigNoz/signoz.git $(SIGNOZ_DIR); \
	fi
	cd $(SIGNOZ_DIR)/deploy/docker && $(COMPOSE) pull
	cd $(SIGNOZ_DIR)/deploy/docker && $(COMPOSE) up -d --remove-orphans

## Stop SigNoz
signoz-down:
	@if [ -d $(SIGNOZ_DIR)/deploy/docker ]; then \
		cd $(SIGNOZ_DIR)/deploy/docker && $(COMPOSE) down; \
	fi

## Start all services (Postgres + SigNoz)
services: db-up signoz-up

## Stop all services
services-down: signoz-down db-down

# ── Build & check ────────────────────────────────────────────────────────────

## Cargo check (workspace, no server features)
check:
	cargo check --workspace

## Cargo check with server features (requires DATABASE_URL)
check-server:
	cargo check -p server --features server

## Cargo check all platform feature flags (web, desktop, mobile, server)
check-platforms:
	cargo check -p app --features web
	cargo check -p app --features desktop
	cargo check -p app --features mobile
	cargo check -p app --features server

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

## Start the dioxus dev server (fullstack with server-side telemetry)
dev:
	dx serve --package app --platform web --fullstack

## Build for release
build:
	dx bundle --package app --platform web --release

## Start the dioxus mobile dev server (iOS simulator)
mobile:
	dx serve --package app --platform ios --fullstack

## Start the dioxus desktop dev server
desktop:
	dx serve --package app --platform desktop --fullstack

# ── CI/CD ────────────────────────────────────────────────────────────────────

## Run full CI pipeline: fmt, check, clippy, test, sqlx-prepare, push, deploy
deploy: ci git-push fly-deploy
	@echo "Deploy complete."

## Run CI checks only (no push/deploy)
ci: fmt check check-server check-platforms clippy test sqlx-prepare
	@echo "All CI checks passed."

## Git add, commit, and push to origin (prompts for commit message)
git-push:
	@if git diff --quiet && git diff --cached --quiet && [ -z "$$(git ls-files --others --exclude-standard)" ]; then \
		echo "No changes to commit — pushing existing commits."; \
	else \
		read -p "Commit message: " msg; \
		git add -A; \
		git commit -m "$$msg"; \
	fi
	git push origin $$(git branch --show-current)

## Deploy to Fly.io
fly-deploy:
	flyctl deploy --remote-only

# ── Helpers ──────────────────────────────────────────────────────────────────

## Show available targets
help:
	@echo "Available targets:"
	@grep -E '^## ' Makefile | sed 's/## /  /'

.PHONY: setup env-file db-up db-down db-wait migrate sqlx-prepare db-reset \
        check check-server check-platforms fmt clippy test \
        dev build mobile desktop help \
        signoz-up signoz-down services services-down \
        deploy ci git-push fly-deploy
