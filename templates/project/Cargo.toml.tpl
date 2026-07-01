[package]
name = "{{ PROJECT_ID }}"
version = "0.1.0"
edition = "2021"
authors = []

[[bin]]
name = "{{ PROJECT_ID }}"
path = "src/main.rs"

[dependencies]
rusters-core = "0.1"

tokio = { version = "1", features = ["full"] }
axum = { version = "0.7", features = ["macros"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
anyhow = "1"
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }
chrono = { version = "0.4", features = ["serde"] }
uuid = { version = "1", features = ["v4", "serde"] }

[dependencies.sqlx]
version = "0.7"
features = ["runtime-tokio-rustls", "macros", "migrate", "{{ DB_DRIVER }}"]
