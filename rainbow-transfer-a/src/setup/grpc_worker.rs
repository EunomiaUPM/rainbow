use axum::{serve, Router};
use tokio::net::TcpListener;
use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;
use rainbow_common::config::provider_config::{ApplicationProviderConfig, ApplicationProviderConfigTrait};

pub struct TransferGrpcWorker {}
impl TransferGrpcWorker {
    pub async fn spawn(
        config: &ApplicationProviderConfig,
        token: &CancellationToken,
    ) -> anyhow::Result<JoinHandle<()>> {
        let router = Self::create_root_grpc_router(&config)?;
        let host = if config.get_environment_scenario() { "127.0.0.1" } else { "0.0.0.0" };
        // let port = config.get_raw_transfer_process_host().clone().expect("no host").port;
        let addr = format!("{}:0", host);

        let listener = TcpListener::bind(&addr).await?;
        tracing::info!("GRPC Transfer Service running on {}", addr);

        let token = token.clone();
        let handle = tokio::spawn(async move {
            let server = serve(listener, router)
                .with_graceful_shutdown(async move {
                    token.cancelled().await;
                    tracing::info!("GRPC Service received shutdown signal, draining connections...");
                });
            match server.await {
                Ok(_) => tracing::info!("GRPC Service stopped successfully"),
                Err(e) => tracing::error!("GRPC Service crashed: {}", e),
            }
        });

        Ok(handle)
    }
    pub fn create_root_grpc_router(_config: &ApplicationProviderConfig) -> anyhow::Result<Router> {
        let router = Router::new();
        Ok(router)
    }
}