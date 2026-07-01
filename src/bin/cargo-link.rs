//! # cargo-link
//!
//! Symlink (or hard-copy on Windows) the project's `rusters.conf` into the
//! system HTTP server configuration directory, then gracefully reload the daemon.
//!
//! ## Usage
//!
//! ```sh
//! cargo link [--verbose]
//! ```
//!
//! The command will prompt for privilege escalation (`sudo` / `pkexec`) on Unix
//! or copy to the XAMPP conf directory on Windows.

use std::path::PathBuf;
use std::process::Command;

use anyhow::{Context, Result};
use clap::Parser;
use colored::Colorize;
use rusters_cli::{flags::GlobalFlags, wizard};
use rusters_core::config::{HttpServer, RustersConfig};

#[derive(Parser, Debug)]
#[command(
    name = "cargo-link",
    bin_name = "cargo link",
    about = "Link rusters.conf into the system HTTP server configuration directory"
)]
struct Cli {
    #[command(flatten)]
    global: GlobalFlags,
}

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args()
        .enumerate()
        .filter(|(i, a)| !(*i == 1 && a == "link"))
        .map(|(_, a)| a)
        .collect();

    let cli = Cli::parse_from(args);
    cli.global.init_tracing();

    if cli.global.maybe_print_version() {
        return Ok(());
    }

    let config = RustersConfig::load()
        .context("Could not load .env — are you inside a Rusters project?")?;

    let src = config.project_root.join("rusters.conf");
    if !src.exists() {
        anyhow::bail!(
            "rusters.conf not found in project root: {}\n\
             Run `cargo ignite` to generate it.",
            config.project_root.display()
        );
    }

    let (conf_dir, daemon) = server_paths(&config.core.http_server);
    let project_id = &config.core.project_id;
    let dst = conf_dir.join(format!("{project_id}.conf"));

    println!(
        "\n{} {}",
        "🔗 cargo link".magenta().bold(),
        "— Linking server configuration".white()
    );
    println!("   Source → {}", src.display().to_string().cyan());
    println!("   Target → {}", dst.display().to_string().cyan());
    println!("   Daemon → {}", daemon.yellow());
    println!();

    #[cfg(unix)]
    link_unix(&src, &dst, &conf_dir, daemon, &config.core.project_id)?;

    #[cfg(windows)]
    link_windows(&src, &dst, &conf_dir, daemon)?;

    wizard::success("Server configuration linked and daemon reloaded.");

    Ok(())
}

/// Returns `(conf_dir, daemon_name)` for the selected HTTP server.
fn server_paths(server: &HttpServer) -> (PathBuf, &'static str) {
    match server {
        HttpServer::Apache => (
            PathBuf::from("/etc/httpd/conf/rusters.d"),
            "httpd",
        ),
        HttpServer::Nginx => (
            PathBuf::from("/etc/nginx/conf.d"),
            "nginx",
        ),
        HttpServer::Caddy => (
            PathBuf::from("/etc/caddy/conf.d"),
            "caddy",
        ),
    }
}

#[cfg(unix)]
fn link_unix(
    src: &PathBuf,
    dst: &PathBuf,
    conf_dir: &PathBuf,
    daemon: &str,
    _project_id: &str,
) -> Result<()> {
    

    // Create conf dir if it doesn't exist (requires sudo)
    if !conf_dir.exists() {
        let status = Command::new("sudo")
            .args(["mkdir", "-p", &conf_dir.to_string_lossy()])
            .status()
            .context("Failed to run sudo mkdir")?;
        if !status.success() {
            anyhow::bail!("Failed to create conf directory (privilege error)");
        }
    }

    // Remove stale link/file if present
    if dst.exists() || dst.symlink_metadata().is_ok() {
        let status = Command::new("sudo")
            .args(["rm", "-f", &dst.to_string_lossy()])
            .status()
            .context("Failed to remove stale link")?;
        if !status.success() {
            anyhow::bail!("Failed to remove existing conf file");
        }
    }

    // Create symlink via sudo ln -s
    let status = Command::new("sudo")
        .args([
            "ln",
            "-s",
            &src.to_string_lossy().to_string(),
            &dst.to_string_lossy().to_string(),
        ])
        .status()
        .context("Failed to create symlink")?;

    if !status.success() {
        anyhow::bail!("Failed to symlink rusters.conf (privilege error)");
    }

    // Reload the daemon
    let status = Command::new("sudo")
        .args(["systemctl", "reload", daemon])
        .status()
        .context("Failed to reload daemon")?;

    if !status.success() {
        println!(
            "{} Daemon reload failed. Reload manually: {}",
            "⚠".yellow().bold(),
            format!("sudo systemctl reload {daemon}").cyan()
        );
    }

    Ok(())
}

#[cfg(windows)]
fn link_windows(
    src: &PathBuf,
    dst: &PathBuf,
    conf_dir: &PathBuf,
    daemon: &str,
) -> Result<()> {
    use std::fs;

    fs::create_dir_all(conf_dir)
        .with_context(|| format!("Could not create {}", conf_dir.display()))?;

    fs::copy(src, dst)
        .with_context(|| format!("Could not copy rusters.conf to {}", dst.display()))?;

    println!(
        "{} Copied (symlinks require admin on Windows): {}",
        "ℹ".blue().bold(),
        dst.display()
    );

    // Restart via net stop / net start (Apache service in XAMPP is usually "Apache2.4")
    let service = "Apache2.4";
    let _ = Command::new("net").args(["stop", service]).status();
    let _ = Command::new("net").args(["start", service]).status();

    Ok(())
}
