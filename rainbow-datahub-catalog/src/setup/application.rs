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

use crate::core::datahub_proxy::datahub_proxy::DatahubProxyService;
use crate::http::datahub_proxy::datahub_proxy::DataHubProxyRouter;
use crate::http::rainbow_entities::policy_relations_router::{PolicyRelationsRouter, PolicyTemplatesRouter};
use crate::setup::config::DatahubCatalogApplicationProviderConfig;
use axum::routing::get;
use axum::{serve, Router};
use rainbow_common::config::provider_config::{ApplicationProviderConfig, ApplicationProviderConfigTrait};
use rainbow_db::catalog::repo::sql::CatalogRepoForSql;
use rainbow_db::catalog::repo::CatalogRepoFactory;
use rainbow_db::datahub::repo::sql::DatahubConnectorRepoForSql;
use rainbow_db::datahub::repo::DatahubConnectorRepoFactory;
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

pub struct DatahubCatalogApplication;

pub async fn create_datahub_catalog_router(config: &DatahubCatalogApplicationProviderConfig) -> Router {
    // let db_connection = Database::connect(config.get_full_db_url()).await.expect("Database can't connect");
    //
    // // config
    // let application_config: ApplicationProviderConfig = config.to_owned().into();
    //
    // // Repos
    // let datahub_catalog_repo = Arc::new(DatahubConnectorRepoForSql::create_repo(
    //     db_connection.clone(),
    // ));
    // let subscription_repo = Arc::new(EventsRepoForSql::create_repo(db_connection.clone()));
    //
    // // Events
    // let subscription_service = Arc::new(RainbowEventsSubscriptionService::new(
    //     subscription_repo.clone(),
    // ));
    // let subscription_router = RainbowEventsSubscriptionRouter::new(
    //     subscription_service,
    //     Option::from(SubscriptionEntities::Catalog),
    // )
    //     .router();
    // let notification_service = Arc::new(RainbowEventsNotificationsService::new(subscription_repo));
    // let notification_router = RainbowEventsNotificationRouter::new(
    //     notification_service.clone(),
    //     Option::from(SubscriptionEntities::Catalog),
    // )
    //     .router();
    //
    // // Datahub Connector Dependency Injection
    // let datahub_proxy_service = Arc::new(DatahubProxyService::new(application_config.clone()));
    //
    //
    // // Routers
    // let datahub_catalog_router = DataHubProxyRouter::new(datahub_proxy_service.clone());
    // // let policy_relations_router = RainbowDatahubPolicyRelationsRouter::new(
    // //     datahub_proxy_service.clone(),
    // //     rainbow_policy_relations_service.clone(),
    // // );
    //
    // // RPC Dependency injection
    // // TODO
    //
    // // Router
    // let catalog_application_router = Router::new()
    //     .merge(datahub_catalog_router.router())
    //     // .merge(policy_relations_router.router())
    //     .nest("/api/v1/datahub", subscription_router.clone())
    //     .nest("/api/v1/datahub", notification_router.clone());
    // catalog_application_router

    let config = ApplicationProviderConfig::default();
    let datahub_service = Arc::new(DatahubProxyService::new(config.clone()));
    let db_connection = Database::connect(config.get_full_db_url()).await.expect("Database can't connect");

    let repo = Arc::new(DatahubConnectorRepoForSql::new(db_connection.clone()));
    // let policy_templates_service = Arc::new(DatahubConnectorRepoForSql::new(db_connection.clone()));
    // let policy_relations_service = Arc::new(DatahubConnectorRepoForSql::new(db_connection.clone()));


    let datahub_router = DataHubProxyRouter::new(datahub_service.clone());
    let policy_templates_router = PolicyTemplatesRouter::new(repo.clone());
    let policy_relations_router = PolicyRelationsRouter::new(repo.clone());

    Router::new()
        .merge(datahub_router.router())
        .merge(policy_templates_router.router())
        .merge(policy_relations_router.router())
}

impl DatahubCatalogApplication {
    pub async fn run(config: &DatahubCatalogApplicationProviderConfig) -> anyhow::Result<()> {
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
