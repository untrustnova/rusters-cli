# rusters-cli

CLI toolchain for the [Rusters](https://github.com/untrustnova) framework — a unified Rust backend with Bun-powered frontends.

## Install

```sh
cargo install --git https://github.com/untrustnova/rusters-cli
```

This installs all 7 `cargo-*` subcommands into `~/.cargo/bin/`.

## Commands

| Command | Description |
|---|---|
| `cargo ignite <name>` | Init a new project with an interactive arrow-key wizard |
| `cargo boot` | Start backend (port 4200) + Bun frontend (port 3000) |
| `cargo push` | Apply database migrations |
| `cargo push --fresh` | Drop all tables and re-migrate |
| `cargo push --with-seed` | Migrate then seed |
| `cargo pull` | Generate Rust structs from existing DB schema |
| `cargo seed` | Populate DB from `src/database/seeds/*.sql` |
| `cargo link` | Symlink `rusters.conf` into system HTTP server config dir |
| `cargo unlink` | Remove the config link |

## Global Flags

Available on every command:

| Flag | Description |
|---|---|
| `--version` | Print toolchain version |
| `--fresh` | Drop all state and rebuild from scratch |
| `--with-seed` | Run seeder after primary command |
| `--verbose` | Print debug logs |

## Quickstart

```sh
# 1. Install the CLI
cargo install --git https://github.com/untrustnova/rusters-cli

# 2. Create a new project (arrow-key wizard: HTTP server, DB, frontend)
cargo ignite my_app

# 3. Enter the project
cd my_app

# 4. Link HTTP server config + reload daemon
cargo link

# 5. Run migrations and seed data
cargo push --with-seed

# 6. Start the development environment
cargo boot
# → Backend: http://localhost:4200
# → Frontend: http://localhost:3000
```

## Supported Stack

| Layer | Options |
|---|---|
| HTTP Server | Apache, Nginx, Caddy |
| Database | SQLite, MySQL, PostgreSQL |
| Frontend | Vue 3, React (TSX) |

## Repository Structure

```
rusters-cli/
├── src/
│   ├── lib.rs
│   ├── flags.rs        # Global CLI flags
│   ├── wizard.rs       # Arrow-key interactive wizard
│   ├── scaffold.rs     # Project scaffold engine
│   └── bin/
│       ├── cargo-ignite.rs
│       ├── cargo-boot.rs
│       ├── cargo-push.rs
│       ├── cargo-pull.rs
│       ├── cargo-seed.rs
│       ├── cargo-link.rs
│       └── cargo-unlink.rs
└── templates/
    └── project/        # Scaffold template (output of cargo ignite)
```

## License

Apache License 2.0 — [Untrustnova](https://github.com/untrustnova)
