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

use crate::core::ds_protocol::ds_protocol::DSProtocolCatalogService;
use crate::core::rainbow_entities::catalog::RainbowCatalogCatalogService;
use crate::core::rainbow_entities::data_service::RainbowCatalogDataServiceService;
use crate::core::rainbow_entities::dataset::RainbowCatalogDatasetService;
use crate::core::rainbow_entities::distribution::RainbowCatalogDistributionService;
use crate::core::rainbow_entities::policies::RainbowCatalogPoliciesService;
use crate::core::rainbow_rpc::rainbow_rpc::RainbowRPCCatalogService;
use crate::http::ds_protocol::ds_protocol::DSProcotolCatalogRouter;
use crate::http::openapi::route_openapi;
use crate::http::rainbow_entities::catalog::RainbowCatalogCatalogRouter;
use crate::http::rainbow_entities::data_service::RainbowCatalogDataServiceRouter;
use crate::http::rainbow_entities::dataset::RainbowCatalogDatasetRouter;
use crate::http::rainbow_entities::distribution::RainbowCatalogDistributionRouter;
use crate::http::rainbow_entities::policies::RainbowCatalogPoliciesRouter;
use crate::http::rainbow_rpc::rainbow_rpc::RainbowRPCCatalogRouter;
use crate::setup::config::CatalogApplicationProviderConfig;
use axum::routing::get;
use axum::{serve, Router};
use rainbow_common::config::provider_config::ApplicationProviderConfigTrait;
use rainbow_db::catalog::repo::sql::CatalogRepoForSql;
use rainbow_db::catalog::repo::CatalogRepoFactory;
use rainbow_db::events::repo::sql::EventsRepoForSql;
use rainbow_db::events::repo::EventsRepoFactory;
use rainbow_db::transfer_consumer::repo::sql::TransferConsumerRepoForSql;
use rainbow_db::transfer_consumer::repo::TransferConsumerRepoFactory;
use rainbow_events::core::notification::notification::RainbowEventsNotificationsService;
use rainbow_events::core::subscription::subscription::RainbowEventsSubscriptionService;
use rainbow_events::core::subscription::subscription_types::SubscriptionEntities;
use rainbow_events::http::notification::notification::RainbowEventsNotificationRouter;
use rainbow_events::http::subscription::subscription::RainbowEventsSubscriptionRouter;
use sea_orm::Database;
use std::sync::Arc;
use tokio::net::TcpListener;
use tracing::info;

pub struct CatalogApplication;

pub async fn create_catalog_router(config: &CatalogApplicationProviderConfig) -> Router {
    let db_connection = Database::connect(config.get_full_db_url()).await.expect("Database can't connect");

    // Repos
    let catalog_repo = Arc::new(CatalogRepoForSql::create_repo(db_connection.clone()));
    let subscription_repo = Arc::new(EventsRepoForSql::create_repo(db_connection.clone()));

    // Events
    let subscription_service = Arc::new(RainbowEventsSubscriptionService::new(
        subscription_repo.clone(),
    ));
    let subscription_router = RainbowEventsSubscriptionRouter::new(
        subscription_service,
        Option::from(SubscriptionEntities::Catalog),
    )
        .router();
    let notification_service = Arc::new(RainbowEventsNotificationsService::new(subscription_repo));
    let notification_router = RainbowEventsNotificationRouter::new(
        notification_service.clone(),
        Option::from(SubscriptionEntities::Catalog),
    )
        .router();

    // DSProtocol Dependency Injection
    let ds_protocol_service = Arc::new(DSProtocolCatalogService::new(catalog_repo.clone()));
    let ds_protocol_router = DSProcotolCatalogRouter::new(ds_protocol_service.clone());

    // Rainbow Entities Dependency injection
    let rainbow_catalog_service = Arc::new(RainbowCatalogCatalogService::new(
        catalog_repo.clone(),
        notification_service.clone(),
    ));
    let rainbow_data_service_service = Arc::new(RainbowCatalogDataServiceService::new(catalog_repo.clone(), notification_service.clone()));
    let rainbow_dataset_service = Arc::new(RainbowCatalogDatasetService::new(catalog_repo.clone(), notification_service.clone()));
    let rainbow_distribution_service = Arc::new(RainbowCatalogDistributionService::new(catalog_repo.clone(), notification_service.clone()));
    let rainbow_policies_service = Arc::new(RainbowCatalogPoliciesService::new(catalog_repo.clone(), notification_service.clone()));

    let rainbow_catalog_router = RainbowCatalogCatalogRouter::new(rainbow_catalog_service, ds_protocol_service.clone());
    let rainbow_data_service_router = RainbowCatalogDataServiceRouter::new(rainbow_data_service_service.clone());
    let rainbow_dataset_router = RainbowCatalogDatasetRouter::new(rainbow_dataset_service.clone());
    let rainbow_distributions_router = RainbowCatalogDistributionRouter::new(rainbow_distribution_service.clone());
    let rainbow_policies_router = RainbowCatalogPoliciesRouter::new(rainbow_policies_service.clone());

    // RPC Dependency injection
    let rainbow_rpc_service = Arc::new(RainbowRPCCatalogService::new(catalog_repo.clone()));
    let rainbow_rpc_router = RainbowRPCCatalogRouter::new(rainbow_rpc_service.clone());

    // Router
    let catalog_application_router = Router::new()
        .merge(route_openapi())
        .merge(rainbow_catalog_router.router())
        .merge(rainbow_data_service_router.router())
        .merge(rainbow_dataset_router.router())
        .merge(rainbow_distributions_router.router())
        .merge(rainbow_policies_router.router())
        .merge(rainbow_rpc_router.router())
        .merge(ds_protocol_router.router())
        .nest("/api/v1/catalog", subscription_router.clone())
        .nest("/api/v1/catalog", notification_router.clone());
    catalog_application_router
}

impl CatalogApplication {
    pub async fn run(config: &CatalogApplicationProviderConfig) -> anyhow::Result<()> {
        // db_connection
        let router = create_catalog_router(config).await;
        // Init server
        let server_message = format!("Starting catalog server in {}", config.get_catalog_host_url().unwrap());
        info!("{}", server_message);
        let listener = TcpListener::bind(format!(
            "{}:{}",
            config.get_raw_catalog_host().clone().unwrap().url,
            config.get_raw_catalog_host().clone().unwrap().port
        )).await?;
        serve(listener, router).await?;
        Ok(())
    }
}
