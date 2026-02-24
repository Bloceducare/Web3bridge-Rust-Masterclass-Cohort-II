



pub const DEPENDENCIES: &str = r#"[dependencies]
# Core web framework built on Tokio — fast and ergonomic
axum = "0.7"

# Async runtime — required for all async Rust code
tokio = { version = "1", features = ["full"] }

# Tower middleware ecosystem (axum is built on top of tower)
tower = "0.4"
tower-http = { version = "0.5", features = ["cors", "trace"] }

# Serialization / Deserialization — JSON support
serde = { version = "1", features = ["derive"] }
serde_json = "1"

# Environment variable management
dotenv = "0.15"

# Logging
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }
"#;


pub fn main_rs(project_name: &str) -> String {
    format!(
        r#"mod handlers;
mod models;
mod router;
mod services;

use tracing_subscriber::{{EnvFilter, fmt}};

// CONCEPT: #[tokio::main] is a macro that sets up the Tokio async runtime.
// Without it, you can't use `async fn main()`.
#[tokio::main]
async fn main() {{
    // Initialize tracing/logging
    fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    // Load environment variables from .env file
    dotenv::dotenv().ok();

    tracing::info!("Starting {project_name} server...");

    // Build the router — all routes are registered in router/mod.rs
    let app = router::create_router();

    // Bind to address and start serving
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Failed to bind to port 3000");

    tracing::info!("🚀 Server running at http://0.0.0.0:3000");

    axum::serve(listener, app)
        .await
        .expect("Server error");
}}
"#,
        project_name = project_name
    )
}

pub const ROUTER_MOD: &str = r#"use axum::Router;
// Import your handlers here as you build them out.
// Example: use crate::handlers::health::health_check;

/// Creates and returns the main application Router.
/// Add all your routes here using `.route(path, method_handler)`.
pub fn create_router() -> Router {
    Router::new()
    // Example routes — uncomment and expand as needed:
    // .route("/health", axum::routing::get(handlers::health_check))
    // .route("/api/v1/users", axum::routing::get(handlers::users::list_users))
}
"#;

pub const HANDLERS_MOD: &str = r#"// handlers/mod.rs
// This module is the entry point for all your HTTP request handlers.
//
// USAGE PATTERN:
// Create submodules for each resource, e.g.:
//   pub mod health;
//   pub mod users;
//
// Each handler receives an Axum extractor and returns a response.
// Example:
//   pub async fn health_check() -> impl axum::response::IntoResponse {
//       axum::http::StatusCode::OK
//   }
"#;