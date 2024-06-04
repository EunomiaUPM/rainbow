use std::convert::Infallible;
use std::net::SocketAddr;

use anyhow::Result;
use axum::{
    http::StatusCode,
    Json,
    Router,
    routing::{get, post}, serve,
};
use tokio::net::TcpListener;
use tower_http::trace::{self, TraceLayer};
use tracing::{info, Level};

async fn root() -> &'static str {
    info!("GET");
    "Hello, World!"
}

pub async fn start_provider_server(host: &Option<String>, url: &Option<String>) -> Result<()> {
    info!("Starting provider server...");

    // config stuff
    let host = host.as_deref().unwrap_or("localhost");
    let url = url.as_deref().unwrap_or("1234");

    // create routing system
    let transfer_router = Router::new()
        .route("/", get(root));

    let server = Router::new()
        .nest("/transfer", transfer_router)
        .layer(TraceLayer::new_for_http());

    // start server
    let listener = TcpListener::bind(format!("{}:{}", host, url)).await?;
    serve(listener, server).await?;

    Ok(())
}