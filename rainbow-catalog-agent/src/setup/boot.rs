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

use crate::setup::grpc_worker::CatalogGrpcWorker;
use crate::setup::http_worker::CatalogHttpWorker;
use rainbow_common::boot::shutdown::shutdown_signal;
use rainbow_common::boot::BootstrapServiceTrait;
use rainbow_common::config::services::{CatalogConfig, ContractsConfig, TransferConfig};
use rainbow_common::config::traits::ConfigLoader;
use rainbow_common::config::types::roles::RoleConfig;
use tokio::signal;
use tokio::sync::broadcast;
use tokio_util::sync::CancellationToken;

pub struct CatalogAgentBoot;

#[async_trait::async_trait]
impl BootstrapServiceTrait for CatalogAgentBoot {
    type Config = CatalogConfig;
    async fn load_config(role_config: RoleConfig, env_file: Option<String>) -> anyhow::Result<Self::Config> {
        let config = Self::Config::load(role_config, env_file);
        let table = json_to_table::json_to_table(&serde_json::to_value(&config)?).collapse().to_string();
        tracing::info!("Current Catalog Agent Config:\n{}", table);
        Ok(config)
    }
    fn enable_participant() -> bool {
        false
    }
    fn enable_catalog() -> bool {
        false
    }
    fn enable_dataservice() -> bool {
        false
    }
    async fn start_services_background(config: &Self::Config) -> anyhow::Result<broadcast::Sender<()>> {
        // thread control
        let (shutdown_tx, mut shutdown_rx) = broadcast::channel(1);
        let cancel_token = CancellationToken::new();

        // workers
        tracing::info!("Spawning HTTP subsystem...");
        let http_handle = CatalogHttpWorker::spawn(config, &cancel_token).await?;

        tracing::info!("Spawning gRPC subsystem...");
        let grpc_handle = CatalogGrpcWorker::spawn(config, &cancel_token).await?;

        // non-blocking thread
        let token_clone = cancel_token.clone();
        tokio::spawn(async move {
            tokio::select! {
                // ctrl+c
                _ = shutdown_rx.recv() => {
                    tracing::info!("Shutdown command received from Main Pipeline.");
                }
                _ = async { http_handle.await } => {
                    tracing::error!("HTTP subsystem failed or stopped unexpectedly!");
                }
                _ = async { grpc_handle.await } => {
                    tracing::error!("GRPC subsystem failed or stopped unexpectedly!");
                }
            }

            tracing::info!("Initiating internal graceful shutdown sequence...");
            token_clone.cancel();
            tracing::info!("Background services stopped.");
        });

        Ok(shutdown_tx)
    }
}
