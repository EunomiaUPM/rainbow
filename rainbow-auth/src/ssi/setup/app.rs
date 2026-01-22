/*
 * Copyright (C) 2025 - Universidad Politécnica de Madrid - UPM
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */
use std::net::SocketAddr;
use std::sync::Arc;

use axum::{serve, Router};
use axum_server::tls_rustls::RustlsConfig;
use rainbow_common::config::services::SsiAuthConfig;
use rainbow_common::config::traits::{HostConfigTrait, IsLocalTrait};
use rainbow_common::config::types::HostType;
use rainbow_common::utils::expect_from_env;
use rainbow_common::vault::secrets::PemHelper;
use rainbow_common::vault::vault_rs::VaultService;
use rainbow_common::vault::VaultTrait;
use tokio::net::TcpListener;
use tracing::{info, warn};

use crate::ssi::core::AuthCore;
use crate::ssi::http::AuthRouter;
use crate::ssi::services::business::basic::config::BusinessConfig;
use crate::ssi::services::business::basic::BasicBusinessService;
use crate::ssi::services::callback::basic::BasicCallbackService;
use crate::ssi::services::client::basic::BasicClientService;
use crate::ssi::services::gaia_self_issuer::basic::config::GaiaSelfIssuerConfig;
use crate::ssi::services::gaia_self_issuer::basic::BasicGaiaSelfIssuer;
use crate::ssi::services::gaia_self_issuer::GaiaSelfIssuerTrait;
use crate::ssi::services::gatekeeper::gnap::config::GnapGateKeeperConfig;
use crate::ssi::services::gatekeeper::gnap::GnapGateKeeperService;
use crate::ssi::services::onboarder::gnap::config::GnapOnboarderConfig;
use crate::ssi::services::onboarder::gnap::GnapOnboarderService;
use crate::ssi::services::repo::postgres::service::AuthRepoForSql;
use crate::ssi::services::vc_requester::basic::config::VCRequesterConfig;
use crate::ssi::services::vc_requester::basic::VCReqService;
use crate::ssi::services::verifier::basic_v1::config::VerifierConfig;
use crate::ssi::services::verifier::basic_v1::VerifierService;
use crate::ssi::services::wallet::waltid::config::WaltIdConfig;
use crate::ssi::services::wallet::waltid::WaltIdService;
use crate::ssi::services::wallet::WalletServiceTrait;

pub struct AuthApplication {}

impl AuthApplication {
    pub async fn create_router(config: &SsiAuthConfig, vault: Arc<VaultService>) -> Router {
        // CONFIGS
        let db_connection = vault.get_db_connection(config.clone()).await;
        let vc_req_config = VCRequesterConfig::from(config.clone());
        let onboarder_config = GnapOnboarderConfig::from(config.clone());
        let gatekeeper_config = GnapGateKeeperConfig::from(config.clone());
        let verifier_config = VerifierConfig::from(config.clone());
        let business_config = BusinessConfig::from(config.clone());
        let core_config = Arc::new(config.clone());

        // SERVICES
        let client = Arc::new(BasicClientService::new());
        let vc_req = Arc::new(VCReqService::new(client.clone(), vault.clone(), vc_req_config));
        let onboarder = Arc::new(GnapOnboarderService::new(
            client.clone(),
            vault.clone(),
            onboarder_config
        ));
        let callback = Arc::new(BasicCallbackService::new(client.clone()));
        let repo = Arc::new(AuthRepoForSql::create_repo(db_connection));
        let gatekeeper = Arc::new(GnapGateKeeperService::new(gatekeeper_config));
        let business = Arc::new(BasicBusinessService::new(business_config));
        let verifier = Arc::new(VerifierService::new(client.clone(), verifier_config));

        let gaia: Option<Arc<dyn GaiaSelfIssuerTrait>> = match config.is_gaia_active() {
            true => {
                let gaia_config = GaiaSelfIssuerConfig::from(config.clone());
                Some(Arc::new(BasicGaiaSelfIssuer::new(vault.clone(), gaia_config)))
            }
            false => None
        };

        let wallet: Option<Arc<dyn WalletServiceTrait>> = match config.is_wallet_active() {
            true => {
                let walt_id_config = WaltIdConfig::from(config.clone());
                Some(Arc::new(WaltIdService::new(
                    client.clone(),
                    vault.clone(),
                    walt_id_config
                )))
            }
            false => None
        };

        // CORE
        let core = Arc::new(AuthCore::new(
            vc_req,
            onboarder,
            callback,
            business,
            gatekeeper,
            verifier,
            repo,
            core_config,
            wallet,
            gaia
        ));

        AuthRouter::new(core).router()
    }

    pub async fn run_basic(
        config: SsiAuthConfig,
        vault_service: Arc<VaultService>
    ) -> anyhow::Result<()> {
        let server_message =
            format!("Starting Auth Consumer server in {}", config.get_host(HostType::Http));
        info!("{}", server_message);

        let router = Self::create_router(&config, vault_service).await;

        let listener = match config.is_local() {
            true => TcpListener::bind(format!("127.0.0.1{}", config.get_weird_port())).await?,
            false => TcpListener::bind(format!("0.0.0.0{}", config.get_weird_port())).await?
        };

        serve(listener, router).await?;

        Ok(())
    }

    pub async fn run_tls(config: &SsiAuthConfig, vault: Arc<VaultService>) -> anyhow::Result<()> {
        let cert = expect_from_env("VAULT_APP_ROOT_CLIENT_KEY");
        let pkey = expect_from_env("VAULT_APP_CLIENT_KEY");
        let cert: PemHelper = vault.read(None, &cert).await?;
        let pkey: PemHelper = vault.read(None, &pkey).await?;

        rustls::crypto::ring::default_provider()
            .install_default()
            .expect("Unable to install crypto utils");

        let tls_config = RustlsConfig::from_pem(
            cert.data().as_bytes().to_vec(),
            pkey.data().as_bytes().to_vec()
        )
        .await?;

        let router = Self::create_router(config, vault).await;

        let addr_str =
            if config.is_local() { "127.0.0.1:443".to_string() } else { "0.0.0.0:443".to_string() };
        let addr: SocketAddr = addr_str.parse()?;
        info!("Starting Authority server with TLS in {}", addr);

        axum_server::bind_rustls(addr, tls_config).serve(router.into_make_service()).await?;
        Ok(())
    }
    pub async fn run(config: SsiAuthConfig, vault: Arc<VaultService>) -> anyhow::Result<()> {
        match Self::run_tls(&config, vault.clone()).await {
            Ok(_) => Ok(()),
            Err(err) => {
                warn!("TLS failed: {:?}, falling back to basic server", err);
                Self::run_basic(config, vault).await
            }
        }
    }
}
