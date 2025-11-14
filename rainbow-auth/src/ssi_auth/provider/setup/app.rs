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
use crate::ssi_auth::provider::core::Manager;
use crate::ssi_auth::provider::http::RainbowAuthProviderRouter;
use axum::serve;
use axum::Router;
use rainbow_common::config::provider_config::{ApplicationProviderConfig, ApplicationProviderConfigTrait};
use rainbow_db::auth_provider::repo_factory::factory::AuthProviderRepoForSql;
use sea_orm::Database;
use std::sync::Arc;
use tokio::net::TcpListener;
use tracing::info;

pub struct SSIAuthProviderApplication;

impl SSIAuthProviderApplication {
    pub async fn run(config: &ApplicationProviderConfig) -> anyhow::Result<()> {
        if std::env::var("TEST_MODE").is_ok() {
            tracing::info!("TEST_MODE activo: saltando conexión a DB");
            return Ok(());
        }

        let router = Router::new().merge(create_ssi_provider_router(config.clone()).await);
        // Init server
        let server_message = format!(
            "Starting Auth Provider server in {}",
            config.get_transfer_host_url().unwrap()
        );
        info!("{}", server_message);
        let listener = match config.get_environment_scenario() {
            true => {
                TcpListener::bind(format!(
                    "127.0.0.1:{}",
                    config.get_raw_ssi_auth_host().clone().unwrap().port
                ))
                    .await?
            }
            false => {
                TcpListener::bind(format!(
                    "0.0.0.0:{}",
                    config.get_raw_ssi_auth_host().clone().unwrap().port
                ))
                    .await?
            }
        };
        serve(listener, router).await?;
        Ok(())
    }
}

pub async fn create_ssi_provider_router(config: ApplicationProviderConfig) -> Router {
    let db_connection = Database::connect(config.get_full_db_url()).await.expect("Database can't connect");
    let auth_repo = Arc::new(AuthProviderRepoForSql::create_repo(db_connection));
    let manager = Arc::new(Manager::new(auth_repo.clone(), config.clone()));
    let auth_router = RainbowAuthProviderRouter::new(manager.clone()).router();
    auth_router
}
