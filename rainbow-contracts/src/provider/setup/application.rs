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

use crate::provider::core::catalog_odrl_facade::catalog_odrl_facade::CatalogOdrlFacadeService;
use crate::provider::core::ds_protocol::ds_protocol::DSProtocolContractNegotiationProviderService;
use crate::provider::core::ds_protocol_rpc::ds_protocol_rpc::DSRPCContractNegotiationProviderService;
use crate::provider::core::rainbow_entities::rainbow_entities::RainbowEntitiesContractNegotiationProviderService;
use crate::provider::http::ds_protocol::ds_protocol::DSProtocolContractNegotiationProviderRouter;
use crate::provider::http::ds_protocol_rpc::ds_protocol_rpc::DSRPCContractNegotiationProviderRouter;
use crate::provider::http::rainbow_entities::rainbow_entities::RainbowEntitesContractNegotiationProviderRouter;
use crate::provider::setup::config::ContractNegotiationApplicationProviderConfig;
use axum::{serve, Router};
use rainbow_common::config::provider_config::ApplicationProviderConfigTrait;
use rainbow_db::contracts_provider::repo::sql::ContractNegotiationProviderRepoForSql;
use rainbow_db::contracts_provider::repo::ContractNegotiationProviderRepoFactory;
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

pub struct ContractNegotiationProviderApplication;

pub async fn create_contract_negotiation_provider_router(config: &ContractNegotiationApplicationProviderConfig) -> Router {
    let db_connection = Database::connect(config.get_full_db_url()).await.expect("Database can't connect");
    let provider_repo = Arc::new(ContractNegotiationProviderRepoForSql::create_repo(
        db_connection.clone(),
    ));

    // Events router
    let subscription_repo = Arc::new(EventsRepoForSql::create_repo(db_connection.clone()));
    let subscription_service = Arc::new(RainbowEventsSubscriptionService::new(
        subscription_repo.clone(),
    ));
    let subscription_router = RainbowEventsSubscriptionRouter::new(
        subscription_service,
        Some(SubscriptionEntities::TransferProcess),
    )
        .router();
    let notification_service = Arc::new(RainbowEventsNotificationsService::new(subscription_repo));
    let notification_router = RainbowEventsNotificationRouter::new(
        notification_service.clone(),
        Some(SubscriptionEntities::TransferProcess),
    )
        .router();

    // Rainbow Entities Dependency injection
    let rainbow_entities_service = Arc::new(RainbowEntitiesContractNegotiationProviderService::new(
        provider_repo.clone(),
        notification_service.clone(),
    ));
    let rainbow_entities_router =
        RainbowEntitesContractNegotiationProviderRouter::new(rainbow_entities_service.clone()).router();

    // DSProtocol Dependency injection
    let catalog_odrl_facade = Arc::new(CatalogOdrlFacadeService::new());
    let ds_protocol_service = Arc::new(DSProtocolContractNegotiationProviderService::new(
        provider_repo.clone(),
        notification_service.clone(),
        catalog_odrl_facade.clone(),
    ));
    let ds_protocol_router = DSProtocolContractNegotiationProviderRouter::new(ds_protocol_service.clone()).router();

    // DSRPCProtocol Dependency injection
    let ds_protocol_rpc_service = Arc::new(DSRPCContractNegotiationProviderService::new(
        provider_repo.clone(),
        notification_service.clone(),
        catalog_odrl_facade.clone(),
    ));
    let ds_protocol_rpc = DSRPCContractNegotiationProviderRouter::new(ds_protocol_rpc_service.clone()).router();

    // Router
    Router::new()
        .merge(rainbow_entities_router)
        .merge(ds_protocol_router)
        .merge(ds_protocol_rpc)
        .nest("/api/v1/contract-negotiation", subscription_router)
        .nest("/api/v1/contract-negotiation", notification_router)
}

impl ContractNegotiationProviderApplication {
    pub async fn run(config: &ContractNegotiationApplicationProviderConfig) -> anyhow::Result<()> {
        // db_connection
        let router = create_contract_negotiation_provider_router(config).await;
        // Init server
        let server_message = format!("Starting provider server in {}", config.get_contract_negotiation_host_url().unwrap());
        info!("{}", server_message);
        let listener = TcpListener::bind(format!(
            "{}:{}",
            config.get_raw_contract_negotiation_host().clone().unwrap().url,
            config.get_raw_contract_negotiation_host().clone().unwrap().port
        )).await?;
        serve(listener, router).await?;
        Ok(())
    }
}
