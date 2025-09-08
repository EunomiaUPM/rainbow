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

use crate::core::datahub_proxy::datahub_proxy::DatahubProxyService;
use crate::core::rainbow_rpc::rainbow_rpc::RainbowRPCDatahubCatalogService;
use crate::http::datahub_proxy::datahub_proxy::DataHubProxyRouter;
use crate::http::rainbow_entities::policies::RainbowCatalogPoliciesRouter;
use crate::http::rainbow_entities::policy_relations_router::PolicyTemplatesRouter;
use crate::http::rainbow_rpc::rainbow_rpc::RainbowRPCDatahubCatalogRouter;
use axum::{serve, Router};
use rainbow_catalog::provider::core::rainbow_entities::policies::RainbowCatalogPoliciesService;
use rainbow_common::config::provider_config::{ApplicationProviderConfig, ApplicationProviderConfigTrait};
use rainbow_db::catalog::repo::sql::CatalogRepoForSql;
use rainbow_db::datahub::repo::sql::DatahubConnectorRepoForSql;
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

pub struct DatahubCatalogApplication;

pub async fn create_datahub_catalog_router(config: &ApplicationProviderConfig) -> Router {
    let db_connection = Database::connect(config.get_full_db_url()).await.expect("Database can't connect");

    // Events router
    let subscription_repo = Arc::new(EventsRepoForSql::create_repo(db_connection.clone()));
    let subscription_service = Arc::new(RainbowEventsSubscriptionService::new(
        subscription_repo.clone(),
    ));
    let subscription_router =
        RainbowEventsSubscriptionRouter::new(subscription_service, Some(SubscriptionEntities::Catalog)).router();
    let notification_service = Arc::new(RainbowEventsNotificationsService::new(subscription_repo));
    let notification_router = RainbowEventsNotificationRouter::new(
        notification_service.clone(),
        Some(SubscriptionEntities::Catalog),
    )
        .router();

    // Datahub services
    let repo = Arc::new(DatahubConnectorRepoForSql::new(db_connection.clone()));
    let datahub_service = Arc::new(DatahubProxyService::new(config.clone()));
    let datahub_router = DataHubProxyRouter::new(datahub_service.clone());
    let policy_templates_router = PolicyTemplatesRouter::new(repo.clone(), notification_service.clone());

    // Plain Catalog Policies Router
    let plain_policies_repo = Arc::new(CatalogRepoForSql::new(db_connection));
    let plain_policies_service = Arc::new(RainbowCatalogPoliciesService::new(plain_policies_repo.clone(), notification_service.clone()));
    let plain_policies_router = RainbowCatalogPoliciesRouter::new(plain_policies_service.clone());

    // RPC policies resolver
    let rpc_service = Arc::new(RainbowRPCDatahubCatalogService::new(plain_policies_repo.clone()));
    let rpc_router = RainbowRPCDatahubCatalogRouter::new(rpc_service.clone());

    // Merge routers
    let datahub_router = Router::new()
        .merge(datahub_router.router())
        .merge(policy_templates_router.router())
        .merge(plain_policies_router.router())
        .merge(rpc_router.router())
        .nest("/api/v1/datahub", subscription_router)
        .nest("/api/v1/datahub", notification_router);

    datahub_router
}

impl DatahubCatalogApplication {
    pub async fn run(config: &ApplicationProviderConfig) -> anyhow::Result<()> {
        // db_connection
        let router = create_datahub_catalog_router(config).await;
        // Init server
        let server_message = format!(
            "Starting catalog server in {}",
            config.get_catalog_host_url().unwrap()
        );
        info!("{}", server_message);
        let listener = TcpListener::bind(format!(
            "{}:{}",
            config.get_raw_catalog_host().clone().unwrap().url,
            config.get_raw_catalog_host().clone().unwrap().port
        ))
            .await?;
        serve(listener, router).await?;
        Ok(())
    }
}
