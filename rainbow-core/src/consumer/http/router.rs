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

use crate::consumer::setup::config::CoreConsumerApplicationConfig;
use axum::Router;
use rainbow_auth::ssi_auth::consumer::core::manager::Manager;
use rainbow_auth::ssi_auth::consumer::http::http::RainbowAuthConsumerRouter;
use rainbow_contracts::consumer::core::ds_protocol::ds_protocol::DSProtocolContractNegotiationConsumerService;
use rainbow_contracts::consumer::core::ds_protocol_rpc::ds_protocol_rpc::DSRPCContractNegotiationConsumerService;
use rainbow_contracts::consumer::core::rainbow_entities::rainbow_entities::RainbowEntitiesContractNegotiationConsumerService;
use rainbow_contracts::consumer::http::ds_protocol::ds_protocol::DSProtocolContractNegotiationConsumerRouter;
use rainbow_contracts::consumer::http::ds_protocol_rpc::ds_protocol_rpc::DSRPCContractNegotiationConsumerRouter;
use rainbow_contracts::consumer::http::rainbow_entities::rainbow_entities::RainbowEntitiesContractNegotiationConsumerRouter;
use rainbow_db::auth_consumer::repo::sql::AuthConsumerRepoForSql;
use rainbow_db::auth_consumer::repo::AuthConsumerRepoFactory;
use rainbow_db::contracts_consumer::repo::sql::ContractNegotiationConsumerRepoForSql;
use rainbow_db::contracts_consumer::repo::ContractNegotiationConsumerRepoFactory;
use rainbow_db::events::repo::sql::EventsRepoForSql;
use rainbow_db::events::repo::EventsRepoFactory;
use rainbow_db::transfer_consumer::repo::sql::TransferConsumerRepoForSql;
use rainbow_db::transfer_consumer::repo::TransferConsumerRepoFactory;
use rainbow_events::core::notification::notification::RainbowEventsNotificationsService;
use rainbow_events::core::subscription::subscription::RainbowEventsSubscriptionService;
use rainbow_events::http::notification::notification::RainbowEventsNotificationRouter;
use rainbow_events::http::subscription::subscription::RainbowEventsSubscriptionRouter;
use rainbow_transfer::consumer::core::data_plane_facade::data_plane_facade::DataPlaneConsumerFacadeImpl;
use rainbow_transfer::consumer::core::ds_protocol::ds_procotol::DSProtocolTransferConsumerService;
use rainbow_transfer::consumer::core::ds_protocol_rpc::ds_protocol_rpc::DSRPCTransferConsumerService;
use rainbow_transfer::consumer::core::rainbow_entities::rainbow_entities::RainbowTransferConsumerServiceImpl;
use rainbow_transfer::consumer::http::ds_protocol::ds_protocol::DSProtocolTransferConsumerRouter;
use rainbow_transfer::consumer::http::ds_protocol_rpc::ds_protocol_rpc::DSRPCTransferConsumerRouter;
use rainbow_transfer::consumer::http::rainbow_entities::rainbow_entities::RainbowTransferConsumerEntitiesRouter;
use rainbow_transfer::consumer::setup::config::TransferConsumerApplicationConfig;
use sea_orm::Database;
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn create_core_consumer_router(config: &CoreConsumerApplicationConfig) -> Router {
    let db_connection = Database::connect(config.get_full_db_url()).await.expect("Database can't connect");

    // DB repos
    let subscription_repo = Arc::new(EventsRepoForSql::create_repo(db_connection.clone()));
    let transfer_provider_repo = Arc::new(TransferConsumerRepoForSql::create_repo(
        db_connection.clone(),
    ));
    let cn_provider_repo = Arc::new(ContractNegotiationConsumerRepoForSql::create_repo(
        db_connection.clone(),
    ));
    let auth_repo = Arc::new(AuthConsumerRepoForSql::create_repo(db_connection.clone()));

    // =====================
    // AUTH
    // =====================
    let auth_manager = Arc::new(Mutex::new(Manager::new(auth_repo.clone(), config.clone().into())));
    let auth_router = RainbowAuthConsumerRouter::new(auth_manager).router();

    // =====================
    // EVENTS
    // =====================

    // Events router
    let subscription_service = Arc::new(RainbowEventsSubscriptionService::new(
        subscription_repo.clone(),
    ));
    let subscription_router = RainbowEventsSubscriptionRouter::new(subscription_service, None).router();
    let notification_service = Arc::new(RainbowEventsNotificationsService::new(subscription_repo));
    let notification_router = RainbowEventsNotificationRouter::new(notification_service.clone(), None).router();

    // =====================
    // CONTRACT NEGOTIATION
    // =====================

    // Rainbow Entities Dependency injection
    let cn_rainbow_entities_service = Arc::new(RainbowEntitiesContractNegotiationConsumerService::new(
        cn_provider_repo.clone(),
    ));
    let cn_rainbow_entities_router =
        RainbowEntitiesContractNegotiationConsumerRouter::new(cn_rainbow_entities_service.clone()).router();

    // DSProtocol Dependency injection
    let cn_ds_protocol_service = Arc::new(DSProtocolContractNegotiationConsumerService::new(
        cn_provider_repo.clone(),
    ));
    let cn_ds_protocol_router =
        DSProtocolContractNegotiationConsumerRouter::new(cn_ds_protocol_service.clone()).router();

    // DSRPCProtocol Dependency injection
    let cn_ds_protocol_rpc_service = Arc::new(DSRPCContractNegotiationConsumerService::new(
        cn_provider_repo.clone(),
    ));
    let cn_ds_protocol_rpc_router =
        DSRPCContractNegotiationConsumerRouter::new(cn_ds_protocol_rpc_service.clone()).router();

    // =====================
    // TRANSFER
    // =====================

    // Rainbow Entities Dependency injection
    let rainbow_entities_service = RainbowTransferConsumerServiceImpl::new(transfer_provider_repo.clone());
    let transfer_rainbow_entities_router =
        RainbowTransferConsumerEntitiesRouter::new(Arc::new(rainbow_entities_service)).router();

    // DSProtocol Dependency injection
    let data_plane_facade = Arc::new(DataPlaneConsumerFacadeImpl::new());
    let ds_protocol_service = Arc::new(DSProtocolTransferConsumerService::new(
        transfer_provider_repo.clone(),
        data_plane_facade.clone(),
    ));
    let transfer_ds_protocol_router = DSProtocolTransferConsumerRouter::new(ds_protocol_service.clone()).router();

    // DSRPCProtocol Dependency injection
    let config = TransferConsumerApplicationConfig::default();
    config.merge_dotenv_configuration().unwrap_or(config.clone());
    let ds_protocol_rpc_service = Arc::new(DSRPCTransferConsumerService::new(
        transfer_provider_repo.clone(),
        data_plane_facade.clone(),
        config.clone().into(),
    ));
    let transfer_ds_protocol_rpc_router = DSRPCTransferConsumerRouter::new(ds_protocol_rpc_service.clone()).router();

    // =====================
    // ROUTER
    // =====================

    let the_router = Router::new()
        .merge(auth_router)
        .merge(cn_rainbow_entities_router)
        .merge(cn_ds_protocol_router)
        .merge(cn_ds_protocol_rpc_router)
        .merge(transfer_rainbow_entities_router)
        .merge(transfer_ds_protocol_router)
        .merge(transfer_ds_protocol_rpc_router)
        .nest("/api/v1/transfers", subscription_router.clone())
        .nest("/api/v1/transfers", notification_router.clone())
        .nest("/api/v1/contract-negotiation", subscription_router)
        .nest("/api/v1/contract-negotiation", notification_router);

    the_router
}
