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

use crate::consumer::router::create_core_consumer_router;
use crate::provider::router::create_core_provider_router;
use axum::serve;
use rainbow_auth::ssi::consumer::setup::AuthConsumerApplication;
use rainbow_auth::ssi::provider::setup::AuthProviderApplication;
use rainbow_common::config::traits::{MonoConfigTrait, RoleTrait};
use rainbow_common::config::types::roles::RoleConfig;
use rainbow_common::config::types::HostType;
use rainbow_common::config::ApplicationConfig;
use rainbow_common::well_known::WellKnownRoot;
use tokio::net::TcpListener;
use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;

pub struct CoreHttpWorker;

impl CoreHttpWorker {
    pub async fn spawn(config: &ApplicationConfig, token: &CancellationToken) -> anyhow::Result<JoinHandle<()>> {
        // message
        let server_message = format!(
            "Starting Dataspace http server in {}",
            config.get_mono_host()
        );
        tracing::info!("{}", server_message);
        // router
        let router = match config.monolith().get_role() {
            RoleConfig::Consumer => create_core_consumer_router(config).await,
            RoleConfig::Provider => create_core_provider_router(config).await,
            _ => {
                panic!("Unsupported role");
            }
        };
        // config
        let host = if config.is_mono_local() { "127.0.0.1" } else { "0.0.0.0" };
        let port = config.get_weird_mono_port();
        let addr = format!("{}{}", host, port);
        // listener
        let listener = TcpListener::bind(&addr).await?;
        // gracefully cancelation token
        let token = token.clone();
        let handle = tokio::spawn(async move {
            let server = serve(listener, router).with_graceful_shutdown(async move {
                token.cancelled().await;
                tracing::info!("HTTP Service received shutdown signal, draining connections...");
            });
            match server.await {
                Ok(_) => tracing::info!("HTTP Service stopped successfully"),
                Err(e) => tracing::error!("HTTP Service crashed: {}", e),
            }
        });

        Ok(handle)
    }
}
