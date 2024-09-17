use anyhow::Result;
use axum::{middleware, serve, Router};
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;
use tracing::info;

use crate::transfer::provider::control_plane;
use crate::transfer::provider::data_plane;
use crate::transfer::provider::middleware::{authentication_middleware, authorization_middleware};
use crate::transfer::provider::misc_router;

pub async fn start_provider_server(host: &Option<String>, url: &Option<String>) -> Result<()> {
    // config stuff
    let host = host.clone().unwrap_or("localhost".to_owned());
    let url = url.clone().unwrap_or("1234".to_owned());
    let server_message = format!("Starting provider server in http://{}:{}", host, url);
    info!("{}", server_message);

    // create routing system
    let server = Router::new()
        .merge(misc_router::router())
        .merge(control_plane::router())
        .merge(data_plane::router())
        .layer(middleware::from_fn(authorization_middleware)) // TODO put middleware where needed
        .layer(middleware::from_fn(authentication_middleware))
        .layer(TraceLayer::new_for_http());

    // start server
    let listener = TcpListener::bind(format!("{}:{}", host, url)).await?;
    serve(listener, server).await?;

    Ok(())
}
