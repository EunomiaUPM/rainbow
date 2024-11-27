use axum::body::to_bytes;
use axum::extract::Request;
use axum::http::StatusCode;
use axum::routing::{get, post};
use axum::{serve, Json, Router};
use std::env::args;
use std::net::SocketAddr;
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
        .route("/data-client", post(|body: String| async move {
            let data = serde_json::from_slice::<serde_json::Value>(body.as_bytes()).unwrap();
            let data_text = serde_json::to_string_pretty(&data).unwrap();
            info!("POST /data-client");
            info!("Receiving data from dataspace: \n{}\n", data_text);
            StatusCode::OK
        }))
        .layer(TraceLayer::new_for_http());

    // start server
    let addr = SocketAddr::from(([0, 0, 0, 0], port_number));
    let listener = TcpListener::bind(addr).await?;
    debug!("listening on {}", listener.local_addr()?);
    serve(listener, static_router).await?;

    Ok(())
}
