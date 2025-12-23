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


use rainbow_common::boot::shutdown::shutdown_signal;
use rainbow_common::boot::BootstrapServiceTrait;
use rainbow_common::config::services::TransferConfig;
use rainbow_common::config::traits::ConfigLoader;
use rainbow_common::config::types::roles::RoleConfig;
use tokio::signal;
use tokio_util::sync::CancellationToken;
use rainbow_common::config::ApplicationConfig;
use crate::setup::CoreHttpWorker;

pub struct CoreBoot;

#[async_trait::async_trait]
impl BootstrapServiceTrait for CoreBoot {
    type Config = ApplicationConfig;
    async fn load_config(role_config: RoleConfig, env_file: Option<String>) -> anyhow::Result<Self::Config> {
        let config = Self::Config::load(role_config, env_file)?;
        let table = json_to_table::json_to_table(&serde_json::to_value(&config.monolith())?).collapse().to_string();
        tracing::info!("Current Monolith Dataspace Agent Config:\n{}", table);
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
        participant_id: Option<String>,
        catalog_id: Option<String>,
    ) -> anyhow::Result<()> {
        let cancel_token = CancellationToken::new();
        // worker http
        tracing::info!("Spawning HTTP subsystem...");
        let http_handle = CoreHttpWorker::spawn(config, &cancel_token).await?;
        // shutdown loop
        let shutdown_signal = shutdown_signal();
        tokio::select! {
            _ = shutdown_signal => {
                tracing::warn!("Shutdown signal received from OS.");
            }
            _ = async { http_handle.await } => {
                tracing::error!("HTTP subsystem failed or stopped unexpectedly!");
            }
        }
        // teardown
        tracing::info!("Initiating graceful shutdown sequence...");
        cancel_token.cancel();
        tracing::info!("System shut down gracefully.");
        Ok(())
    }
}