/*
 *
 *  * Copyright (C) 2024 - Universidad Politécnica de Madrid - UPM
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

use crate::core::vc_request_service::vc_request_service::VCRequestService;
use crate::data::repo::sql::VCRequestsRepoForSql;
use crate::data::repo::VCRequestsFactory;
use crate::http::router::AuthorityRouter;
use crate::setup::config::AuthorityApplicationConfig;
use axum::{serve, Router};
use rainbow_common::config::global_config::ApplicationGlobalConfig;
use rainbow_common::config::provider_config::{ApplicationProviderConfig, ApplicationProviderConfigTrait};
use sea_orm::Database;
use std::sync::Arc;
use tokio::net::TcpListener;
use tracing::info;

pub struct AuthorityApplication;

pub async fn create_authority_router(config: &AuthorityApplicationConfig) -> Router {
    let db_connection = Database::connect(config.get_full_db_url()).await.expect("Database can't connect");
    // Repo
    let authority_repo = Arc::new(VCRequestsRepoForSql::create_repo(db_connection.clone()));

    // Services
    let vc_requests_service = Arc::new(VCRequestService::new(authority_repo.clone()));
    let vc_requests_router = AuthorityRouter::new(vc_requests_service.clone()).router();

    // Router
    let authority_application_router = Router::new()
        .merge(vc_requests_router);

    authority_application_router
}

impl AuthorityApplication {
    pub async fn run(config: &AuthorityApplicationConfig) -> anyhow::Result<()> {
        // db_connection
        let router = create_authority_router(config).await;
        // Init server
        let server_message = format!(
            "Starting provider server in {}",
            config.get_auth_host_url().unwrap()
        );
        info!("{}", server_message);
        let listener = TcpListener::bind(format!(
            "{}:{}",
            config.get_raw_ssi_auth_host().clone().unwrap().url,
            config.get_raw_ssi_auth_host().clone().unwrap().port
        ))
            .await?;
        serve(listener, router).await?;
        Ok(())
    }
}
