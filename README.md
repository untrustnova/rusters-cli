# ⚡ Rusters CLI Toolchain

[![GitHub License](https://img.shields.io/github/license/untrustnova/rusters-cli?style=flat-square&color=blue)](LICENSE)
[![GitHub Tag](https://img.shields.io/github/v/tag/untrustnova/rusters-cli?style=flat-square&color=purple)](https://github.com/untrustnova/rusters-cli/releases)
[![Rust Version](https://img.shields.io/badge/rust-1.75+-orange?style=flat-square)](https://www.rust-lang.org/)
[![Bun Version](https://img.shields.io/badge/bun-1.0+-yellow?style=flat-square)](https://bun.sh/)

> The official command-line interface (CLI) for the **Rusters** framework. 

Initialize, compile, run, migrate, and link multiplexed Rust backend servers with Vite-powered Bun frontends.

---

## 🚀 Key Features

* **Interactive Setup Wizard**: Navigate frameworks, HTTP servers, and database options using your keyboard's arrow keys.
* **Auto Dependency Handling**: Runs `bun install` and pre-compiles backend dependencies (`cargo build`) automatically during project scaffolding.
* **Multiplexed Port Devserver**: Boot backend (port `4200`) and frontend (port `3000`) concurrently with colored log streaming using `cargo boot`.
* **Zero-config Webserver Symlinks**: Automatically link or unlink project configurations (`rusters.conf`) to host HTTP web daemons (Apache, Nginx, Caddy).
* **Robust DB Introspection**: Generate native Rust structures from existing schemas automatically via `cargo pull`.

---

## 📦 Installation

Ensure your machine satisfies the system requirements (Rust, Cargo, Bun). Then install the CLI toolchain globally from GitHub:

```sh
cargo install --git https://github.com/untrustnova/rusters-cli
```

*This installs the custom cargo subcommands (`cargo-ignite`, `cargo-boot`, `cargo-push`, `cargo-pull`, `cargo-seed`, `cargo-link`, `cargo-unlink`) into your `~/.cargo/bin/` folder.*

---

## 🛠 Command Reference

### `cargo ignite <project-name>`
Scaffold a fresh, ready-to-run Rusters project. Launches the interactive setup wizard:
1. **HTTP Server Configuration**: Apache | Nginx | Caddy
2. **Database Driver**: SQLite | MySQL | PostgreSQL
3. **Frontend Framework**: Vue 3 | React (TSX)

*Runs `bun install` and `cargo build` automatically to ensure the project is immediately runnable.*

### `cargo boot`
Launches the development environment. Starts both the backend Rust engine and Vite's dev server, streaming their stdout/stderr side-by-side with color-coded prefixes.
*Automatically installs node modules if they are missing.*

### `cargo push`
Applies pending database migrations.
* **`--fresh`**: Drops all tables and regenerates the database schema.
* **`--with-seed`**: Automatically chains seeder script execution after successful migration.

### `cargo seed`
Executes SQL seeder scripts located in `src/database/seeds/*.sql` in lexicographic order.

### `cargo pull`
Introspects the active database schema and writes a fully typed Rust representation to `src/database/schema.rs`.

### `cargo link`
Generates your web server routing configurations and links (`symlinks` on Unix, `copies` on Windows) them into the host web server's configuration directories before reloading the server daemon. Requires privilege escalation (`sudo` prompt).

### `cargo unlink`
Safely removes the symlink/file configurations from the host web server and reloads the daemon.

---

## 📝 Quickstart Flow

Get your first application running in seconds:

```sh
# 1. Scaffold your project (arrow keys to choose configuration)
cargo ignite my_new_app

# 2. Enter directory
cd my_new_app

# 3. Connect your local server (Apache/Nginx/Caddy)
cargo link

# 4. Migrate and seed database
cargo push --with-seed

# 5. Boot servers
cargo boot
```

---

## 📄 License

This toolchain is licensed under the Apache License 2.0. Built by the **Untrustnova** community.
