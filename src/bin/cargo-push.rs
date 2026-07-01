//! # cargo-push
//!
//! Apply database migrations based on the schema defined in `src/`.
//!
//! ## Usage
//!
//! ```sh
//! cargo push [--fresh] [--with-seed] [--verbose]
//! ```
//!
//! - `--fresh`     Drops all tables before running migrations.
//! - `--with-seed` Runs `cargo seed` automatically after migrations succeed.

use std::process::Command;

use anyhow::{Context, Result};
use clap::Parser;
use colored::Colorize;
use indicatif::{ProgressBar, ProgressStyle};
use rusters_cli::flags::GlobalFlags;
use rusters_core::config::{ConnectionType, RustersConfig};

#[derive(Parser, Debug)]
#[command(
    name = "cargo-push",
    bin_name = "cargo push",
    about = "Apply database migrations (optionally reset with --fresh)"
)]
struct Cli {
    #[command(flatten)]
    global: GlobalFlags,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args: Vec<String> = std::env::args()
        .enumerate()
        .filter(|(i, a)| !(*i == 1 && a == "push"))
        .map(|(_, a)| a)
        .collect();

    let cli = Cli::parse_from(args);
    cli.global.init_tracing();

    if cli.global.maybe_print_version() {
        return Ok(());
    }

    let config = RustersConfig::load()
        .context("Could not load .env — are you inside a Rusters project?")?;

    println!(
        "\n{} {}",
        "📦 cargo push".magenta().bold(),
        "— Applying migrations".white()
    );

    if cli.global.fresh {
        println!("{}", "  ⚠  --fresh: all existing tables will be dropped".yellow());
    }

    let spinner = ProgressBar::new_spinner();
    spinner.set_style(
        ProgressStyle::with_template("{spinner:.cyan} {msg}")
            .unwrap()
            .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"]),
    );
    spinner.enable_steady_tick(std::time::Duration::from_millis(80));

    // -----------------------------------------------------------------------
    // SQLite: ensure the database file exists
    // -----------------------------------------------------------------------
    if config.database.connection_type == ConnectionType::Sqlite {
        if let Some(path) = &config.database.source_path {
            rusters_core::db::sqlite::ensure_file(path)
                .context("Failed to create SQLite database file")?;
            spinner.set_message(format!("SQLite: {}", path.display()));
        }
    }

    // -----------------------------------------------------------------------
    // Connect and run migrations
    // -----------------------------------------------------------------------
    spinner.set_message("Connecting to database…");
    let pool = rusters_core::db::DatabasePool::connect(&config.database).await
        .context("Failed to connect to database")?;

    if cli.global.fresh {
        spinner.set_message("Dropping all tables (--fresh)…");
        // Issue a raw "DROP TABLE" for known migration tables so sqlx can re-run
        sqlx::query("DROP TABLE IF EXISTS _sqlx_migrations")
            .execute(&pool.pool)
            .await
            .context("Failed to drop migration table")?;
    }

    spinner.set_message("Running migrations…");
    let migrations_path = config.project_root.join("migrations");
    if !migrations_path.exists() {
        std::fs::create_dir_all(&migrations_path)
            .context("Failed to create migrations directory")?;
    }
    pool.run_migrations(&migrations_path).await
        .context("Migration failed — ensure migrations/ directory exists with .sql files")?;

    spinner.finish_and_clear();
    println!("{}", "✔  Migrations applied successfully!".green().bold());

    // -----------------------------------------------------------------------
    // --with-seed: invoke cargo-seed
    // -----------------------------------------------------------------------
    if cli.global.with_seed {
        println!("{}", "\n  → Running seeder (--with-seed)…".cyan());
        let status = Command::new("cargo")
            .arg("seed")
            .status()
            .context("Failed to invoke cargo seed")?;

        if !status.success() {
            anyhow::bail!("cargo seed exited with a non-zero status");
        }
    }

    Ok(())
}
