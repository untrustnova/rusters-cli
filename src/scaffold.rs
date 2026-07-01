//! # scaffold
//!
//! Project scaffolding engine for `cargo ignite`.
//!
//! Templates are embedded directly into the binary at compile time via
//! `include_dir!`, so they are always available regardless of where the
//! binary is installed — no external template directory required.

use std::fs;
use std::path::Path;

use anyhow::{Context, Result};
use include_dir::{include_dir, Dir, DirEntry};
use rusters_core::TemplateEngine;

/// All project templates embedded into the binary at compile time.
static TEMPLATES: Dir<'static> = include_dir!("$CARGO_MANIFEST_DIR/templates/project");

/// Context passed from the wizard into the scaffold engine.
#[derive(Debug, Clone)]
pub struct ScaffoldContext {
    pub project_name: String,
    pub project_id: String,
    pub http_server: String, // "apache" | "nginx" | "caddy"
    pub db_driver: String,   // "sqlite" | "mysql" | "postgres"
    pub frontend: String,    // "vue" | "react"
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
        engine.set("ENTRY_FILE", if self.frontend == "vue" { "main.ts" } else { "main.tsx" });
        engine.set("BACKEND_PORT", self.backend_port.to_string());
        engine.set("FRONTEND_PORT", self.frontend_port.to_string());
        engine.set(
            "IS_APACHE",
            if self.http_server == "apache" {
                "true"
            } else {
                "false"
            },
        );
        engine.set(
            "IS_SQLITE",
            if self.db_driver == "sqlite" {
                "true"
            } else {
                "false"
            },
        );
        engine.set(
            "IS_VUE",
            if self.frontend == "vue" { "true" } else { "false" },
        );
        engine.set(
            "IS_REACT",
            if self.frontend == "react" {
                "true"
            } else {
                "false"
            },
        );
        engine
    }
}

/// Scaffold a new Rusters project into `target_dir`.
///
/// Uses the templates embedded in the binary — no external directory needed.
///
/// 1. Creates the target directory.
/// 2. Recursively walks the embedded `TEMPLATES` dir.
/// 3. `.tpl` files are rendered through `TemplateEngine`, written without the `.tpl` suffix.
/// 4. Other files are written verbatim.
/// 5. Conditional files (`.htaccess`, server confs, `App.vue`, `main.tsx`,
///    `package.json` variants) are only written when appropriate.
pub fn scaffold(ctx: &ScaffoldContext, target_dir: &Path) -> Result<()> {
    fs::create_dir_all(target_dir)
        .with_context(|| format!("Failed to create project directory: {}", target_dir.display()))?;

    let engine = ctx.to_engine();
    write_dir(&TEMPLATES, target_dir, &engine, ctx)
}

/// Recursively write an embedded `Dir` into the filesystem target.
fn write_dir(
    dir: &Dir<'_>,
    target: &Path,
    engine: &TemplateEngine,
    ctx: &ScaffoldContext,
) -> Result<()> {
    for entry in dir.entries() {
        match entry {
            DirEntry::Dir(sub_dir) => {
                let dir_name = sub_dir
                    .path()
                    .file_name()
                    .unwrap_or_default()
                    .to_string_lossy();
                let dst = target.join(dir_name.as_ref());
                fs::create_dir_all(&dst)?;
                write_dir(sub_dir, &dst, engine, ctx)?;
            }
            DirEntry::File(file) => {
                let file_name = file
                    .path()
                    .file_name()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .to_string();

                // Skip files that don't apply to this project's configuration
                if should_skip(&file_name, ctx) {
                    continue;
                }

                // Determine output filename
                let out_name = resolve_output_name(&file_name);
                let dst_path = target.join(&out_name);

                // Ensure parent dir exists
                if let Some(parent) = dst_path.parent() {
                    fs::create_dir_all(parent)?;
                }

                let raw = file.contents();

                if file_name.ends_with(".tpl") {
                    // Render as UTF-8 template
                    let content = std::str::from_utf8(raw)
                        .with_context(|| format!("Template file is not valid UTF-8: {file_name}"))?;
                    let rendered = engine.render(content);
                    fs::write(&dst_path, rendered.as_bytes())
                        .with_context(|| format!("Failed to write: {}", dst_path.display()))?;
                } else {
                    // Copy verbatim (handles binary files, images, etc.)
                    fs::write(&dst_path, raw)
                        .with_context(|| format!("Failed to write: {}", dst_path.display()))?;
                }
            }
        }
    }

    Ok(())
}

/// Strip `.tpl` suffix and handle framework-specific renames.
fn resolve_output_name(name: &str) -> String {
    // Strip .tpl suffix
    let base = if name.ends_with(".tpl") {
        &name[..name.len() - 4]
    } else {
        name
    };

    // Rename the React-specific package.json to the standard name
    match base {
      "package.react.json" => "package.json".to_string(),
      "vite.config.react.ts" => "vite.config.ts".to_string(),
      // Rename server conf files to the canonical rusters.conf
      "rusters.conf.nginx" => "rusters.conf".to_string(),
      "rusters.conf.caddy" => "rusters.conf".to_string(),
      other => other.to_string(),
    }
}

/// Returns `true` if a template file should NOT be written for this config.
fn should_skip(name: &str, ctx: &ScaffoldContext) -> bool {
    match name {
        // Apache-only files
        ".htaccess.tpl" => ctx.http_server != "apache",
        "rusters.conf.tpl" => ctx.http_server != "apache",

        // Nginx-only
        "rusters.conf.nginx.tpl" => ctx.http_server != "nginx",

        // Caddy-only
        "rusters.conf.caddy.tpl" => ctx.http_server != "caddy",

        // Vue-only
        "App.vue" => ctx.frontend != "vue",
        "package.json.tpl" => ctx.frontend != "vue",
        "vite.config.ts.tpl" => ctx.frontend != "vue",
        "main.ts.tpl" => ctx.frontend != "vue",

        // React-only
        "main.tsx" => ctx.frontend != "react",
        "package.react.json.tpl" => ctx.frontend != "react",
        "vite.config.react.ts.tpl" => ctx.frontend != "react",

        _ => false,
    }
}
