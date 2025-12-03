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
use crate::ssi::consumer::setup::AuthConsumerApplication;
use crate::ssi::provider::setup::AuthProviderApplication;
use axum::serve;
use rainbow_common::config::services::SsiAuthConfig;
use rainbow_common::config::traits::{HostConfigTrait, IsLocalTrait};
use rainbow_common::config::types::roles::RoleConfig;
use tokio::net::TcpListener;
use tracing::info;
use rainbow_common::config::types::HostType;

pub struct Application {}

impl Application {
    pub async fn run(role: RoleConfig, config: SsiAuthConfig) -> anyhow::Result<()> {
        let server_message = format!("Starting Auth Consumer server in {}", config.get_host(HostType::Http));
        info!("{}", server_message);

        let router = match role {
            RoleConfig::Consumer => AuthConsumerApplication::create_router(&config).await,
            RoleConfig::Provider => AuthProviderApplication::create_router(&config).await,
        };

        let listener = match config.is_local() {
            true => TcpListener::bind(format!("127.0.0.1{}", config.get_weird_port())).await?,
            false => TcpListener::bind(format!("0.0.0.0{}", config.get_weird_port())).await?,
        };

        serve(listener, router).await?;

        Ok(())
    }
}
