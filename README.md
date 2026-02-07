# Dioxus Fullstack Template

A fullstack Rust application built with [Dioxus](https://dioxuslabs.com/) 0.7, PostgreSQL, and Axum.

## Project Structure

```
crates/
  app/            # Frontend application — routes, pages, entry point
  server/         # Backend — Dioxus server fns, REST API, database layer, OpenAPI docs
  shared-types/   # Shared data models (User, Product, DashboardStats)
  shared-ui/      # 38 cyberpunk-themed UI components wrapping dioxus-primitives
```

## Prerequisites

- [Rust](https://rustup.rs/) (stable)
- A container runtime for PostgreSQL — [OrbStack](https://orbstack.dev/) (recommended, macOS), [Docker Desktop](https://www.docker.com/), or [Rancher Desktop](https://rancherdesktop.io/)
- [Dioxus CLI](https://dioxuslabs.com/learn/0.7/getting_started/) (`cargo install dioxus-cli`)
- [sqlx-cli](https://github.com/launchbadge/sqlx/tree/main/sqlx-cli) (`cargo install sqlx-cli --no-default-features --features postgres`)

## Quick Start

Start your container runtime, then run:

```bash
make setup
```

If you're using a non-Docker runtime like Rancher Desktop with containerd, override the compose command:

```bash
COMPOSE="nerdctl compose" make setup
```

This will:

1. Create a `.env` file from `.env.example`
2. Start PostgreSQL via Docker Compose
3. Wait for the database to be ready
4. Run sqlx migrations
5. Generate the `.sqlx` offline query cache

Then start the dev server:

```bash
make dev
```

## Make Targets

| Command              | Description                                       |
| -------------------- | ------------------------------------------------- |
| `make setup`         | Full local dev setup (one command)                 |
| `make db-up`         | Start PostgreSQL                                   |
| `make db-down`       | Stop PostgreSQL                                    |
| `make db-reset`      | Drop, recreate, and migrate the database           |
| `make migrate`       | Run sqlx migrations                                |
| `make sqlx-prepare`  | Regenerate `.sqlx` offline query cache             |
| `make dev`           | Start the Dioxus dev server                        |
| `make build`         | Bundle for release                                 |
| `make check`         | Cargo check (workspace)                            |
| `make check-server`  | Cargo check with server features                   |
| `make test`          | Run all tests                                      |
| `make fmt`           | Format code                                        |
| `make clippy`        | Run clippy lints                                   |

## API Documentation

Once the dev server is running (`make dev`), open the URL shown in the terminal and navigate to:

- **Interactive docs** — `/docs`

For example, if the server is at `http://127.0.0.1:50222`, visit `http://127.0.0.1:50222/docs` to browse and test all API endpoints.

### REST Endpoints

| Method   | Path                      | Description             |
| -------- | ------------------------- | ----------------------- |
| `GET`    | `/api/users`              | List all users          |
| `GET`    | `/api/users/{user_id}`    | Get user by ID          |
| `POST`   | `/api/users`              | Create a user           |
| `PUT`    | `/api/users/{user_id}`    | Update a user           |
| `DELETE` | `/api/users/{user_id}`    | Delete a user           |
| `GET`    | `/api/products`           | List all products       |
| `POST`   | `/api/products`           | Create a product        |
| `PUT`    | `/api/products/{id}`      | Update a product        |
| `DELETE` | `/api/products/{id}`      | Delete a product        |
| `GET`    | `/api/dashboard/stats`    | Dashboard statistics    |

## Offline Builds

The `.sqlx/` directory contains cached query metadata so the project can compile without a running database. This is used by the Dockerfile (`SQLX_OFFLINE=true`) and CI. Regenerate it after changing any SQL queries:

```bash
make sqlx-prepare
```

Commit the `.sqlx/` directory after regenerating.

## Deployment

The included `Dockerfile` builds a multi-stage production image. A `fly.toml` is provided for deploying to [Fly.io](https://fly.io/).
