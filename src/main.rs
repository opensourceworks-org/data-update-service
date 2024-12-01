use anyhow::{Context, Result};
use axum::{routing::get, Router};
use dotenvy::dotenv;
use std::{env, net::SocketAddr};

mod routers;
use routers::health::health;

mod helpers;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt().init();
    dotenv().ok();

    let port: u16 = env::var("PORT")
        .context("PORT environment variable not set")?
        .parse()
        .context("PORT must be a valid u16")?;

    tracing::info!("router initialized, now listening on port {}", port);
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{port}"))
        .await
        .unwrap();

    let health_router = Router::new().route("/health", get(health));

    let app = Router::new().nest("/api/v1", health_router);

    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap();

    Ok(())
}
