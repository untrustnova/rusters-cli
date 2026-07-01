//! # flags
//!
//! Global CLI flags shared across all Rusters subcommands.
//!
//! Every binary in `rusters-cli` parses these flags before executing
//! its primary logic.

use clap::Args;

/// Global flags available on every Rusters subcommand.
///
/// Usage (in each binary):
/// ```rust,no_run
/// use rusters_cli::flags::GlobalFlags;
/// use clap::Parser;
///
/// #[derive(Parser)]
/// struct Cli {
///     #[command(flatten)]
///     global: GlobalFlags,
///     // ... command-specific args
/// }
/// ```
#[derive(Args, Debug, Clone, Default)]
pub struct GlobalFlags {
    /// Output the current version of the Rusters toolchain and exit.
    #[arg(long, global = true)]
    pub version: bool,

    /// Drop all existing states/tables and rebuild from scratch.
    /// Applicable to: cargo-push, cargo-seed.
    #[arg(long, global = true)]
    pub fresh: bool,

    /// Automatically trigger seeder logic after the primary command succeeds.
    /// Applicable to: cargo-push.
    #[arg(long = "with-seed", global = true)]
    pub with_seed: bool,

    /// Output comprehensive debugging logs during execution.
    #[arg(long, global = true)]
    pub verbose: bool,
}

impl GlobalFlags {
    /// Initialise the tracing subscriber based on the `--verbose` flag.
    pub fn init_tracing(&self) {
        let level = if self.verbose { "debug" } else { "info" };
        tracing_subscriber::fmt()
            .with_env_filter(
                tracing_subscriber::EnvFilter::try_from_default_env()
                    .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new(level)),
            )
            .init();
    }

    /// Print the version string and return `true` if `--version` was requested.
    pub fn maybe_print_version(&self) -> bool {
        if self.version {
            println!(
                "rusters-cli v{} (rusters-core v{})",
                env!("CARGO_PKG_VERSION"),
                rusters_core::VERSION,
            );
            true
        } else {
            false
        }
    }
}
