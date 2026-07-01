//! Database connection setup.
//!
//! Uses `rusters_core::db::DatabasePool` — agnostic to the driver configured
//! in `.env` (`[RUSTERS_DATABASE]` section).

use std::sync::Arc;

use anyhow::Result;
use rusters_core::{config::RustersConfig, db::DatabasePool};

/// Establish the database pool from the loaded project configuration.
pub async fn connect(config: &RustersConfig) -> Result<Arc<DatabasePool>> {
    let pool = DatabasePool::connect(&config.database).await?;
    Ok(Arc::new(pool))
}
