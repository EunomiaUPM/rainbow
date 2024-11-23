use axum::routing::{get, post};
use axum::{serve, Json, Router};
use std::env::args;
use std::net::SocketAddr;
use axum::http::StatusCode;
use tokio::net::TcpListener;
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;
use tracing::{debug, info, Level};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args: Vec<String> = args().collect();
    let port_number = *&args
        .get(1)
        .unwrap_or(&"1237".to_string())
        .parse::<u16>()?;

    // logging
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .init();

    // static router
    let static_router = Router::new()
        .route("/data-client", post(|Json(input): Json<serde_json::Value>| async move {
            info!("POST /data-client");
            info!("Receiving data from dataspace: \n{}\n", serde_json::to_string_pretty(&input).unwrap());
            StatusCode::OK
        }))
        .layer(TraceLayer::new_for_http());

    // start server
    let addr = SocketAddr::from(([127, 0, 0, 1], port_number));
    let listener = TcpListener::bind(addr).await?;
    debug!("listening on {}", listener.local_addr()?);
    serve(listener, static_router).await?;

    Ok(())
}
