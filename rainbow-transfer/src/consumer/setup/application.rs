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
use crate::consumer::core::data_plane_facade::data_plane_facade::DataPlaneConsumerFacadeImpl;
use crate::consumer::core::ds_protocol::ds_procotol::DSProtocolTransferConsumerService;
use crate::consumer::core::ds_protocol_rpc::ds_protocol_rpc::DSRPCTransferConsumerService;
use crate::consumer::core::rainbow_entities::rainbow_entities::RainbowTransferConsumerServiceImpl;
use crate::consumer::http::ds_protocol::ds_protocol::DSProtocolTransferConsumerRouter;
use crate::consumer::http::ds_protocol_rpc::ds_protocol_rpc::DSRPCTransferConsumerRouter;
use crate::consumer::http::rainbow_entities::rainbow_entities::RainbowTransferConsumerEntitiesRouter;
use crate::consumer::setup::config::TransferConsumerApplicationConfig;
use axum::{serve, Router};
use rainbow_common::config::consumer_config::ApplicationConsumerConfigTrait;
use rainbow_db::transfer_consumer::repo::sql::TransferConsumerRepoForSql;
use rainbow_db::transfer_consumer::repo::TransferConsumerRepoFactory;
use sea_orm::Database;
use std::sync::Arc;
use tokio::net::TcpListener;
use tracing::info;

pub struct TransferConsumerApplication;

pub async fn create_transfer_consumer_router(config: &TransferConsumerApplicationConfig) -> Router {
    let db_connection = Database::connect(config.get_full_db_url()).await.expect("Database can't connect");

    // Rainbow Entities Dependency injection
    let consumer_repo = Arc::new(TransferConsumerRepoForSql::create_repo(db_connection));
    let rainbow_entities_service = RainbowTransferConsumerServiceImpl::new(consumer_repo.clone());
    let rainbow_entities_router =
        RainbowTransferConsumerEntitiesRouter::new(Arc::new(rainbow_entities_service)).router();

    // DSProtocol Dependency injection
    let data_plane_facade = Arc::new(DataPlaneConsumerFacadeImpl::new());
    let ds_protocol_service = Arc::new(DSProtocolTransferConsumerService::new(
        consumer_repo.clone(),
        data_plane_facade.clone(),
    ));
    let ds_protocol_router = DSProtocolTransferConsumerRouter::new(ds_protocol_service.clone()).router();

    // DSRPCProtocol Dependency injection
    let ds_protocol_rpc_service = Arc::new(DSRPCTransferConsumerService::new(
        consumer_repo.clone(),
        data_plane_facade.clone(),
        config.clone(),
    ));
    let ds_protocol_rpc_router = DSRPCTransferConsumerRouter::new(ds_protocol_rpc_service.clone()).router();

    // Router
    let transfer_provider_application_router =
        Router::new().merge(rainbow_entities_router).merge(ds_protocol_router).merge(ds_protocol_rpc_router);

    transfer_provider_application_router
}

impl TransferConsumerApplication {
    pub async fn run(config: &TransferConsumerApplicationConfig) -> anyhow::Result<()> {
        // db_connection
        let router = create_transfer_consumer_router(&config.clone()).await;
        // Init server
        let server_message = format!("Starting consumer server in {}", config.get_transfer_host_url().unwrap());
        info!("{}", server_message);
        let listener = TcpListener::bind(config.get_transfer_host_url().unwrap())
            .await?;
        serve(listener, router).await?;
        Ok(())
    }
}
