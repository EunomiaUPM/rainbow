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

pub async fn start_consumer_server(host: &Option<String>, url: &Option<String>) -> Result<()> {
    // config stuff
    let host = host.clone().unwrap_or("localhost".to_owned());
    let url = url.clone().unwrap_or("1235".to_owned());
    let server_message = format!("Starting consumer server in http://{}:{}", host, url);
    info!("{}", server_message);

    // start server
    let listener = TcpListener::bind(format!("{}:{}", host, url)).await?;
    serve(listener, create_consumer_router().await).await?;

    Ok(())
}

pub async fn start_consumer_server_with_listener(
    host: &Option<String>,
    url: &Option<String>,
) -> Result<TcpListener> {
    // config stuff
    let host = host.clone().unwrap_or("localhost".to_owned());
    let url = url.clone().unwrap_or("1235".to_owned());
    let server_message = format!("Starting consumer server in http://{}:{}", host, url);
    info!("{}", server_message);

    let listener = TcpListener::bind(format!("{}:{}", host, url)).await?;
    Ok(listener)
}
