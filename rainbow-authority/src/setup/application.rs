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

use crate::core::Authority;
use crate::http::router::RainbowAuthorityRouter;
use crate::services::gatekeeper::gnap::{config::GnapConfig, GnapService};
use crate::services::client::basic::BasicClientService;
use crate::services::issuer::basic::{config::BasicIssuerConfig, BasicIssuerService};
use crate::services::repo::sql::RepoForSql;
use crate::services::verifier::basic::{config::BasicVerifierConfig, BasicVerifierService};
use crate::services::wallet::waltid::{config::WaltIdConfig, WaltIdService};
use crate::setup::config::AuthorityApplicationConfig;
use crate::setup::AuthorityApplicationConfigTrait;
use axum::{serve, Router};
use sea_orm::Database;
use std::sync::Arc;
use tokio::net::TcpListener;
use tracing::info;

pub struct AuthorityApplication;

pub async fn create_authority_router(config: &AuthorityApplicationConfig) -> Router {
    // CONFIGS
    let db_connection = Database::connect(config.get_full_db_url()).await.expect("Database can't connect");
    let waltid_config = WaltIdConfig::from(config.clone());
    let gnap_config = GnapConfig::from(config.clone());
    let issuer_config = BasicIssuerConfig::from(config.clone());
    let verifier_config = BasicVerifierConfig::from(config.clone());

    // SERVICES
    let repo = Arc::new(RepoForSql::new(db_connection));
    let client = Arc::new(BasicClientService::new());
    let wallet = Arc::new(WaltIdService::new(waltid_config, client.clone()));
    let access = Arc::new(GnapService::new(gnap_config, client.clone()));
    let issuer = Arc::new(BasicIssuerService::new(issuer_config));
    let verifier = Arc::new(BasicVerifierService::new(verifier_config));

    // CORE
    let authority = Authority::new(wallet, access, issuer, verifier, repo, client, config.clone());

    // ROUTER
    let router = RainbowAuthorityRouter::new(Arc::new(authority)).router();

    Router::new().merge(router)
}

impl AuthorityApplication {
    pub async fn run(config: &AuthorityApplicationConfig) -> anyhow::Result<()> {
        // db_connection
        let router = create_authority_router(config).await;
        // Init server
        let server_message = format!("Starting Authority server in {}", config.get_host());
        info!("{}", server_message);

        let listener = match config.is_local() {
            true => TcpListener::bind(format!("127.0.0.1{}", config.get_weird_port())).await?,
            false => TcpListener::bind(format!("0.0.0.0{}", config.get_weird_port())).await?,
        };

        serve(listener, router).await?;
        Ok(())
    }
}
