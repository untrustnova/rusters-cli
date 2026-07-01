//! Database module entry.
//! Exposes connection helpers and the database pool.

pub mod connection;

// Re-export generated schema if it exists
#[cfg(feature = "sqlite")]
pub mod schema;
