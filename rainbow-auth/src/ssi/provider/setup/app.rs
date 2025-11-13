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
use crate::ssi::common::services::client::basic::BasicClientService;
use crate::ssi::common::services::wallet::waltid::config::WaltIdConfig;
use crate::ssi::common::services::wallet::waltid::WaltIdService;
use crate::ssi::provider::config::{AuthProviderConfig, AuthProviderConfigTrait};
use crate::ssi::provider::core::AuthProvider;
use crate::ssi::provider::http::AuthProviderRouter;
use axum::serve;
use rainbow_db::auth::provider::factory::factory::AuthProviderRepoForSql;
use sea_orm::Database;
use std::sync::Arc;
use tokio::net::TcpListener;
use tracing::info;
pub struct AuthProviderApplication;

impl AuthProviderApplication {
    pub async fn run(config: &AuthProviderConfig) -> anyhow::Result<()> {
        // CONFIGS
        let db_connection = Database::connect(config.get_full_db_url()).await.expect("Database can't connect");
        let core_config = Arc::new(config.clone());
        let waltid_config = WaltIdConfig::from(config.clone());

        // SERVICES
        let client_service = Arc::new(BasicClientService::new());
        let wallet_service = Arc::new(WaltIdService::new(client_service.clone(), waltid_config));
        let repo_service = Arc::new(AuthProviderRepoForSql::create_repo(db_connection));

        // CORE
        let provider = Arc::new(AuthProvider::new(
            wallet_service,
            repo_service,
            client_service,
            core_config,
        ));

        // ROUTER
        let router = AuthProviderRouter::new(provider).router();

        // Init server
        let server_message = format!("Starting Auth Provider server in {}", config.get_host());
        info!("{}", server_message);

        let listener = match config.is_local() {
            true => TcpListener::bind(format!("127.0.0.1{}", config.get_weird_port())).await?,
            false => TcpListener::bind(format!("0.0.0.0{}", config.get_weird_port())).await?,
        };

        serve(listener, router).await?;

        Ok(())
    }
}
