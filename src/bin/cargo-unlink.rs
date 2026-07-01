//! # cargo-unlink
//!
//! Remove the project's `rusters.conf` symlink (or copy) from the system
//! HTTP server configuration directory, then gracefully reload the daemon.
//!
//! ## Usage
//!
//! ```sh
//! cargo unlink [--verbose]
//! ```

use std::path::PathBuf;
use std::process::Command;

use anyhow::{Context, Result};
use clap::Parser;
use colored::Colorize;
use rusters_cli::{flags::GlobalFlags, wizard};
use rusters_core::config::{HttpServer, RustersConfig};

#[derive(Parser, Debug)]
#[command(
    name = "cargo-unlink",
    bin_name = "cargo unlink",
    about = "Remove the rusters.conf link from the system HTTP server configuration directory"
)]
struct Cli {
    #[command(flatten)]
    global: GlobalFlags,
}

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args()
        .enumerate()
        .filter(|(i, a)| !(*i == 1 && a == "unlink"))
        .map(|(_, a)| a)
        .collect();

    let cli = Cli::parse_from(args);
    cli.global.init_tracing();

    if cli.global.maybe_print_version() {
        return Ok(());
    }

    let config = RustersConfig::load()
        .context("Could not load .env — are you inside a Rusters project?")?;

    let (conf_dir, daemon) = server_paths(&config.core.http_server);
    let project_id = &config.core.project_id;
    let dst = conf_dir.join(format!("{project_id}.conf"));

    println!(
        "\n{} {}",
        "🔗 cargo unlink".magenta().bold(),
        "— Removing server configuration link".white()
    );
    println!("   Target → {}", dst.display().to_string().cyan());
    println!("   Daemon → {}", daemon.yellow());
    println!();

    if !dst.exists() && dst.symlink_metadata().is_err() {
        wizard::warn("No conf file found at the expected location — nothing to remove.");
        return Ok(());
    }

    #[cfg(unix)]
    {
        let status = Command::new("sudo")
            .args(["rm", "-f", &dst.to_string_lossy()])
            .status()
            .context("Failed to remove conf file")?;

        if !status.success() {
            anyhow::bail!("Failed to remove conf file (privilege error)");
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
    }

    #[cfg(windows)]
    {
        std::fs::remove_file(&dst)
            .with_context(|| format!("Failed to remove {}", dst.display()))?;

        let service = "Apache2.4";
        let _ = Command::new("net").args(["stop", service]).status();
        let _ = Command::new("net").args(["start", service]).status();
    }

    wizard::success("Configuration unlinked and daemon reloaded.");

    Ok(())
}

fn server_paths(server: &HttpServer) -> (PathBuf, &'static str) {
    match server {
        HttpServer::Apache => (PathBuf::from("/etc/httpd/conf/rusters.d"), "httpd"),
        HttpServer::Nginx => (PathBuf::from("/etc/nginx/conf.d"), "nginx"),
        HttpServer::Caddy => (PathBuf::from("/etc/caddy/conf.d"), "caddy"),
    }
}
