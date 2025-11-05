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
use crate::services::access_manager::AccessManagerService;
use crate::services::client::ClientService;
use crate::services::oidc::OidcService;
use crate::services::repo::RepoForSql;
use crate::services::wallet::WalletService;
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
    let wallet_config = config.parse_to_wallet();
    let access_config = config.parse_to_access();
    let oidc_config = config.parse_to_oidc();

    // SERVICES
    let repo = Arc::new(RepoForSql::new(db_connection));
    let client = Arc::new(ClientService::new());
    let wallet = Arc::new(WalletService::new(wallet_config, client.clone()));
    let access = Arc::new(AccessManagerService::new(access_config, client.clone()));
    let oidc = Arc::new(OidcService::new(oidc_config));

    // CORE
    let authority = Authority::new(repo, client, wallet, access, oidc, config.clone());

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
