


pub mod axum;
pub mod actix;


pub const SERVICES_MOD: &str = r#"// services/mod.rs
// Services contain your BUSINESS LOGIC — the core of your application.
// They are framework-agnostic: no Axum/Actix types here.
//
// USAGE PATTERN:
// Create submodules for each domain, e.g.:
//   pub mod user_service;
//   pub mod auth_service;
//
// Services are called FROM handlers. Example:
//   pub async fn find_user_by_id(id: u64) -> Option<crate::models::User> {
//       // database query logic here
//       None
//   }
"#;

pub const MODELS_MOD: &str = r#"// models/mod.rs
// Models define your DATA STRUCTURES — the shape of your domain objects.
// Use serde's Serialize/Deserialize to convert to/from JSON automatically.
//
// USAGE PATTERN:
// Create submodules for each entity, e.g.:
//   pub mod user;
//   pub mod post;
//
// Example model:
//   use serde::{Deserialize, Serialize};
//
//   #[derive(Debug, Serialize, Deserialize)]
//   pub struct User {
//       pub id: u64,
//       pub name: String,
//       pub email: String,
//   }
"#;

pub const GITIGNORE: &str = "/target\n.env\n";