use crate::auth::router as auth_router;
use crate::config::GLOBAL_CONFIG;
use anyhow::bail;
use axum::{serve, Router};
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;
use tracing::info;

pub mod router;

pub async fn start_provider_auth_server() -> anyhow::Result<()> {
    info!("Starting provider auth server...");

    // config stuff
    let config = GLOBAL_CONFIG.get().unwrap();

    if config.auth_url.is_none() || config.auth_port.is_none() {
        bail!("Auth URL is required!");
    }

    // create routing system
    let server = Router::new()
        .merge(auth_router::router())
        .layer(TraceLayer::new_for_http());

    // start server
    let listener = TcpListener::bind(format!(
        "{}:{}",
        config.auth_url.clone().unwrap(),
        config.auth_port.clone().unwrap()
    ))
        .await?;
    serve(listener, server).await?;

    Ok(())
}
