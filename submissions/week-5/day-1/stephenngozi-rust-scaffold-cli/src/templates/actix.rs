



pub const DEPENDENCIES: &str = r#"[dependencies]
# Core web framework — high-performance, battle-tested
actix-web = "4"

# Tokio async runtime (actix-web uses it under the hood)
tokio = { version = "1", features = ["full"] }

# Middleware utilities for actix-web
actix-cors = "0.7"

# Serialization / Deserialization — JSON support
serde = { version = "1", features = ["derive"] }
serde_json = "1"

# Environment variable management
dotenv = "0.15"

# Logging
env_logger = "0.11"
log = "0.4"
"#;


pub fn main_rs(project_name: &str) -> String {
    format!(
        r#"mod handlers;
mod models;
mod router;
mod services;

use actix_web::{{App, HttpServer}};

// CONCEPT: #[actix_web::main] sets up the Actix async runtime.
// Actix has its own runtime built on top of Tokio.
#[actix_web::main]
async fn main() -> std::io::Result<()> {{
    // Initialize logger
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    // Load environment variables from .env file
    dotenv::dotenv().ok();

    log::info!("Starting {project_name} server...");
    log::info!("🚀 Server running at http://0.0.0.0:8080");

    // HttpServer::new takes a closure that returns an App
    // The move || closure captures environment for thread safety
    HttpServer::new(move || {{
        App::new()
            .configure(router::configure_routes)
    }})
    .bind("0.0.0.0:8080")?
    .run()
    .await
}}
"#,
        project_name = project_name
    )
}

pub const ROUTER_MOD: &str = r#"use actix_web::web;
// Import your handlers here as you build them out.
// Example: use crate::handlers::health::health_check;

/// Configures all routes for the Actix-web application.
/// Called in main.rs inside App::new().configure(...)
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg
    // Example routes — uncomment and expand as needed:
    // .service(web::resource("/health").route(web::get().to(handlers::health_check)))
    // .service(
    //     web::scope("/api/v1")
    //         .service(web::resource("/users").route(web::get().to(handlers::users::list_users)))
    // )
    ;
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
// Each handler is an async function that returns an HttpResponse.
// Example:
//   pub async fn health_check() -> actix_web::HttpResponse {
//       actix_web::HttpResponse::Ok().finish()
//   }
"#;