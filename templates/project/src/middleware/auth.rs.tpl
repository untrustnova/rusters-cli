//! Authentication middleware.
//!
//! Validates `Authorization: Bearer <token>` headers.
//!
//! ## Usage
//!
//! Wrap a protected router group:
//!
//! ```rust
//! use rusters_core::middleware::AuthLayer;
//!
//! let secret = std::env::var("API_SECRET").unwrap_or_default();
//!
//! let protected = Router::new()
//!     .route("/api/admin", get(admin_handler))
//!     .layer(AuthLayer::new(secret));
//! ```

#[allow(unused_imports)]
pub use rusters_core::middleware::AuthLayer;
