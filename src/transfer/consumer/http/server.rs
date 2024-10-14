use crate::config::GLOBAL_CONFIG;
use crate::transfer::common::misc_router;
use crate::transfer::consumer::http::router;
use crate::transfer::provider::http::middleware::{authentication_middleware, authorization_middleware};
use anyhow::Result;
use axum::{middleware, serve, Router};
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;
use tracing::info;

pub async fn create_consumer_router() -> Router {
    // create routing system
    let server = Router::new()
        .merge(misc_router::router())
        .merge(router::router())
        .layer(TraceLayer::new_for_http());
    server
}

pub async fn start_consumer_server() -> Result<()> {
    // config stuff
    let config = GLOBAL_CONFIG.get().unwrap();
    let server_message = format!(
        "Starting consumer server in http://{}:{}",
        config.host_url, config.host_port
    );
    info!("{}", server_message);
    let url = config.host_url.clone().replace("http://", "");

    // start server
    let listener = TcpListener::bind(format!("{}:{}", config.host_url, config.host_port)).await?;
    serve(listener, create_consumer_router().await).await?;

    Ok(())
}

pub async fn start_consumer_server_with_listener() -> Result<TcpListener> {
    // config stuff
    let config = GLOBAL_CONFIG.get().unwrap();
    let server_message = format!(
        "Starting consumer server in http://{}:{}",
        config.host_url, config.host_port
    );
    info!("{}", server_message);
    let url = config.host_url.clone().replace("http://", "");

    let listener = TcpListener::bind(format!("{}:{}", config.host_url, config.host_port)).await?;
    Ok(listener)
}
