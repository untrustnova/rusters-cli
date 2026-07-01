//! # cargo-ignite
//!
//! Initialise a fresh Rusters project with an interactive wizard.
//!
//! ## Usage
//!
//! ```sh
//! cargo ignite <project-name> [--verbose]
//! ```
//!
//! The wizard uses arrow keys + Enter — no number typing required.

use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;
use colored::Colorize;
use indicatif::{ProgressBar, ProgressStyle};
use rusters_cli::{
    flags::GlobalFlags,
    scaffold::{scaffold, ScaffoldContext},
    wizard,
};

#[derive(Parser, Debug)]
#[command(
    name = "cargo-ignite",
    bin_name = "cargo ignite",
    about = "Initialise a new Rusters project with an interactive setup wizard"
)]
struct Cli {
    /// Name of the new project (also used as the directory name).
    project_name: String,

    #[command(flatten)]
    global: GlobalFlags,
}

fn main() -> Result<()> {
    // cargo passes "ignite" as the first arg; skip it
    let args: Vec<String> = std::env::args()
        .enumerate()
        .filter(|(i, a)| !(*i == 1 && a == "ignite"))
        .map(|(_, a)| a)
        .collect();

    let cli = Cli::parse_from(args);
    cli.global.init_tracing();

    if cli.global.maybe_print_version() {
        return Ok(());
    }

    let project_name = &cli.project_name;

    // Run the interactive wizard
    let (http_idx, db_idx, fe_idx) = wizard::run_ignite_wizard(project_name)?;

    let http_server = match http_idx {
        0 => "apache",
        1 => "nginx",
        _ => "caddy",
    };
    let db_driver = match db_idx {
        0 => "sqlite",
        1 => "mysql",
        _ => "postgres",
    };
    let frontend = match fe_idx {
        0 => "vue",
        _ => "react",
    };

    // Derive a sanitised project_id
    let project_id = project_name
        .to_lowercase()
        .replace([' ', '-'], "_")
        .chars()
        .filter(|c| c.is_alphanumeric() || *c == '_')
        .collect::<String>();

    let ctx = ScaffoldContext {
        project_name: project_name.clone(),
        project_id,
        http_server: http_server.to_string(),
        db_driver: db_driver.to_string(),
        frontend: frontend.to_string(),
        backend_port: 4200,
        frontend_port: 3000,
    };

    let target = PathBuf::from(project_name);

    if target.exists() {
        wizard::warn(&format!(
            "Directory '{}' already exists. Continuing may overwrite files.",
            project_name
        ));
        if !wizard::confirm("Continue anyway?", false)? {
            println!("{}", "Aborted.".red());
            return Ok(());
        }
    }

    // Scaffold with a progress spinner
    let spinner = ProgressBar::new_spinner();
    spinner.set_style(
        ProgressStyle::with_template("{spinner:.cyan} {msg}")
            .unwrap()
            .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"]),
    );
    spinner.set_message(format!("Scaffolding {}…", project_name.white().bold()));
    spinner.enable_steady_tick(std::time::Duration::from_millis(80));

    scaffold(&ctx, &target)?;

    spinner.set_message("Installing frontend dependencies with Bun…");
    let status = std::process::Command::new("bun")
        .arg("install")
        .current_dir(target.join("frontend"))
        .status();

    spinner.finish_and_clear();

    match status {
        Ok(s) if s.success() => {
            wizard::success("Frontend dependencies installed successfully.");
        }
        _ => {
            wizard::warn("Failed to run `bun install` automatically. Please run it manually inside the `frontend` directory.");
        }
    }

    // Pre-compile Rust backend dependencies
    let rust_spinner = ProgressBar::new_spinner();
    rust_spinner.set_style(
        ProgressStyle::with_template("{spinner:.cyan} {msg}")
            .unwrap()
            .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"]),
    );
    rust_spinner.set_message("Compiling Rust backend dependencies (cargo build)…");
    rust_spinner.enable_steady_tick(std::time::Duration::from_millis(80));

    let rust_status = std::process::Command::new("cargo")
        .arg("build")
        .current_dir(&target)
        .status();

    rust_spinner.finish_and_clear();

    match rust_status {
        Ok(s) if s.success() => {
            wizard::success("Rust backend compiled successfully.");
        }
        _ => {
            wizard::warn("Failed to pre-compile the Rust backend. You can compile it manually by running `cargo build` inside the project directory.");
        }
    }

    println!("\n{}", "✔  Project scaffolded successfully!".green().bold());
    println!();
    println!("   {} {}", "→  cd".dimmed(), project_name.white().bold());
    println!("   {} {}", "→ ".dimmed(), "cargo link".white().bold());
    println!("   {} {}", "→ ".dimmed(), "cargo push --with-seed".white().bold());
    println!("   {} {}", "→ ".dimmed(), "cargo boot".white().bold());
    println!();

    Ok(())
}
