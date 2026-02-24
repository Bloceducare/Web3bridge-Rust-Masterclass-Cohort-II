
mod app;
mod models;
mod router;
mod handlers;
mod error;
mod services;
mod config;

// use handlers::{get_authors, get_author_by_id};
use serde::Serialize;
use axum::{response::IntoResponse, Json, http::StatusCode};


use tracing_subscriber::{EnvFilter, fmt};


fn trace_setup() {
    let env_filter: EnvFilter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    fmt()
        .with_target(false)
        .pretty()
        .with_env_filter(env_filter)
        .init();

    
}















// async fn connect_db() -> Result<sqlx::PgPool, AppError> {
//     let db_url = get_env_vars::<String>("DATABASE_URL".to_string()).unwrap();

//     let pg_pool: sqlx::Pool<sqlx::Postgres> = sqlx::postgres::PgPoolOptions::new()
//         .connect(&db_url)
//         .await
//         .map_err(|_| AppError::NotFound)?;

//     sqlx::migrate!("./migrations")
//         .run(&pg_pool)
//         .await
//         .map_err(|err| AppError::InternalServerError(err.to_string()))?;

//     // if let Ok(db_url) = db_url {
//     //     tracing::info!("Connecting to database at: {}", db_url);
//     // } else {
//     //     tracing::error!("Database URL is not set in environment variables");
//     // }

//     Ok(pg_pool)
// }






#[derive(Serialize)]
struct HealthResponse {
    status: String,
    message : String
}

async fn root_handler() -> impl IntoResponse {
    let response =  HealthResponse {
        status: "ok". to_string(),
        message: "Welcome to my Blog Api".to_string(),
    }; (StatusCode::OK, Json(response))
}
















#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    trace_setup();
    app::serve().await;
}