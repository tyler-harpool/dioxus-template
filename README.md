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
make web
```

The app will be available at the URL printed in the terminal (typically `http://127.0.0.1:8080`).

## Make Targets

### Local Dev Setup

| Command              | Description                                                |
| -------------------- | ---------------------------------------------------------- |
| `make setup`         | Full local dev setup (.env, Postgres, migrations, sqlx)    |
| `make env-file`      | Create `.env` from `.env.example` (skips if exists)        |

### Database

| Command              | Description                                                |
| -------------------- | ---------------------------------------------------------- |
| `make db-up`         | Start PostgreSQL via Docker Compose                        |
| `make db-down`       | Stop all Compose services                                  |
| `make db-wait`       | Wait for Postgres to accept connections                    |
| `make db-reset`      | Drop, recreate, and migrate the database                   |
| `make migrate`       | Run sqlx migrations                                        |
| `make sqlx-prepare`  | Regenerate `.sqlx` offline query cache                     |

### Services

| Command              | Description                                                |
| -------------------- | ---------------------------------------------------------- |
| `make services`      | Start all services (Postgres + MinIO + SigNoz)             |
| `make services-down` | Stop all services                                          |
| `make minio-up`      | Start MinIO (S3-compatible object storage)                 |
| `make minio-init`    | Start MinIO and create the avatars bucket                  |
| `make signoz-up`     | Start SigNoz (dashboard at `http://localhost:3301`)        |
| `make signoz-down`   | Stop SigNoz                                                |

### Dev Servers

| Command              | Description                                                |
| -------------------- | ---------------------------------------------------------- |
| `make web`           | Start the Dioxus web dev server (fullstack)                |
| `make desktop`       | Start the Dioxus desktop dev server                        |
| `make mobile`        | Start the Dioxus mobile dev server (iOS simulator)         |
| `make build`         | Bundle for release                                         |

### Build & Lint

| Command              | Description                                                |
| -------------------- | ---------------------------------------------------------- |
| `make check`         | Cargo check (workspace, no server features)                |
| `make check-server`  | Cargo check with server features (requires DATABASE_URL)   |
| `make check-platforms` | Check all platform feature flags (web, desktop, mobile, server) |
| `make fmt`           | Format all code                                            |
| `make clippy`        | Run clippy lints                                           |
| `make test`          | Run all tests                                              |

### CI/CD & Deployment

| Command              | Description                                                |
| -------------------- | ---------------------------------------------------------- |
| `make ci`            | Run full CI checks (fmt, check, clippy, test, sqlx)        |
| `make deploy`        | Full deploy pipeline (CI + push + Fly.io)                  |
| `make git-push`      | Git add, commit, and push (prompts for message)            |
| `make fly-secrets`   | Sync `.env.production` secrets to Fly.io                   |
| `make fly-deploy`    | Deploy to Fly.io                                           |
| `make promote-user`  | Promote a user to admin (`EMAIL=user@example.com`)         |

## API Documentation

Once the dev server is running (`make web`), navigate to `/docs` for the interactive Scalar UI where you can browse and test all API endpoints.

### REST Endpoints

| Method   | Path                        | Description               |
| -------- | --------------------------- | ------------------------- |
| `POST`   | `/api/auth/register`        | Register a new user       |
| `POST`   | `/api/auth/login`           | Login with email/password |
| `POST`   | `/api/auth/logout`          | Logout (revoke tokens)    |
| `GET`    | `/api/users`                | List all users            |
| `GET`    | `/api/users/{user_id}`      | Get user by ID            |
| `POST`   | `/api/users`                | Create a user             |
| `PUT`    | `/api/users/{user_id}`      | Update a user             |
| `DELETE` | `/api/users/{user_id}`      | Delete a user             |
| `PUT`    | `/api/users/{user_id}/tier` | Update user tier (admin)  |
| `POST`   | `/api/users/me/avatar`      | Upload avatar (multipart) |
| `GET`    | `/api/products`             | List all products         |
| `POST`   | `/api/products`             | Create a product          |
| `PUT`    | `/api/products/{id}`        | Update a product          |
| `DELETE` | `/api/products/{id}`        | Delete a product          |
| `GET`    | `/api/dashboard/stats`      | Dashboard statistics      |
| `GET`    | `/health`                   | Health check              |

## Theming

The app ships with two themes defined in `crates/app/assets/cyberpunk-theme.css`:

- **Cyberpunk** — the default dark theme with neon cyan accents on deep blue-black backgrounds
- **Light** — clean whites with teal accents

Toggle between them using the switch in the sidebar footer. All 38 components automatically adapt via CSS custom properties.

### Creating a Custom Theme

All 38 UI components are styled through CSS custom properties, so adding a new theme requires zero Rust changes. Just define your palette and activate it.

**Step 1 — Add a `[data-theme]` block** to `crates/app/assets/cyberpunk-theme.css`:

```css
[data-theme="solar"] {
    /* Dark/light mode flag (pick one) */
    --dark: initial;   /* set --dark for dark themes */
    --light: ;         /* leave empty for dark themes (swap for light) */

    /* Primary palette — backgrounds, surfaces, borders (dark to light) */
    --primary-color-1: #002b36;
    --primary-color-2: #073642;
    --primary-color-3: #0a3f4e;
    --primary-color-4: #0e4d5e;
    --primary-color-5: #155a6b;
    --primary-color-6: #1c6e80;
    --primary-color-7: #268399;
    --primary-color-8: #2aa198;
    --primary-color-9: #35c4ba;

    /* Secondary palette — text colors (light to dark) */
    --secondary-color-1: #fdf6e3;
    --secondary-color-2: #eee8d5;
    --secondary-color-3: #c4b99a;
    --secondary-color-4: #93a1a1;
    --secondary-color-5: #657b83;
    --secondary-color-6: #586e75;
    --secondary-color-7: #4a6068;

    /* Semantic mappings (point these at palette slots) */
    --color-background: var(--primary-color-2);
    --color-surface: var(--primary-color-3);
    --color-surface-raised: var(--primary-color-5);
    --color-surface-dialog: var(--primary-color-6);
    --color-on-surface: var(--secondary-color-1);
    --color-on-surface-muted: var(--secondary-color-4);
    --color-border: var(--primary-color-6);

    /* Accent colors */
    --color-primary: #b58900;
    --color-primary-hover: #d4a017;
    --color-on-primary: #002b36;

    --color-secondary: #6c71c4;
    --color-secondary-hover: #8a8fd6;
    --color-on-secondary: #ffffff;

    --color-danger: #dc322f;
    --color-on-danger: #ffffff;
    --color-success: #859900;
    --color-on-success: #002b36;
    --color-warning: #cb4b16;
    --color-on-warning: #ffffff;

    --focused-border-color: var(--color-primary);

    /* Shadows and glow effects */
    --shadow-sm: 0 1px 3px rgba(0, 0, 0, 0.4);
    --shadow-md: 0 4px 8px rgba(0, 0, 0, 0.4);
    --shadow-lg: 0 10px 20px rgba(0, 0, 0, 0.4);
    --cyber-neon-glow: 0 0 8px rgba(181, 137, 0, 0.4);
    --cyber-neon-glow-strong: 0 0 12px rgba(181, 137, 0, 0.6);
    --cyber-scanline-opacity: 0;
}
```

**Step 2 — Activate it** from anywhere in your app:

```rust
shared_ui::theme::set_theme("solar");
```

That's it. Every component picks up the new palette automatically. The theme persists across page reloads via a cookie and syncs across tabs.

#### Variable Reference

| Variable Group | Purpose |
| --- | --- |
| `--primary-color-1` to `--primary-color-9` | Background/surface palette (darkest to lightest) |
| `--secondary-color-1` to `--secondary-color-7` | Text palette (lightest to darkest) |
| `--color-background`, `--color-surface`, `--color-surface-raised`, `--color-surface-dialog` | Semantic surface mappings |
| `--color-on-surface`, `--color-on-surface-muted` | Text on surfaces |
| `--color-primary`, `--color-primary-hover`, `--color-on-primary` | Primary accent (buttons, links, focus rings) |
| `--color-secondary`, `--color-danger`, `--color-success`, `--color-warning` | Additional accent colors |
| `--cyber-neon-glow`, `--cyber-neon-glow-strong` | Focus/hover glow effects |
| `--cyber-scanline-opacity` | Scanline overlay on primary buttons (0 to disable) |

## Offline Builds

The `.sqlx/` directory contains cached query metadata so the project compiles without a running database. This is used by the Dockerfile (`SQLX_OFFLINE=true`) and CI. Regenerate it after changing any SQL queries:

```bash
make sqlx-prepare
```

Commit the `.sqlx/` directory after regenerating.

## Deployment

The included `Dockerfile` builds a multi-stage production image. A `fly.toml` is provided for deploying to [Fly.io](https://fly.io/).
