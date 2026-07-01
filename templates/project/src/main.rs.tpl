//! # {{ PROJECT_NAME }}
//!
//! Entry point for the Rusters backend.
//! All routes are registered here and served via the multiplexer on port {{ BACKEND_PORT }}.

use std::sync::Arc;

use anyhow::Result;
use rusters_core::{config::RustersConfig, router::RustersRouter, TemplateEngine};

mod controllers;
mod database;
mod middleware;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialise tracing
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    // Load configuration from .env
    let config = Arc::new(RustersConfig::load()?);

    // Initialise the template engine with project context
    let mut template_engine = TemplateEngine::new();
    template_engine.set("PROJECT_NAME", "{{ PROJECT_NAME }}");
    template_engine.set("ENVIRONMENT", format!("{:?}", config.core.environment));

    // Build the router
    let router = RustersRouter::new()
        .route(
            "/api/users",
            axum::routing::get(controllers::user::list_users)
                .post(controllers::user::create_user),
        )
        .route(
            "/api/users/:id",
            axum::routing::get(controllers::user::get_user)
                .put(controllers::user::update_user)
                .delete(controllers::user::delete_user),
        );

    // Start the server
    router.serve(config).await?;

    Ok(())
}
