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
use rainbow_common::config::traits::MonoConfigTrait;
use rainbow_common::config::types::roles::RoleConfig;
use rainbow_common::config::ApplicationConfig;
use tokio::net::TcpListener;
use tracing::info;

pub struct CoreApplication;

impl CoreApplication {
    pub async fn run(role: RoleConfig, config: &ApplicationConfig) -> anyhow::Result<()> {
        let port = config.get_weird_mono_port();
        let router = match role {
            RoleConfig::Consumer => create_core_consumer_router(config).await,
            RoleConfig::Provider => create_core_provider_router(config).await,
            _ => {
                panic!("Invalid role");
            }
        };

        let server_message = format!(
            "Starting core {} server in {}",
            role.to_string(),
            config.get_mono_host(),
        );
        info!("{}", server_message);

        let listener = match config.is_mono_local() {
            true => TcpListener::bind(format!("127.0.0.1{}", port)).await?,
            false => TcpListener::bind(format!("0.0.0.0{}", port)).await?,
        };
        serve(listener, router).await?;
        Ok(())
    }
}
