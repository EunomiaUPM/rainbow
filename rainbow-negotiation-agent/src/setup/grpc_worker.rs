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

use rainbow_common::config::provider_config::{ApplicationProviderConfig, ApplicationProviderConfigTrait};
use sea_orm::Database;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;
use tonic::codegen::tokio_stream::wrappers::TcpListenerStream;
use tonic::transport::Server;

pub struct NegotiationGrpcWorker {}

impl NegotiationGrpcWorker {
    pub async fn spawn(
        config: &ApplicationProviderConfig,
        token: &CancellationToken,
    ) -> anyhow::Result<JoinHandle<()>> {
        // let router = Self::create_root_grpc_router(&config).await?;
        let host = if config.get_environment_scenario() { "127.0.0.1" } else { "0.0.0.0" };
        let port = config.get_raw_contract_negotiation_host().clone().expect("no host").port;
        let grpc_port = format!("{}{}", port, "1");
        let addr = format!("{}:{}", host, grpc_port);

        let listener = TcpListener::bind(&addr).await?;
        let incoming = TcpListenerStream::new(listener);
        tracing::info!("GRPC Negotiation Service running on {}", addr);

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
    //     config: &ApplicationProviderConfig,
    // ) -> anyhow::Result<tonic::transport::server::Router> {
    //     let db_connection = Database::connect(config.get_full_db_url()).await.expect("Database can't connect");
    //     let config = Arc::new(config.clone());
    //
    //     let router = Server::default().add_service();
    //
    //     Ok(router)
    // }
}
