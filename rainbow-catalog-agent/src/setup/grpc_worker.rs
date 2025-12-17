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

use crate::data::factory_sql::CatalogAgentRepoForSql;
use crate::entities::catalogs::catalogs::CatalogEntities;
use crate::entities::data_services::data_services::DataServiceEntities;
use crate::entities::datasets::datasets::DatasetEntities;
use crate::entities::distributions::distributions::DistributionEntities;
use crate::entities::odrl_policies::odrl_policies::OdrlPolicyEntities;
use crate::entities::policy_templates::policy_templates::PolicyTemplateEntities;
use crate::grpc::api::catalog_agent::catalog_entity_service_server::CatalogEntityServiceServer;
use crate::grpc::api::catalog_agent::data_service_entity_service_server::DataServiceEntityServiceServer;
use crate::grpc::api::catalog_agent::dataset_entity_service_server::DatasetEntityServiceServer;
use crate::grpc::api::catalog_agent::distribution_entity_service_server::DistributionEntityServiceServer;
use crate::grpc::api::catalog_agent::odrl_policy_entity_service_server::OdrlPolicyEntityServiceServer;
use crate::grpc::api::catalog_agent::policy_template_entity_service_server::PolicyTemplateEntityServiceServer;
use crate::grpc::api::FILE_DESCRIPTOR_SET;
use crate::grpc::catalogs::CatalogEntityGrpc;
use crate::grpc::data_services::DataServiceEntityGrpc;
use crate::grpc::datasets::DatasetEntityGrpc;
use crate::grpc::distributions::DistributionEntityGrpc;
use crate::grpc::odrl_policies::OdrlPolicyEntityGrpc;
use crate::grpc::policy_templates::PolicyTemplateEntityGrpc;
use crate::http::catalogs::CatalogEntityRouter;
use crate::http::data_services::DataServiceEntityRouter;
use crate::http::datasets::DatasetEntityRouter;
use crate::http::distributions::DistributionEntityRouter;
use crate::http::odrl_policies::OdrlOfferEntityRouter;
use crate::http::policy_templates::PolicyTemplateEntityRouter;
use rainbow_common::config::global_config::ApplicationGlobalConfig;
use sea_orm::Database;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;
use tonic::codegen::tokio_stream::wrappers::TcpListenerStream;
use tonic::transport::Server;

pub struct CatalogGrpcWorker {}

impl CatalogGrpcWorker {
    pub async fn spawn(config: &ApplicationGlobalConfig, token: &CancellationToken) -> anyhow::Result<JoinHandle<()>> {
        let router = Self::create_root_grpc_router(&config).await?;
        let host = if config.is_local { "127.0.0.1" } else { "0.0.0.0" };
        let port = config.catalog_host.clone().expect("no host").port;
        let grpc_port = format!("{}{}", port, "1");
        let addr = format!("{}:{}", host, grpc_port);

        let listener = TcpListener::bind(&addr).await?;
        let incoming = TcpListenerStream::new(listener);
        tracing::info!("GRPC Catalog Service running on {}", addr);

        let token = token.clone();
        let handle = tokio::spawn(async move {
            let server = router.serve_with_incoming_shutdown(incoming, async move {
                token.cancelled().await;
                tracing::info!("GRPC Service received shutdown signal, draining connections...");
            });
            match server.await {
                Ok(_) => tracing::info!("GRPC Service stopped successfully"),
                Err(e) => tracing::error!("GRPC Service crashed: {}", e),
            }
        });

        Ok(handle)
    }
    pub async fn create_root_grpc_router(
        config: &ApplicationGlobalConfig,
    ) -> anyhow::Result<tonic::transport::server::Router> {
        let db_connection =
            Database::connect(config.database_config.as_db_url()).await.expect("Database can't connect");
        let catalog_agent_repo = Arc::new(CatalogAgentRepoForSql::create_repo(db_connection.clone()));

        // entities
        let catalog_controller_service = Arc::new(CatalogEntities::new(catalog_agent_repo.clone()));
        let catalog_router = CatalogEntityGrpc::new(catalog_controller_service.clone());
        let data_services_controller_service = Arc::new(DataServiceEntities::new(catalog_agent_repo.clone()));
        let data_services_router = DataServiceEntityGrpc::new(data_services_controller_service.clone());
        let datasets_controller_service = Arc::new(DatasetEntities::new(catalog_agent_repo.clone()));
        let datasets_router = DatasetEntityGrpc::new(datasets_controller_service.clone());
        let distributions_controller_service = Arc::new(DistributionEntities::new(catalog_agent_repo.clone()));
        let distributions_router = DistributionEntityGrpc::new(distributions_controller_service.clone());
        let odrl_offer_controller_service = Arc::new(OdrlPolicyEntities::new(catalog_agent_repo.clone()));
        let odrl_offer_router = OdrlPolicyEntityGrpc::new(odrl_offer_controller_service.clone());
        let policy_templates_controller_service = Arc::new(PolicyTemplateEntities::new(catalog_agent_repo.clone()));
        let policy_templates_router = PolicyTemplateEntityGrpc::new(policy_templates_controller_service.clone());

        let reflection_service = tonic_reflection::server::Builder::configure()
            .register_encoded_file_descriptor_set(FILE_DESCRIPTOR_SET)
            .build_v1()?;

        let router = Server::builder()
            .add_service(reflection_service)
            .add_service(CatalogEntityServiceServer::new(catalog_router))
            .add_service(DataServiceEntityServiceServer::new(data_services_router))
            .add_service(DatasetEntityServiceServer::new(datasets_router))
            .add_service(DistributionEntityServiceServer::new(distributions_router))
            .add_service(OdrlPolicyEntityServiceServer::new(odrl_offer_router))
            .add_service(PolicyTemplateEntityServiceServer::new(
                policy_templates_router,
            ));

        Ok(router)
    }
}
