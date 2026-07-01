//! # scaffold
//!
//! Project scaffolding engine for `cargo ignite`.
//!
//! Walks the embedded template directory, renders `.tpl` files through the
//! `TemplateEngine`, and copies static files verbatim into the new project
//! directory.

use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use rusters_core::TemplateEngine;

/// Context passed from the wizard into the scaffold engine.
#[derive(Debug, Clone)]
pub struct ScaffoldContext {
    pub project_name: String,
    pub project_id: String,
    pub http_server: String,   // "apache" | "nginx" | "caddy"
    pub db_driver: String,     // "sqlite" | "mysql" | "postgres"
    pub frontend: String,      // "vue" | "react"
    pub backend_port: u16,
    pub frontend_port: u16,
}

impl ScaffoldContext {
    /// Build the template engine context from the scaffold settings.
    pub fn to_engine(&self) -> TemplateEngine {
        let mut engine = TemplateEngine::new();
        engine.set("PROJECT_NAME", &self.project_name);
        engine.set("PROJECT_ID", &self.project_id);
        engine.set("HTTP_SERVER", &self.http_server);
        engine.set("DB_DRIVER", &self.db_driver);
        engine.set("FRONTEND", &self.frontend);
        engine.set("BACKEND_PORT", self.backend_port.to_string());
        engine.set("FRONTEND_PORT", self.frontend_port.to_string());
        // Conditional flags used by templates
        engine.set("IS_APACHE", if self.http_server == "apache" { "true" } else { "false" });
        engine.set("IS_SQLITE", if self.db_driver == "sqlite" { "true" } else { "false" });
        engine.set("IS_VUE", if self.frontend == "vue" { "true" } else { "false" });
        engine.set("IS_REACT", if self.frontend == "react" { "true" } else { "false" });
        engine
    }
}

/// Path to the bundled templates directory.
///
/// During development this resolves relative to the workspace root.
/// When installed, templates are embedded at compile-time via `include_dir!`.
pub fn templates_dir() -> PathBuf {
    // Walk upwards from the binary until we find templates/project/
    let mut dir = std::env::current_exe()
        .unwrap_or_else(|_| PathBuf::from("."))
        .parent()
        .unwrap_or(Path::new("."))
        .to_path_buf();

    // Development: look for the workspace templates dir
    for _ in 0..8 {
        let candidate = dir.join("templates").join("project");
        if candidate.exists() {
            return candidate;
        }
        if let Some(parent) = dir.parent() {
            dir = parent.to_path_buf();
        } else {
            break;
        }
    }

    // Fallback — callers should handle the error
    PathBuf::from("templates/project")
}

/// Scaffold a new project into `target_dir`.
///
/// 1. Creates the target directory.
/// 2. Walks `templates_dir`.
/// 3. `.tpl` files → rendered through the template engine, written without the `.tpl` extension.
/// 4. Other files → copied verbatim.
/// 5. Conditional files (`.htaccess`, `App.vue`, `main.tsx`) only copied when appropriate.
pub fn scaffold(ctx: &ScaffoldContext, target_dir: &Path) -> Result<()> {
    fs::create_dir_all(target_dir)
        .with_context(|| format!("Failed to create project directory: {}", target_dir.display()))?;

    let templates = templates_dir();
    let engine = ctx.to_engine();

    walk_and_copy(&templates, target_dir, &engine, ctx)?;

    Ok(())
}

/// Recursively walk the template tree and copy/render into the target.
fn walk_and_copy(
    src_dir: &Path,
    dst_dir: &Path,
    engine: &TemplateEngine,
    ctx: &ScaffoldContext,
) -> Result<()> {
    for entry in fs::read_dir(src_dir)? {
        let entry = entry?;
        let src_path = entry.path();
        let name = entry.file_name();
        let name_str = name.to_string_lossy();

        // Skip conditional files that don't apply
        if should_skip(&name_str, ctx) {
            continue;
        }

        if src_path.is_dir() {
            let dst_path = dst_dir.join(&name);
            fs::create_dir_all(&dst_path)?;
            walk_and_copy(&src_path, &dst_path, engine, ctx)?;
        } else {
            let dst_name = if name_str.ends_with(".tpl") {
                name_str[..name_str.len() - 4].to_string()
            } else {
                name_str.to_string()
            };

            // Rename framework-specific package.json files
            let dst_name = match dst_name.as_str() {
                "package.react.json" => "package.json".to_string(),
                other => other.to_string(),
            };

            let dst_path = dst_dir.join(&dst_name);

            if name_str.ends_with(".tpl") {
                engine.render_file_to(&src_path, &dst_path)
                    .with_context(|| format!("Failed to render template: {}", src_path.display()))?;
            } else {
                fs::copy(&src_path, &dst_path)
                    .with_context(|| format!("Failed to copy file: {}", src_path.display()))?;
            }
        }
    }

    Ok(())
}

/// Determine whether a template file should be skipped for this project.
fn should_skip(name: &str, ctx: &ScaffoldContext) -> bool {
    match name {
        // Apache-only
        ".htaccess.tpl" => ctx.http_server != "apache",
        // Vue-only
        "App.vue" => ctx.frontend != "vue",
        // React-only
        "main.tsx" => ctx.frontend != "react",
        // Only include the correct package.json variant
        "package.json.tpl" => ctx.frontend != "vue",
        "package.react.json.tpl" => ctx.frontend != "react",
        // Nginx conf: only for nginx
        "rusters.conf.nginx.tpl" => ctx.http_server != "nginx",
        // Caddy conf: only for caddy
        "rusters.conf.caddy.tpl" => ctx.http_server != "caddy",
        // Apache conf: only for apache
        "rusters.conf.tpl" => ctx.http_server != "apache",
        _ => false,
    }
}
