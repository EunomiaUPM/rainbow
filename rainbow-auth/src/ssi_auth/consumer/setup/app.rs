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
use crate::ssi_auth::consumer::core::Manager;
use crate::ssi_auth::consumer::http::RainbowAuthConsumerRouter;
use axum::serve;
use axum::Router;
use rainbow_common::config::consumer_config::{ApplicationConsumerConfig, ApplicationConsumerConfigTrait};
use rainbow_db::auth_consumer::repo_factory::factory::AuthConsumerRepoForSql;
use sea_orm::Database;
use std::sync::Arc;
use tokio::net::TcpListener;
use tracing::info;

pub struct SSIAuthConsumerApplication;

impl SSIAuthConsumerApplication {
    pub async fn run(config: &ApplicationConsumerConfig) -> anyhow::Result<()> {
        let router = Router::new().merge(create_ssi_consumer_router(config.clone()).await);
        // Init server
        let server_message = format!(
            "Starting Auth Consumer server in {}",
            config.get_transfer_host_url().unwrap()
        );
        info!("{}", server_message);
        let listener = TcpListener::bind(format!(
            "{}:{}",
            config.get_raw_auth_host().clone().unwrap().url,
            config.get_raw_auth_host().clone().unwrap().port
        ))
        .await?;
        serve(listener, router).await?;
        Ok(())
    }
}

pub async fn create_ssi_consumer_router(config: ApplicationConsumerConfig) -> Router {
    let db_connection = Database::connect(config.get_full_db_url()).await.expect("Database can't connect");
    let auth_repo = Arc::new(AuthConsumerRepoForSql::create_repo(db_connection));
    let manager = Arc::new(Manager::new(auth_repo.clone(), config.clone()));
    let auth_router = RainbowAuthConsumerRouter::new(manager.clone()).router();
    auth_router
}
