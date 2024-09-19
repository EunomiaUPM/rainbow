use crate::transfer::common::misc_router;
use crate::transfer::consumer::kickoff_router;
use crate::transfer::provider::control_plane;
use anyhow::Result;
use axum::{middleware, serve, Router};
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;
use tracing::info;

pub async fn start_consumer_server(host: &Option<String>, url: &Option<String>) -> Result<()> {
    // config stuff
    let host = host.clone().unwrap_or("localhost".to_owned());
    let url = url.clone().unwrap_or("1235".to_owned());
    let server_message = format!("Starting consumer server in http://{}:{}", host, url);
    info!("{}", server_message);

    // create routing system
    let server = Router::new()
        .merge(misc_router::router())
        .merge(kickoff_router::router())
        .merge(control_plane::router())
        .layer(TraceLayer::new_for_http());

    // start server
    let listener = TcpListener::bind(format!("{}:{}", host, url)).await?;
    serve(listener, server).await?;

    Ok(())
}