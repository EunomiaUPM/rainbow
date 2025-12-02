/*
 *
 *  * Copyright (C) 2025 - Universidad Politécnica de Madrid - UPM
 *  *
 *  * This program is free software: you can redistribute it and/or modify
 *  * it under the terms of the GNU General Public License as published by
 *  * the Free Software Foundation, either version 3 of the License, or
 *  * (at your option) any later version.
 *  *
 *  * This program is distributed in the hope that it will be useful,
 *  * but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *  * GNU General Public License for more details.
 *  *
 *  * You should have received a copy of the GNU General Public License
 *  * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 *
 */

use crate::setup::http_worker::NegotiationHttpWorker;
use rainbow_common::config::provider_config::ApplicationProviderConfig;
use tokio::signal;
use tokio_util::sync::CancellationToken;

pub struct NegotiationAgentApplication;
impl NegotiationAgentApplication {
    pub async fn run(config: &ApplicationProviderConfig) -> anyhow::Result<()> {
        // TODO ApplicationProviderConfig for TransferModule (rodrigo's way)
        let cancel_token = CancellationToken::new();

        // worker http
        tracing::info!("Spawning HTTP subsystem...");
        let http_handle = NegotiationHttpWorker::spawn(config, &cancel_token).await?;
        // worker grpc
        tracing::info!("Spawning gRPC subsystem...");
        // TODO GRPC configuration in ApplicationProviderConfig with own port and host config
        // let grpc_handle = NegotiationGrpcWorker::spawn(config, &cancel_token).await?;
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
            // _ = async { grpc_handle.await } => {
            //     tracing::error!("GRPC subsystem failed or stopped unexpectedly!");
            // }
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
