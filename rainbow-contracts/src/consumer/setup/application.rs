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

use crate::consumer::core::ds_protocol::ds_protocol::DSProtocolContractNegotiationConsumerService;
use crate::consumer::core::ds_protocol_rpc::ds_protocol_rpc::DSRPCContractNegotiationConsumerService;
// use crate::consumer::core::ds_protocol_rpc::ds_protocol_rpc::DSRPCContractNegotiationConsumerService;
use crate::consumer::core::rainbow_entities::rainbow_entities::RainbowEntitiesContractNegotiationConsumerService;
use crate::consumer::http::ds_protocol::ds_protocol::DSProtocolContractNegotiationConsumerRouter;
use crate::consumer::http::ds_protocol_rpc::ds_protocol_rpc::DSRPCContractNegotiationConsumerRouter;
// use crate::consumer::http::ds_protocol_rpc::ds_protocol_rpc::DSRPCContractNegotiationConsumerRouter;
use crate::consumer::http::rainbow_entities::rainbow_entities::RainbowEntitiesContractNegotiationConsumerRouter;
use crate::consumer::setup::config::ContractNegotiationConsumerApplicationConfig;
use axum::{serve, Router};
use rainbow_common::config::consumer_config::ApplicationConsumerConfigTrait;
use rainbow_db::contracts_consumer::repo::sql::ContractNegotiationConsumerRepoForSql;
use rainbow_db::contracts_consumer::repo::ContractNegotiationConsumerRepoFactory;
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

pub struct ContractNegotiationConsumerApplication;

pub async fn create_contract_negotiation_consumer_router(config: &ContractNegotiationConsumerApplicationConfig) -> Router {
    let db_connection = Database::connect(config.get_full_db_url()).await.expect("Database can't connect");
    let consumer_repo = Arc::new(ContractNegotiationConsumerRepoForSql::create_repo(
        db_connection.clone(),
    ));

    // Events router
    let subscription_repo = Arc::new(EventsRepoForSql::create_repo(db_connection.clone()));
    let subscription_service = Arc::new(RainbowEventsSubscriptionService::new(
        subscription_repo.clone(),
    ));
    let subscription_router = RainbowEventsSubscriptionRouter::new(
        subscription_service,
        Some(SubscriptionEntities::ContractNegotiationProcess),
    )
        .router();
    let notification_service = Arc::new(RainbowEventsNotificationsService::new(subscription_repo));
    let notification_router = RainbowEventsNotificationRouter::new(
        notification_service.clone(),
        Some(SubscriptionEntities::ContractNegotiationProcess),
    )
        .router();

    // Rainbow Entities Dependency injection
    let rainbow_entities_service = Arc::new(RainbowEntitiesContractNegotiationConsumerService::new(
        consumer_repo.clone(),
    ));
    let rainbow_entities_router =
        RainbowEntitiesContractNegotiationConsumerRouter::new(rainbow_entities_service.clone()).router();

    // DSRPCProtocol Dependency injection
    let ds_rpc_protocol_service = Arc::new(DSRPCContractNegotiationConsumerService::new(
        consumer_repo.clone(),
        notification_service.clone(),
    ));
    let ds_rpc_protocol_router = DSRPCContractNegotiationConsumerRouter::new(ds_rpc_protocol_service.clone()).router();

    // DSProtocol Dependency injection
    let ds_protocol_service = Arc::new(DSProtocolContractNegotiationConsumerService::new(
        consumer_repo.clone(),
        notification_service.clone(),
    ));
    let ds_protocol_router = DSProtocolContractNegotiationConsumerRouter::new(ds_protocol_service.clone()).router();

    // Router
    Router::new()
        .merge(ds_protocol_router)
        .merge(ds_rpc_protocol_router)
        .merge(rainbow_entities_router)
        .nest("/api/v1/contract-negotiation", subscription_router)
        .nest("/api/v1/contract-negotiation", notification_router)
}

impl ContractNegotiationConsumerApplication {
    pub async fn run(config: &ContractNegotiationConsumerApplicationConfig) -> anyhow::Result<()> {
        // db_connection
        let router = create_contract_negotiation_consumer_router(config).await;
        // Init server
        let server_message = format!(
            "Starting provider server in {}",
            config.get_contract_negotiation_host_url().unwrap()
        );
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
