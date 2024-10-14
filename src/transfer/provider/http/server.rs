use crate::transfer::common::misc_router;
use crate::transfer::provider::http::middleware::{authentication_middleware, authorization_middleware};
use crate::transfer::provider::http::proxy;
use crate::transfer::provider::http::router;

use crate::config::GLOBAL_CONFIG;
use anyhow::Result;
use axum::{middleware, serve, Router};
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;
use tracing::info;

pub async fn create_provider_router() -> Router {
    // create routing system
    let server = Router::new()
        .merge(misc_router::router())
        .merge(router::router())
        .merge(proxy::router())
        .layer(middleware::from_fn(authorization_middleware)) // TODO put middleware where needed
        .layer(middleware::from_fn(authentication_middleware))
        .layer(TraceLayer::new_for_http());
    server
}

pub async fn start_provider_server() -> Result<()> {
    // config stuff
    let config = GLOBAL_CONFIG.get().unwrap();
    let server_message = format!(
        "Starting provider server in {}:{}",
        config.host_url, config.host_port
    );
    info!("{}", server_message);
    let url = config.host_url.clone().replace("http://", "");

    // start server
    let listener = TcpListener::bind(format!("{}:{}", url, config.host_port)).await?;
    serve(listener, create_provider_router().await).await?;

    Ok(())
}

pub async fn start_provider_server_with_listener() -> Result<TcpListener> {
    // config stuff
    let config = GLOBAL_CONFIG.get().unwrap();
    let server_message = format!(
        "Starting provider server in http://{}:{}",
        config.host_url, config.host_port
    );
    info!("{}", server_message);
    let url = config.host_url.clone().replace("http://", "");


    let listener = TcpListener::bind(format!("{}:{}", config.host_url, config.host_port)).await?;
    Ok(listener)
}
