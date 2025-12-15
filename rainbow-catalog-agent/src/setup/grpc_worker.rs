#![allow(unused)]

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

use rainbow_common::config::global_config::ApplicationGlobalConfig;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;
use tonic::codegen::tokio_stream::wrappers::TcpListenerStream;
use tonic::transport::Server;

pub struct CatalogGrpcWorker {}

impl CatalogGrpcWorker {
    pub async fn spawn(config: &ApplicationGlobalConfig, token: &CancellationToken) -> anyhow::Result<JoinHandle<()>> {
        //let router = Self::create_root_grpc_router(&config).await?;
        let host = if config.is_local { "127.0.0.1" } else { "0.0.0.0" };
        let port = config.catalog_host.clone().expect("no host").port;
        let grpc_port = format!("{}{}", port, "1");
        let addr = format!("{}:{}", host, grpc_port);

        let listener = TcpListener::bind(&addr).await?;
        let incoming = TcpListenerStream::new(listener);
        tracing::info!("GRPC Catalog Service running on {}", addr);

        let token = token.clone();
        let handle = tokio::spawn(async move {
            // let server = router.serve_with_incoming_shutdown(incoming, async move {
            //     token.cancelled().await;
            //     tracing::info!("GRPC Service received shutdown signal, draining connections...");
            // });
            // match server.await {
            //     Ok(_) => tracing::info!("GRPC Service stopped successfully"),
            //     Err(e) => tracing::error!("GRPC Service crashed: {}", e),
            // }
        });

        Ok(handle)
    }
    // pub async fn create_root_grpc_router(
    //     config: &ApplicationGlobalConfig,
    // ) -> anyhow::Result<tonic::transport::server::Router> {
    //     let db_connection =
    //         Database::connect(config.database_config.as_db_url()).await.expect("Database can't connect");
    //     // let transfer_repo = Arc::new(TransferAgentRepoForSql::create_repo(db_connection.clone()));
    //     //
    //     // let messages_service = Arc::new(TransferAgentMessagesService::new(transfer_repo.clone()));
    //     // let messages_controller = TransferAgentMessagesGrpc::new(messages_service);
    //     // let processes_service = Arc::new(TransferAgentProcessesService::new(transfer_repo.clone()));
    //     // let processes_controller = TransferAgentProcessesGrpc::new(processes_service);
    //     //
    //     // let reflection_service = tonic_reflection::server::Builder::configure()
    //     //     .register_encoded_file_descriptor_set(FILE_DESCRIPTOR_SET)
    //     //     .build_v1()?;
    //     //
    //     // let router = Server::builder()
    //     //     .add_service(reflection_service)
    //     //     .add_service(TransferAgentProcessesServer::new(processes_controller))
    //     //     .add_service(TransferAgentMessagesServer::new(messages_controller));
    //     //
    //     // Ok(router)
    //     // Ok(Server::builder().add_optional_service(None))
    // }
}
