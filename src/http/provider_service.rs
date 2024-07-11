use anyhow::Result;
use axum::{
    Router,
    serve,
};
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;
use tracing::info;

use crate::http::{auth_router, transfer_router, version_router};

pub async fn start_provider_server(host: &Option<String>, url: &Option<String>) -> Result<()> {
    info!("Starting provider server...");

    // config stuff
    let host = host.clone().unwrap_or("localhost".to_owned());
    let url = url.clone().unwrap_or("1234".to_owned());

    // create routing system
    let server = Router::new()
        .merge(version_router::router())
        .merge(transfer_router::router())
        .layer(TraceLayer::new_for_http());

    // start server
    let listener = TcpListener::bind(format!("{}:{}", host, url)).await?;
    serve(listener, server).await?;

    Ok(())
}

pub async fn start_provider_auth_server(host: &Option<String>, url: &Option<String>) -> Result<()> {
    info!("Starting provider auth server...");

    // config stuff
    let host = host.clone().unwrap_or("localhost".to_owned());
    let url = url.clone().unwrap_or("1235".to_owned());

    // create routing system
    let server = Router::new()
        .merge(auth_router::router())
        .layer(TraceLayer::new_for_http());

    // start server
    let listener = TcpListener::bind(format!("{}:{}", host, url)).await?;
    serve(listener, server).await?;

    Ok(())
}