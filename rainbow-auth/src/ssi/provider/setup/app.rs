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
use crate::ssi::common::services::callback::basic::BasicCallbackService;
use crate::ssi::common::services::client::basic::BasicClientService;
use crate::ssi::common::services::gaia_self_issuer::basic::config::GaiaSelfIssuerConfig;
use crate::ssi::common::services::gaia_self_issuer::basic::BasicGaiaSelfIssuer;
use crate::ssi::common::services::gaia_self_issuer::GaiaSelfIssuerTrait;
use crate::ssi::common::services::vc_requester::basic::config::VCRequesterConfig;
use crate::ssi::common::services::vc_requester::basic::VCReqService;
use crate::ssi::common::services::wallet::waltid::config::WaltIdConfig;
use crate::ssi::common::services::wallet::waltid::WaltIdService;
use crate::ssi::provider::config::{AuthProviderConfig, AuthProviderConfigTrait};
use crate::ssi::provider::core::AuthProvider;
use crate::ssi::provider::http::AuthProviderRouter;
use crate::ssi::provider::services::business::basic::config::BusinessConfig;
use crate::ssi::provider::services::business::basic::BasicBusinessService;
use crate::ssi::provider::services::gatekeeper::gnap::config::GnapGateKeeperConfig;
use crate::ssi::provider::services::gatekeeper::gnap::GnapGateKeeperService;
use crate::ssi::provider::services::repo::postgres::AuthProviderRepoForSql;
use crate::ssi::provider::services::verifier::basic_v1::config::VerifierConfig;
use crate::ssi::provider::services::verifier::basic_v1::VerifierService;
use axum::{serve, Router};
use rainbow_common::config::provider_config::ApplicationProviderConfig;
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
        let gatekeeper_config = GnapGateKeeperConfig::from(config.clone());
        let verifier_config = VerifierConfig::from(config.clone());
        let business_config = BusinessConfig::from(config.clone());
        let core_config = Arc::new(config.clone());

        // SERVICES
        let client_service = Arc::new(BasicClientService::new());
        let wallet_service = Arc::new(WaltIdService::new(client_service.clone(), waltid_config));
        let vc_req_service = Arc::new(VCReqService::new(client_service.clone(), vc_req_config));
        let gatekeeper_service = Arc::new(GnapGateKeeperService::new(gatekeeper_config));
        let verifier_service = Arc::new(VerifierService::new(
            client_service.clone(),
            verifier_config,
        ));
        let callback_service = Arc::new(BasicCallbackService::new(client_service.clone()));
        let business_service = Arc::new(BasicBusinessService::new(business_config));
        let repo_service = Arc::new(AuthProviderRepoForSql::create_repo(db_connection));

        let gaia_service: Option<Arc<dyn GaiaSelfIssuerTrait>> = match config.gaia() {
            true => {
                let gaia_config = GaiaSelfIssuerConfig::from(config.clone());
                Some(Arc::new(BasicGaiaSelfIssuer::new(gaia_config)))
            }
            false => None,
        };

        // CORE
        let provider = Arc::new(AuthProvider::new(
            wallet_service,
            vc_req_service,
            gatekeeper_service,
            verifier_service,
            callback_service,
            business_service,
            repo_service,
            client_service,
            core_config,
            gaia_service,
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
    pub async fn create_router_4_monolith(config: ApplicationProviderConfig) -> Router {
        let config = AuthProviderConfig::from(config);
        AuthProviderApplication::create_router(&config).await
    }
}
