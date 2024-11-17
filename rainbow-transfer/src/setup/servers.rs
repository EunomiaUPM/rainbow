use crate::consumer::http::server::create_consumer_router;
use crate::provider::http::server::create_provider_router;
use crate::setup::config::GLOBAL_CONFIG;
use axum::serve;
use tokio::net::TcpListener;
use tracing::info;

pub async fn start_provider_server() -> anyhow::Result<()> {
    // config stuff
    let config = GLOBAL_CONFIG.get().unwrap();
    let server_message = format!(
        "Starting provider server in http://{}:{}",
        config.host_url, config.host_port
    );
    info!("{}", server_message);
    let url = config.host_url.clone().replace("http://", "");

    // start server
    let listener = TcpListener::bind(format!("{}:{}", url, config.host_port)).await?;
    serve(listener, create_provider_router().await).await?;

    Ok(())
}

pub async fn start_consumer_server() -> anyhow::Result<()> {
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
