use std::net::{Ipv6Addr, SocketAddr};
use tokio::net::TcpListener;
use crate::router::create_app;
use crate::error::AppError;


pub fn get_env_vars<T>(key: String) -> Result<T, AppError>
where
    T: std::str::FromStr,
    T::Err: std::fmt::Display,
{
    let value = std::env::var(&key).map_err(|_| AppError::MissingEnvironmentVarible(key))?;
    value
        .parse::<T>()
        .map_err(|err| AppError::ParsingError(err.to_string()))

    
}

pub async fn serve() {
    let port: u16 = get_env_vars::<u16>("PORT".to_string()).unwrap_or(8080);
    let addr = SocketAddr::from((Ipv6Addr::UNSPECIFIED, port));
    let listener = TcpListener::bind(addr)
        .await
        .expect("Failed to bind address");

    tracing::info!("Server is listening on: {}", listener.local_addr().unwrap());

    let app = create_app();
    axum::serve(listener, app).await.unwrap();
}