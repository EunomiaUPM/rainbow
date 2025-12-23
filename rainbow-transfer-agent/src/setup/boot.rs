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

use crate::setup::grpc_worker::TransferGrpcWorker;
use crate::setup::http_worker::TransferHttpWorker;
use rainbow_common::boot::shutdown::shutdown_signal;
use rainbow_common::boot::BootstrapServiceTrait;
use rainbow_common::config::services::TransferConfig;
use rainbow_common::config::traits::ConfigLoader;
use rainbow_common::config::types::roles::RoleConfig;
use tokio_util::sync::CancellationToken;

pub struct TransferBoot;

#[async_trait::async_trait]
impl BootstrapServiceTrait for TransferBoot {
    type Config = TransferConfig;
    async fn load_config(role_config: RoleConfig, env_file: Option<String>) -> anyhow::Result<Self::Config> {
        let config = Self::Config::load(role_config, env_file);
        let table = json_to_table::json_to_table(&serde_json::to_value(&config)?).collapse().to_string();
        tracing::info!("Current Transfer Agent Config:\n{}", table);
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
    async fn start_services(
        config: &Self::Config,
        _participant_id: Option<String>,
        _catalog_id: Option<String>,
    ) -> anyhow::Result<()> {
        let cancel_token = CancellationToken::new();
        // worker http
        tracing::info!("Spawning HTTP subsystem...");
        let http_handle = TransferHttpWorker::spawn(config, &cancel_token).await?;
        // worker grpc
        tracing::info!("Spawning gRPC subsystem...");
        let grpc_handle = TransferGrpcWorker::spawn(config, &cancel_token).await?;
        // shutdown loop
        let shutdown_signal = shutdown_signal();
        tokio::select! {
            _ = shutdown_signal => {
                tracing::warn!("Shutdown signal received from OS.");
            }
            _ = async { http_handle.await } => {
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
}
