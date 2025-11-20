use crate::setup::grpc_worker::TransferGrpcWorker;
use crate::setup::http_worker::TransferHttpWorker;
use rainbow_common::config::provider_config::{ApplicationProviderConfig};
use tokio::signal;
use tokio_util::sync::CancellationToken;

pub struct TransferApplication;
impl TransferApplication {
    pub async fn run(config: &ApplicationProviderConfig) -> anyhow::Result<()> {
        // TODO ApplicationProviderConfig for TransferModule (rodrigo's way)
        let cancel_token = CancellationToken::new();

        // worker http
        tracing::info!("Spawning HTTP subsystem...");
        let http_handle = TransferHttpWorker::spawn(config, &cancel_token).await?;
        // worker grpc
        tracing::info!("Spawning gRPC subsystem...");
        // TODO GRPC configuration in ApplicationProviderConfig with own port and host config
        let grpc_handle = TransferGrpcWorker::spawn(config, &cancel_token).await?;
        // shutdown loop
        let shutdown_signal = Self::shutdown_signal();
        tokio::select! {
            _ = shutdown_signal => {
                tracing::warn!("Shutdown signal received from OS.");
            }
            _ = async { http_handle.await } => {
                // TODO errors well done...
                tracing::error!("HTTP subsystem failed or stopped unexpectedly!");
            }
            _ = async { grpc_handle.await } => {
                tracing::error!("GRPC subsystem failed or stopped unexpectedly!");
            }
        }
        // teardown
        tracing::info!("Initiating graceful shutdown sequence...");
        cancel_token.cancel();
        tracing::info!("System shut down gracefully.");
        Ok(())
    }

    async fn shutdown_signal() -> anyhow::Result<()> {
        let ctrl_c = async {
            signal::ctrl_c().await.expect("failed to install Ctrl+C handler");
        };
        #[cfg(unix)]
        let terminate = async {
            signal::unix::signal(signal::unix::SignalKind::terminate())
                .expect("failed to install signal handler")
                .recv()
                .await;
        };
        #[cfg(not(unix))]
        let terminate = std::future::pending::<()>();
        tokio::select! {
            _ = ctrl_c => {},
            _ = terminate => {},
        }
        Ok(())
    }
}
