use std::env::args;
use std::net::SocketAddr;

use axum::{Router, serve};
use tokio::net::TcpListener;
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;
use tracing::{debug, Level};

static PARQUET_SAMPLES_FOLDER: &str = "../../file-transfer-tests/parquet";
static MAIN_ROUTE: &str = "/data-space";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args: Vec<String> = args().collect();
    let port_number = *&args
        .get(1)
        .unwrap_or(&"1236".to_string())
        .parse::<u16>()?;

    // logging
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .init();

    // static router
    let static_router = Router::new()
        .nest_service(MAIN_ROUTE, ServeDir::new(PARQUET_SAMPLES_FOLDER))
        .layer(TraceLayer::new_for_http());

    // start server
    let addr = SocketAddr::from(([127, 0, 0, 1], port_number));
    let listener = TcpListener::bind(addr).await?;
    debug!("listening on {}", listener.local_addr()?);
    serve(listener, static_router).await?;

    Ok(())
}
