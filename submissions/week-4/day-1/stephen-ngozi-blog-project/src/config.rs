use crate::error::AppError;
use sqlx;

pub struct AppConfig {
    pub database_url: String,
    pub server_port: u16,
}

impl AppConfig {
    pub fn from_env() -> Result<Self, AppError> {
        let database_url = get_env_vars::<String>("DATABASE_URL")?;
        let server_port = get_env_vars::<u16>("SERVER_PORT")?;
        Ok(Self {
            database_url,
            server_port,
        })
    }
}

pub async fn connect_db(database_url: &str) -> Result<sqlx::PgPool, AppError> {
    println!("Connecting to database at {}", database_url);
    let pg_pool: sqlx::Pool<sqlx::Postgres> = sqlx::postgres::PgPoolOptions::new()
        .connect(database_url)
        .await
        .map_err(|err| AppError::NotFound(err.to_string()))?;

    sqlx::migrate!("./migrations")
        .run(&pg_pool)
        .await
        .map_err(|err| AppError::InternalServerError(err.to_string()))?;

    Ok(pg_pool)
}

// for set up and environment variables
fn get_env_vars<T>(key: &str) -> Result<T, AppError>
where
    T: std::str::FromStr,
    T::Err: std::fmt::Display,
{
    let value =
        std::env::var(key).map_err(|_| AppError::MissingEnvironmentVarible(key.to_string()))?;
    value.parse::<T>().map_err(|e| {
        AppError::ParsingError(format!(
            "Failed to parse environment variable '{}': {}",
            key, e
        ))
    })
}