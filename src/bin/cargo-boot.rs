//! # cargo-boot
//!
//! Spin up the full Rusters development environment.
//!
//! Starts:
//!   1. The Rust backend (via `cargo run`) on `BACKEND_PORT` (default 4200).
//!   2. The Bun dev server (`bun run dev`) on `FRONTEND_PORT` (default 3000).
//!
//! Both processes stream their output with colour-coded prefixes.
//!
//! ## Usage
//!
//! ```sh
//! cargo boot [--verbose]
//! ```

use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};
use std::thread;

use anyhow::{Context, Result};
use clap::Parser;
use colored::Colorize;
use rusters_cli::flags::GlobalFlags;
use rusters_core::config::RustersConfig;

#[derive(Parser, Debug)]
#[command(
    name = "cargo-boot",
    bin_name = "cargo boot",
    about = "Start the Rusters development environment (backend + Bun frontend)"
)]
struct Cli {
    #[command(flatten)]
    global: GlobalFlags,
}

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args()
        .enumerate()
        .filter(|(i, a)| !(*i == 1 && a == "boot"))
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
        "\n{} {} {}",
        "🦀 Rusters".magenta().bold(),
        "·".dimmed(),
        "Booting development environment".white()
    );
    println!(
        "   Backend  → {}",
        format!("http://localhost:{}", config.core.backend_port).cyan()
    );
    println!(
        "   Frontend → {}",
        format!("http://localhost:{}", config.core.frontend_port).cyan()
    );
    println!();

    let project_root = config.project_root.clone();
    let backend_port = config.core.backend_port;
    let frontend_port = config.core.frontend_port;
    let _verbose = cli.global.verbose;

    // -----------------------------------------------------------------------
    // Backend: cargo run
    // -----------------------------------------------------------------------
    let backend_root = project_root.clone();
    let backend_handle = thread::spawn(move || -> Result<()> {
        let mut cmd = Command::new("cargo");
        cmd.arg("run")
            .env("BACKEND_PORT", backend_port.to_string())
            .current_dir(&backend_root)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

        let mut child = cmd.spawn().context("Failed to start Rust backend (cargo run)")?;

        let stdout = child.stdout.take().unwrap();
        let stderr = child.stderr.take().unwrap();

        let prefix_out = "[backend]".bright_blue().bold().to_string();
        let prefix_err = "[backend]".red().bold().to_string();

        let out_thread = thread::spawn(move || {
            for line in BufReader::new(stdout).lines().flatten() {
                println!("{} {}", prefix_out, line);
            }
        });

        let err_thread = thread::spawn(move || {
            for line in BufReader::new(stderr).lines().flatten() {
                eprintln!("{} {}", prefix_err, line);
            }
        });

        child.wait()?;
        let _ = out_thread.join();
        let _ = err_thread.join();
        Ok(())
    });

    // -----------------------------------------------------------------------
    // Frontend: bun run dev
    // -----------------------------------------------------------------------
    let frontend_root = project_root.join("frontend");
    let frontend_handle = thread::spawn(move || -> Result<()> {
        let node_modules = frontend_root.join("node_modules");
        if !node_modules.exists() {
            println!("{} Frontend node_modules not found. Running bun install...", "[frontend]".bright_green().bold());
            let status = Command::new("bun")
                .arg("install")
                .current_dir(&frontend_root)
                .status()
                .context("Failed to run bun install")?;
            if !status.success() {
                anyhow::bail!("bun install failed");
            }
        }

        let mut cmd = Command::new("bun");
        cmd.arg("run")
            .arg("dev")
            .env("PORT", frontend_port.to_string())
            .current_dir(&frontend_root)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

        let mut child = cmd.spawn().context("Failed to start Bun frontend (bun run dev)")?;

        let stdout = child.stdout.take().unwrap();
        let stderr = child.stderr.take().unwrap();

        let prefix_out = "[frontend]".bright_green().bold().to_string();
        let prefix_err = "[frontend]".yellow().bold().to_string();

        let out_thread = thread::spawn(move || {
            for line in BufReader::new(stdout).lines().flatten() {
                println!("{} {}", prefix_out, line);
            }
        });

        let err_thread = thread::spawn(move || {
            for line in BufReader::new(stderr).lines().flatten() {
                eprintln!("{} {}", prefix_err, line);
            }
        });

        child.wait()?;
        let _ = out_thread.join();
        let _ = err_thread.join();
        Ok(())
    });

    // Wait for both processes; report errors
    let backend_result = backend_handle.join().expect("Backend thread panicked");
    let frontend_result = frontend_handle.join().expect("Frontend thread panicked");

    backend_result.context("Backend process exited with error")?;
    frontend_result.context("Frontend process exited with error")?;

    Ok(())
}
