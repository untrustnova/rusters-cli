# {{ PROJECT_NAME }}

A Rusters-powered web application.

## Getting Started

```sh
# Start the development environment
cargo boot

# Apply database migrations
cargo push

# Populate with seed data
cargo seed

# Or do all of the above at once
cargo push --with-seed --fresh && cargo boot
```

## Commands

| Command | Description |
|---|---|
| `cargo boot` | Start backend (port {{ BACKEND_PORT }}) + frontend (port {{ FRONTEND_PORT }}) |
| `cargo push` | Apply database migrations |
| `cargo push --fresh` | Drop all tables and re-migrate |
| `cargo push --with-seed` | Migrate then seed |
| `cargo pull` | Generate Rust structs from existing DB schema |
| `cargo seed` | Populate DB from `src/database/seeds/*.sql` |
| `cargo link` | Link `rusters.conf` to system HTTP server |
| `cargo unlink` | Remove the configuration link |

## Project Structure

```
.
├── src/
│   ├── main.rs              # Entry point & route registration
│   ├── controllers/         # Request handlers (CRUD)
│   ├── middleware/          # Auth, logging, etc.
│   └── database/            # Connection + schema + seeds/
├── frontend/                # Bun + {{ FRONTEND }} app (port {{ FRONTEND_PORT }})
├── rustplace/               # Framework core (vendor-equivalent)
├── database/                # SQLite file (if applicable)
├── rusters.conf             # HTTP server configuration
└── .env                     # Section-based configuration
```

## License

Apache License 2.0
