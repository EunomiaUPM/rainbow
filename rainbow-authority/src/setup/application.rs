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
use crate::data::repo_factory::factory::AuthRepoForSql;
use crate::http::RainbowAuthorityRouter;
use crate::setup::config::AuthorityApplicationConfig;
use crate::setup::config::AuthorityFunctions;
use crate::setup::AuthorityApplicationConfigTrait;
use axum::{serve, Router};
use sea_orm::Database;
use std::sync::Arc;
use tokio::net::TcpListener;
use tracing::info;

pub struct AuthorityApplication;

pub async fn create_authority_router(config: &AuthorityApplicationConfig) -> Router {
    let db_connection = Database::connect(config.get_full_db_url()).await.expect("Database can't connect");
    let authority_repo = Arc::new(AuthRepoForSql::create_repo(db_connection.clone()));
    let authority = Arc::new(Authority::new(authority_repo.clone(), config.clone()));
    let authority_router = RainbowAuthorityRouter::new(authority.clone()).router();
    Router::new().merge(authority_router)
}

impl AuthorityApplication {
    pub async fn run(config: &AuthorityApplicationConfig) -> anyhow::Result<()> {
        // db_connection
        let router = create_authority_router(config).await;
        // Init server
        let server_message = format!("Starting Authority server in {}", config.get_host());
        info!("{}", server_message);

        let listener = match config.get_environment_scenario() {
            true => TcpListener::bind(format!("127.0.0.1:{}", config.get_port())).await?,
            false => TcpListener::bind(format!("0.0.0.0:{}", config.get_port())).await?,
        };

        serve(listener, router).await?;
        Ok(())
    }
}
