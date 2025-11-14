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
use crate::ssi::common::services::vc_request::basic::config::VCRequesterConfig;
use crate::ssi::common::services::vc_request::basic::VCReqService;
use crate::ssi::common::services::wallet::waltid::config::WaltIdConfig;
use crate::ssi::common::services::wallet::waltid::WaltIdService;
use crate::ssi::consumer::config::AuthConsumerConfig;
use crate::ssi::consumer::setup::AuthConsumerApplication;
use crate::ssi::provider::config::{AuthProviderConfig, AuthProviderConfigTrait};
use crate::ssi::provider::core::AuthProvider;
use crate::ssi::provider::http::AuthProviderRouter;
use axum::{serve, Router};
use rainbow_common::config::consumer_config::ApplicationConsumerConfig;
use rainbow_common::config::provider_config::ApplicationProviderConfig;
use rainbow_db::auth::provider::factory::factory::AuthProviderRepoForSql;
use sea_orm::Database;
use std::sync::Arc;
use tokio::net::TcpListener;
use tracing::info;

pub struct AuthProviderApplication;

impl AuthProviderApplication {
    pub async fn create_router(config: &AuthProviderConfig) -> Router {
        // CONFIGS
        let db_connection = Database::connect(config.get_full_db_url()).await.expect("Database can't connect");
        let waltid_config = WaltIdConfig::from(config.clone());
        let vc_req_config = VCRequesterConfig::from(config.clone());

        // SERVICES
        let client_service = Arc::new(BasicClientService::new());
        let wallet_service = Arc::new(WaltIdService::new(client_service.clone(), waltid_config));
        let vc_req_service = Arc::new(VCReqService::new(client_service.clone(), vc_req_config));
        let repo_service = Arc::new(AuthProviderRepoForSql::create_repo(db_connection));

        // CORE
        let provider = Arc::new(AuthProvider::new(
            wallet_service,
            vc_req_service,
            repo_service,
            client_service,
            config.clone(),
        ));

        // ROUTER
        AuthProviderRouter::new(provider).router()
    }
    pub async fn run(config: &AuthProviderConfig) -> anyhow::Result<()> {
        let router = AuthProviderApplication::create_router(config).await;

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
    pub async fn create_router_4_core(config: ApplicationProviderConfig) -> Router {
        let config = AuthProviderConfig::from(config);
        AuthProviderApplication::create_router(&config).await
    }
}
