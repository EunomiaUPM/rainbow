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

use crate::http::version;

pub async fn start_provider_server(host: &Option<String>, url: &Option<String>) -> Result<()> {
    info!("Starting provider server...");

    // config stuff
    let host = host.clone().unwrap_or("localhost".to_owned());
    let url = url.clone().unwrap_or("1234".to_owned());

    // create routing system
    let server = Router::new()
        .merge(version::router())
        .layer(TraceLayer::new_for_http());

    // start server
    let listener = TcpListener::bind(format!("{}:{}", host, url)).await?;
    serve(listener, server).await?;

    Ok(())
}