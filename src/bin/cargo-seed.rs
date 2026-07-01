//! # cargo-seed
//!
//! Populate the configured database with initial data from seeder files.
//!
//! ## Usage
//!
//! ```sh
//! cargo seed [--fresh] [--verbose]
//! ```
//!
//! Seeder files live in `src/database/seeds/` and are executed in
//! lexicographic order (prefix filenames with numbers to control order,
//! e.g. `01_users.sql`, `02_posts.sql`).

use std::fs;

use anyhow::{Context, Result};
use clap::Parser;
use colored::Colorize;
use indicatif::{ProgressBar, ProgressStyle};
use rusters_cli::flags::GlobalFlags;
use rusters_core::config::RustersConfig;

#[derive(Parser, Debug)]
#[command(
    name = "cargo-seed",
    bin_name = "cargo seed",
    about = "Populate the database from seeder files in src/database/seeds/"
)]
struct Cli {
    #[command(flatten)]
    global: GlobalFlags,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args: Vec<String> = std::env::args()
        .enumerate()
        .filter(|(i, a)| !(*i == 1 && a == "seed"))
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
        "🌱 cargo seed".magenta().bold(),
        "— Populating database from seed files".white()
    );

    let seeds_dir = config.project_root.join("src").join("database").join("seeds");
    if !seeds_dir.exists() {
        println!(
            "{} No seeds directory found at {}",
            "ℹ".blue().bold(),
            seeds_dir.display()
        );
        println!("  Create .sql files in that directory to define your seed data.");
        return Ok(());
    }

    // Collect and sort seed files
    let mut seed_files: Vec<_> = fs::read_dir(&seeds_dir)?
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.path()
                .extension()
                .map(|ext| ext == "sql")
                .unwrap_or(false)
        })
        .collect();
    seed_files.sort_by_key(|e| e.file_name());

    if seed_files.is_empty() {
        println!("{} No .sql seed files found in {}", "ℹ".blue(), seeds_dir.display());
        return Ok(());
    }

    let pool = rusters_core::db::DatabasePool::connect(&config.database).await
        .context("Failed to connect to database")?;

    let spinner = ProgressBar::new_spinner();
    spinner.set_style(
        ProgressStyle::with_template("{spinner:.cyan} {msg}")
            .unwrap()
            .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"]),
    );
    spinner.enable_steady_tick(std::time::Duration::from_millis(80));

    let mut applied = 0usize;

    for file in &seed_files {
        let path = file.path();
        let name = file.file_name();
        spinner.set_message(format!("Running seed: {}", name.to_string_lossy().yellow()));

        let sql = fs::read_to_string(&path)
            .with_context(|| format!("Failed to read seed file: {}", path.display()))?;

        // Execute each statement separately (split on ';')
        for statement in sql.split(';') {
            let stmt = statement.trim();
            if !stmt.is_empty() {
                sqlx::query(stmt)
                    .execute(&pool.pool)
                    .await
                    .with_context(|| format!("Seed failed in file: {}", path.display()))?;
            }
        }

        applied += 1;
    }

    spinner.finish_and_clear();
    println!(
        "{} {} seed file(s) applied.",
        "✔".green().bold(),
        applied
    );

    Ok(())
}
