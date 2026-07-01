//! # wizard
//!
//! Interactive wizard primitives for arrow-key navigation.
//!
//! All selection menus in Rusters use `dialoguer::Select` / `dialoguer::MultiSelect`
//! so the user never needs to type numbers — only arrow keys and Enter.

use anyhow::Result;
use console::style;
use dialoguer::{theme::ColorfulTheme, Confirm, Input, Select};

/// Display a single-select menu with arrow-key navigation.
///
/// Returns the index of the selected item.
pub fn select(prompt: &str, items: &[&str]) -> Result<usize> {
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt)
        .items(items)
        .default(0)
        .interact()?;
    Ok(selection)
}

/// Display a yes/no confirmation prompt.
pub fn confirm(prompt: &str, default: bool) -> Result<bool> {
    let result = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt)
        .default(default)
        .interact()?;
    Ok(result)
}

/// Prompt for a text string with optional default.
pub fn input(prompt: &str, default: Option<&str>) -> Result<String> {
    let theme = ColorfulTheme::default();
    let mut input_builder = Input::<String>::with_theme(&theme)
        .with_prompt(prompt);

    if let Some(d) = default {
        input_builder = input_builder.default(d.to_string());
    }

    let result = input_builder.interact_text()?;
    Ok(result)
}

/// Print a styled section header to stdout.
pub fn section(title: &str) {
    println!("\n{}", style(format!("━━━ {title} ━━━")).cyan().bold());
}

/// Print a success message.
pub fn success(msg: &str) {
    println!("{} {}", style("✔").green().bold(), msg);
}

/// Print an info message.
pub fn info(msg: &str) {
    println!("{} {}", style("ℹ").blue().bold(), msg);
}

/// Print a warning.
pub fn warn(msg: &str) {
    println!("{} {}", style("⚠").yellow().bold(), msg);
}

/// Print an error.
pub fn error(msg: &str) {
    eprintln!("{} {}", style("✖").red().bold(), msg);
}

/// HTTP server choices presented to the user.
pub const HTTP_SERVERS: &[&str] = &["Apache", "Nginx", "Caddy"];

/// Database driver choices presented to the user.
pub const DB_DRIVERS: &[&str] = &["SQLite (local file)", "MySQL", "PostgreSQL"];

/// Frontend framework choices presented to the user.
pub const FRONTENDS: &[&str] = &["Vue 3", "React (TSX)"];

/// Run the `cargo ignite` interactive wizard.
///
/// Returns the user's selections as a tuple:
/// `(http_server_index, db_driver_index, frontend_index)`
pub fn run_ignite_wizard(project_name: &str) -> Result<(usize, usize, usize)> {
    println!(
        "\n{}",
        style("🚀  Welcome to Rusters — cargo ignite").magenta().bold()
    );
    println!(
        "   Initialising project: {}\n",
        style(project_name).white().bold()
    );

    section("HTTP Server");
    info("Select your deployment HTTP server:");
    let http = select("HTTP Server", HTTP_SERVERS)?;

    section("Database");
    info("Select your database driver:");
    let db = select("Database", DB_DRIVERS)?;

    section("Frontend");
    info("Select your frontend framework:");
    let fe = select("Frontend", FRONTENDS)?;

    println!();
    success(&format!(
        "Configuration locked — {}, {}, {}",
        HTTP_SERVERS[http],
        DB_DRIVERS[db],
        FRONTENDS[fe]
    ));

    Ok((http, db, fe))
}
