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

use crate::ssi::common::setup::app::SSIAuthHttpWorker;
use rainbow_common::boot::shutdown::shutdown_signal;
use rainbow_common::boot::BootstrapServiceTrait;
use rainbow_common::config::services::SsiAuthConfig;
use rainbow_common::config::traits::ConfigLoader;
use rainbow_common::config::types::roles::RoleConfig;
use tokio_util::sync::CancellationToken;

pub struct SSIAuthBoot;

#[async_trait::async_trait]
impl BootstrapServiceTrait for SSIAuthBoot {
    type Config = SsiAuthConfig;
    async fn load_config(role: RoleConfig, env_file: Option<String>) -> anyhow::Result<Self::Config> {
        let config = Self::Config::load(role, env_file);
        let table = json_to_table::json_to_table(&serde_json::to_value(&config)?).collapse().to_string();
        tracing::info!("Current SSI Auth module Config:\n{}", table);
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
        let http_handle = SSIAuthHttpWorker::spawn(config, &cancel_token).await?;
        // worker grpc
        // TODO implement grpc
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
