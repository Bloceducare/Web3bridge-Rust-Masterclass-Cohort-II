use crate::templates::{axum, actix};



#[derive(Debug)]
pub enum Framework {
    Axum,
    Actix,
}

impl Framework {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "axum" => Some(Framework::Axum),
            "actix" => Some(Framework::Actix),
            _ => None,
        }
    }
    pub fn display_name(&self) -> &str {
        match self {
            Framework::Axum => "Axum",
            Framework::Actix => "Actix-web",
        }
    }

    // Returns the Cargo.toml [dependencies] section for each framework.
    pub fn dependencies(&self) -> String {
        match self {
            Framework::Axum => {
                r#"[dependencies]
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
"#
                .to_string()
            }

            Framework::Actix => {
                r#"[dependencies]
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
"#
                .to_string()
            }
        }
    }

    // Returns the content of main.rs for each framework.
    pub fn main_rs_content(&self, project_name: &str) -> String {
        match self {
            Framework::Axum => format!(
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

    tracing::info!("Starting {} server...", "{project_name}");

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
            ),

            Framework::Actix => format!(
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

    log::info!("Starting {} server...", "{project_name}");
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
            ),
        }
    }

    // Returns the router/mod.rs content
    pub fn router_mod_content(&self) -> String {
        match self {
            Framework::Axum => {
                r#"use axum::Router;
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
"#
                .to_string()
            }

            Framework::Actix => {
                r#"use actix_web::web;
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
"#
                .to_string()
            }
        }
    }


    // Returns the handlers/mod.rs content
    pub fn handlers_mod_content(&self) -> &'static str {
        match self {
            Framework::Axum => axum::HANDLERS_MOD,
            Framework::Actix => actix::HANDLERS_MOD,
        }
    }

    // Returns the .env content
    pub fn env_content(&self) -> &'static str {
        match self {
            Framework::Axum => "RUST_LOG=info\nPORT=3000\n",
            Framework::Actix => "RUST_LOG=info\nPORT=8080\n",
        }
    }
}