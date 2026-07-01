# 🦀 {{ PROJECT_NAME }}

> A modern, premium full-stack application built using the **Rusters** framework.

Designed for high-performance, developer ergonomics, and seamless frontend-backend integration. Powered by a Rust backend, Tailwind CSS v4, Bun, and Vite on the frontend.

---

## 🚀 Getting Started

Ensure you have [Rust](https://www.rust-lang.org/) and [Bun](https://bun.sh/) installed on your machine.

### 1. Database & Server Link
If you are deploying behind Apache, Nginx, or Caddy, link your server configuration file first:
```sh
cargo link
```
*Note: This generates a symlink or file copy into your system's HTTP server configuration directories and reloads the daemon.*

### 2. Apply Migrations & Seed
Prepare your database schema and seed the initial data:
```sh
cargo push --with-seed --fresh
```

### 3. Spin Up Development Servers
Boot the unified backend (port `{{ BACKEND_PORT }}`) and frontend (port `{{ FRONTEND_PORT }}`) simultaneously:
```sh
cargo boot
```

---

## 🛠 Command Cheat Sheet

Rusters ships with a collection of native Cargo subcommands to manage your app lifecycle:

| Command | Flags | Description |
|---|---|---|
| `cargo boot` | `--verbose` | Runs backend (`cargo run`) & frontend (`bun run dev`) concurrently with color-coded logging. |
| `cargo push` | `--fresh`, `--with-seed` | Applies pending SQL database migrations. `--fresh` drops all tables first. |
| `cargo seed` | `--verbose` | Seeds initial data from `src/database/seeds/*.sql` in alphabetical order. |
| `cargo pull` | None | Introspects your database schema and updates `src/database/schema.rs` with typed Rust structs. |
| `cargo link` | None | Symlinks `rusters.conf` to the local HTTP server and reloads the host daemon. |
| `cargo unlink` | None | Safely removes the host HTTP server link. |

---

## 📁 Directory Structure

```
├── .env                  # Section-based configuration (Custom ini-style format)
├── rusters.conf          # Auto-generated HTTP server VirtualHost/Proxy configuration
├── Cargo.toml            # Backend Rust dependencies
├── Dockerfile            # Multi-stage production build (Bun -> Rust -> Debian slim)
├── docker-compose.yml    # Production container orchestration setup
│
├── database/             # Local SQLite storage path (if SQLite is selected)
│
├── src/                  # Rust Backend Source
│   ├── main.rs           # Web server entry point and route registrations
│   ├── controllers/      # Request handlers & controllers (e.g. user.rs)
│   ├── database/         # Database connection helper, generated schema, and seeds
│   └── middleware/       # Tower auth and logging request interceptors
│
└── frontend/             # Bun Frontend Source (Vite Dev Server)
    ├── package.json      # Frontend package configuration (Tailwind v4, React/Vue)
    ├── vite.config.ts    # Frontend configuration with automatic backend API proxy
    ├── index.html        # Main HTML layout entry (Rusters template-processed)
    └── src/
        ├── App.vue / main.tsx  # Interactive frontend root styled with Tailwind CSS
        └── styles.css          # Main stylesheet loading Tailwind CSS v4
```

---

## ⚙️ Environment Configuration (`.env`)

Rusters utilizes a structured section-based `.env` format rather than flat properties.

```ini
[RUSTERS_CORE]
PROJECT_ID="{{ PROJECT_ID }}"
BACKEND_PORT={{ BACKEND_PORT }}
FRONTEND_PORT={{ FRONTEND_PORT }}
ENVIRONMENT="development"
HTTP_SERVER="{{ HTTP_SERVER }}"

[RUSTERS_DATABASE]
CONNECTION_TYPE="{{ DB_DRIVER }}"
SOURCE_PATH="database/project.sqlite"
CREDENTIAL_USER=""
CREDENTIAL_PASS=""
```

---

## 🐳 Production Deployment

Build and launch the application in production containers using Docker Compose:

```sh
docker-compose up -d --build
```
This builds the frontend production bundle via Bun, compiles the Rust backend in `--release` mode, and runs both inside a lightweight Debian runtime image, exposing port `{{ BACKEND_PORT }}` and `{{ FRONTEND_PORT }}` safely.

---

## 📄 License

This project is licensed under the Apache License 2.0. Built by the **Untrustnova** community.
