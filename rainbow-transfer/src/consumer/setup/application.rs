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
use crate::common::core::mates_facade::mates_facade::MatesFacadeService;
use crate::consumer::core::data_plane_facade::data_plane_facade::DataPlaneConsumerFacadeForDSProtocol;
use crate::consumer::core::ds_protocol::ds_procotol::DSProtocolTransferConsumerService;
use crate::consumer::core::ds_protocol_rpc::ds_protocol_rpc::DSRPCTransferConsumerService;
use crate::consumer::core::rainbow_entities::rainbow_entities::RainbowTransferConsumerServiceImpl;
use crate::consumer::http::ds_protocol::ds_protocol::DSProtocolTransferConsumerRouter;
use crate::consumer::http::ds_protocol_rpc::ds_protocol_rpc::DSRPCTransferConsumerRouter;
use crate::consumer::http::rainbow_entities::rainbow_entities::RainbowTransferConsumerEntitiesRouter;
use crate::consumer::setup::config::TransferConsumerApplicationConfig;
use axum::{serve, Router};
use rainbow_common::config::consumer_config::{ApplicationConsumerConfig, ApplicationConsumerConfigTrait};
use rainbow_common::facades::ssi_auth_facade::ssi_auth_facade::SSIAuthFacadeService;
use rainbow_dataplane::coordinator::controller::controller_service::DataPlaneControllerService;
use rainbow_dataplane::coordinator::dataplane_process::dataplane_process_service::DataPlaneProcessService;
use rainbow_dataplane::data_plane_info::data_plane_info::DataPlaneInfoService;
use rainbow_dataplane::http::DataPlaneRouter;
use rainbow_dataplane::testing_proxy::http::http::TestingHTTPProxy;
use rainbow_db::dataplane::repo::sql::DataPlaneRepoForSql;
use rainbow_db::dataplane::repo::DataPlaneRepoFactory;
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

pub struct TransferConsumerApplication;

pub async fn create_transfer_consumer_router(config: &TransferConsumerApplicationConfig) -> Router {
    let db_connection = Database::connect(config.get_full_db_url()).await.expect("Database can't connect");

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

    // Dataplane services
    let application_global_config: ApplicationConsumerConfig = config.clone().into();
    let dataplane_repo = Arc::new(DataPlaneRepoForSql::create_repo(db_connection.clone()));
    let dataplane_process_service = Arc::new(DataPlaneProcessService::new(dataplane_repo));
    let dataplane_controller = Arc::new(DataPlaneControllerService::new(
        Arc::new(application_global_config.clone().into()),
        dataplane_process_service.clone(),
    ));
    let dataplane_testing_router = TestingHTTPProxy::new(
        application_global_config.clone().into(),
        dataplane_process_service.clone(),
    )
        .router();

    // Dataplane Router
    let dataplane_info_service = Arc::new(DataPlaneInfoService::new(
        dataplane_process_service.clone(),
        application_global_config.clone().into(),
    ));
    let dataplane_info_router = DataPlaneRouter::new(dataplane_info_service.clone()).router();

    // Rainbow Entities Dependency injection
    let consumer_repo = Arc::new(TransferConsumerRepoForSql::create_repo(db_connection));
    let rainbow_entities_service = RainbowTransferConsumerServiceImpl::new(consumer_repo.clone());
    let rainbow_entities_router =
        RainbowTransferConsumerEntitiesRouter::new(Arc::new(rainbow_entities_service)).router();

    // DSProtocol Dependency injection
    let ssi_auth_facade = Arc::new(SSIAuthFacadeService::new(
        application_global_config.clone().into(),
    ));
    let data_plane_facade = Arc::new(DataPlaneConsumerFacadeForDSProtocol::new(
        dataplane_controller.clone(),
        config.clone(),
    ));
    let ds_protocol_service = Arc::new(DSProtocolTransferConsumerService::new(
        consumer_repo.clone(),
        data_plane_facade.clone(),
        notification_service.clone(),
        ssi_auth_facade.clone(),
    ));
    let ds_protocol_router = DSProtocolTransferConsumerRouter::new(ds_protocol_service.clone()).router();

    // DSRPCProtocol Dependency injection
    let app_config: ApplicationConsumerConfig = config.clone().into();
    let mates_facade = Arc::new(MatesFacadeService::new(
        app_config.into()
    ));
    let ds_protocol_rpc_service = Arc::new(DSRPCTransferConsumerService::new(
        consumer_repo.clone(),
        data_plane_facade.clone(),
        config.clone(),
        notification_service.clone(),
        mates_facade.clone(),
    ));
    let ds_protocol_rpc_router = DSRPCTransferConsumerRouter::new(ds_protocol_rpc_service.clone()).router();

    // Router
    let transfer_provider_application_router = Router::new()
        .merge(rainbow_entities_router)
        .merge(ds_protocol_router)
        .merge(ds_protocol_rpc_router)
        .merge(dataplane_testing_router)
        .merge(dataplane_info_router)
        .nest("/api/v1/transfers", subscription_router)
        .nest("/api/v1/transfers", notification_router);

    transfer_provider_application_router
}

impl TransferConsumerApplication {
    pub async fn run(config: &TransferConsumerApplicationConfig) -> anyhow::Result<()> {
        // db_connection
        let router = create_transfer_consumer_router(&config.clone()).await;
        // Init server
        let server_message = format!(
            "Starting consumer server in {}",
            config.get_transfer_host_url().unwrap()
        );
        info!("{}", server_message);
        let listener = TcpListener::bind(format!(
            "{}:{}",
            config.get_raw_transfer_process_host().clone().unwrap().url,
            config.get_raw_transfer_process_host().clone().unwrap().port
        ))
            .await?;
        serve(listener, router).await?;
        Ok(())
    }
}
