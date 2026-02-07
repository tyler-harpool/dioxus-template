# Dioxus Fullstack Template

A fullstack Rust application built with [Dioxus](https://dioxuslabs.com/) 0.7, PostgreSQL, and Axum. Features a cyberpunk-themed UI component library, server-side rendering, and interactive API documentation.

## Project Structure

```
crates/
  app/            # Frontend — routes, pages, layout, theme assets
  server/         # Backend — Dioxus server fns, REST API, database layer, OpenAPI docs
  shared-types/   # Shared data models (User, Product, DashboardStats)
  shared-ui/      # 38 cyberpunk-themed UI components wrapping dioxus-primitives
```

## Features

- **Fullstack Rust** — shared types between frontend and backend, no serialization mismatches
- **38 UI components** — cyberpunk-styled wrappers around [dioxus-primitives](https://github.com/DioxusLabs/components) (buttons, dialogs, forms, sidebar, calendar, toast notifications, and more)
- **Dark / Light theme** — toggle between cyberpunk dark and light modes via the sidebar
- **Responsive layout** — sidebar collapses to a mobile drawer on small screens
- **OpenAPI docs** — interactive Swagger UI at `/docs` when running fullstack
- **PostgreSQL** — async database access via sqlx with compile-time checked queries
- **Offline builds** — `.sqlx/` cache allows building without a running database

## Pages

| Route        | Description                                                                     |
| ------------ | ------------------------------------------------------------------------------- |
| `/`          | **Dashboard** — statistics cards, product table with search/filter              |
| `/users`     | **Users** — CRUD user management with checkboxes, context menus, avatar badges  |
| `/products`  | **Products** — product catalog with create/edit dialogs and tab navigation      |
| `/settings`  | **Settings** — profile form, theme toggle, notifications, calendar, danger zone |

## UI Components

The `shared-ui` crate provides 38 themed components:

**Layout:** Sidebar, Navbar, Card, Separator, AspectRatio, ScrollArea, Sheet

**Forms:** Button, Input, Textarea, Checkbox, RadioGroup, Select, Slider, Switch, Toggle, ToggleGroup, Form, Label, DatePicker

**Feedback:** Dialog, AlertDialog, Toast, Tooltip, HoverCard, Popover, Progress, Skeleton, Badge

**Navigation:** Tabs, Accordion, Collapsible, Toolbar, Menubar, ContextMenu, DropdownMenu

**Data:** Avatar, Calendar

## Prerequisites

- [Rust](https://rustup.rs/) (stable)
- A container runtime for PostgreSQL — [OrbStack](https://orbstack.dev/) (recommended on macOS), [Docker Desktop](https://www.docker.com/), or [Rancher Desktop](https://rancherdesktop.io/)
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

The app will be available at the URL printed in the terminal (typically `http://127.0.0.1:8080`).

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

Once the dev server is running (`make dev`), navigate to `/docs` for the interactive Swagger UI where you can browse and test all API endpoints.

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

## Theming

The app ships with two themes defined in `crates/app/assets/cyberpunk-theme.css`:

- **Cyberpunk (dark)** — neon cyan accents on deep blue-black backgrounds
- **Light** — clean whites with teal accents

Toggle between them using the switch in the sidebar footer. Components automatically adapt via CSS custom properties.

## Offline Builds

The `.sqlx/` directory contains cached query metadata so the project compiles without a running database. This is used by the Dockerfile (`SQLX_OFFLINE=true`) and CI. Regenerate it after changing any SQL queries:

```bash
make sqlx-prepare
```

Commit the `.sqlx/` directory after regenerating.

## Deployment

The included `Dockerfile` builds a multi-stage production image. A `fly.toml` is provided for deploying to [Fly.io](https://fly.io/).
