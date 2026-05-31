//! SchemaForge backend entrypoint.

use std::net::SocketAddr;

use schema_forge::{config::Config, routes, state::AppState};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Load `.env` if present (ignored in production where env is injected).
    let _ = dotenvy::dotenv();

    // Structured logging; control verbosity with RUST_LOG (e.g. `info,schema_forge=debug`).
    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let config = Config::from_env()?;
    tracing::info!(?config, "starting SchemaForge backend");

    // Build shared application state (DB pool, schema engine, Kafka producer).
    // Connections are lazy, so startup succeeds even before Postgres/Kafka are up.
    let state = AppState::init(&config).await?;

    let app = routes::router(state).into_make_service_with_connect_info::<SocketAddr>();

    let addr: SocketAddr = config.bind_addr.parse()?;
    let listener = tokio::net::TcpListener::bind(addr).await?;
    tracing::info!(%addr, "listening");

    axum::serve(listener, app).await?;
    Ok(())
}
