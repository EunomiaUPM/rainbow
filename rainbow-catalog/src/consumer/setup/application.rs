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

use crate::consumer::core::bypass_service::bypass_service::CatalogBypassService;
use crate::consumer::http::bypass_catalog::CatalogBypassRouter;
use axum::{serve, Router};
use rainbow_common::config::services::CatalogConfig;
use rainbow_common::config::traits::{HostConfigTrait, IsLocalTrait};
use rainbow_common::config::types::HostType;
use rainbow_common::facades::ssi_auth_facade::mates_facade::MatesFacadeService;
use rainbow_common::facades::ssi_auth_facade::ssi_auth_facade::SSIAuthFacadeService;
use rainbow_db::events::repo::sql::EventsRepoForSql;
use rainbow_db::events::repo::EventsRepoFactory;
use rainbow_events::core::notification::notification::RainbowEventsNotificationsService;
use rainbow_events::core::subscription::subscription::RainbowEventsSubscriptionService;
use rainbow_events::core::subscription::subscription_types::SubscriptionEntities;
use rainbow_events::http::notification::notification::RainbowEventsNotificationRouter;
use rainbow_events::http::subscription::subscription::RainbowEventsSubscriptionRouter;
use sea_orm::Database;
use std::sync::Arc;
use tokio::net::TcpListener;
use tracing::info;

pub struct CatalogBypassConsumerApplication;

pub async fn create_catalog_bypass_consumer_router(config: CatalogConfig) -> Router {
    let mates_facade = Arc::new(MatesFacadeService::new(config.ssi_auth()));
    let bypass_service = Arc::new(CatalogBypassService::new(mates_facade.clone()));
    let bypass_router = CatalogBypassRouter::new(bypass_service.clone()).router();
    Router::new().merge(bypass_router)
}

impl CatalogBypassConsumerApplication {
    pub async fn run(config: &CatalogConfig) -> anyhow::Result<()> {
        // db_connection
        let router = create_catalog_bypass_consumer_router(config.clone()).await;
        // Init server
        let server_message = format!(
            "Starting consumer bypass server in {}",
            config.get_host(HostType::Http)
        );
        info!("{}", server_message);

        let listener = match config.is_local() {
            true => TcpListener::bind(format!("127.0.0.1{}", config.get_weird_port())).await?,
            false => TcpListener::bind(format!("0.0.0.0{}", config.get_weird_port())).await?,
        };

        serve(listener, router).await?;
        Ok(())
    }
}
