use crate::db::factory_sql::TransferAgentRepoForSql;
use crate::entities::transfer_messages::transfer_messages::TransferAgentMessagesService;
use crate::entities::transfer_process::transfer_process::TransferAgentProcessesService;
use crate::grpc::api::transfer_messages::transfer_agent_messages_server::TransferAgentMessagesServer;
use crate::grpc::api::transfer_processes::transfer_agent_processes_server::TransferAgentProcessesServer;
use crate::grpc::api::FILE_DESCRIPTOR_SET;
use crate::grpc::transfer_messages::TransferAgentMessagesGrpc;
use crate::grpc::transfer_process::TransferAgentProcessesGrpc;
use rainbow_common::config::provider_config::{ApplicationProviderConfig, ApplicationProviderConfigTrait};
use sea_orm::Database;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;
use tonic::codegen::tokio_stream::wrappers::TcpListenerStream;
use tonic::transport::Server;

pub struct TransferGrpcWorker {}

impl TransferGrpcWorker {
    pub async fn spawn(
        config: &ApplicationProviderConfig,
        token: &CancellationToken,
    ) -> anyhow::Result<JoinHandle<()>> {
        let router = Self::create_root_grpc_router(&config).await?;
        let host = if config.get_environment_scenario() { "127.0.0.1" } else { "0.0.0.0" };
        let port = config.get_raw_transfer_process_host().clone().expect("no host").port;
        let grpc_port = format!("{}{}", port, "1");
        let addr = format!("{}:{}", host, grpc_port);

        let listener = TcpListener::bind(&addr).await?;
        let incoming = TcpListenerStream::new(listener);
        tracing::info!("GRPC Transfer Service running on {}", addr);

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
        config: &ApplicationProviderConfig,
    ) -> anyhow::Result<tonic::transport::server::Router> {
        let db_connection = Database::connect(config.get_full_db_url()).await.expect("Database can't connect");
        let config = Arc::new(config.clone());
        let transfer_repo = Arc::new(TransferAgentRepoForSql::create_repo(db_connection.clone()));

        let messages_service = Arc::new(TransferAgentMessagesService::new(transfer_repo.clone()));
        let messages_controller = TransferAgentMessagesGrpc::new(messages_service, config.clone());
        let processes_service = Arc::new(TransferAgentProcessesService::new(transfer_repo.clone()));
        let processes_controller = TransferAgentProcessesGrpc::new(processes_service);

        let reflection_service = tonic_reflection::server::Builder::configure()
            .register_encoded_file_descriptor_set(FILE_DESCRIPTOR_SET)
            .build_v1()?;

        let router = Server::builder()
            .add_service(reflection_service)
            .add_service(TransferAgentProcessesServer::new(processes_controller))
            .add_service(TransferAgentMessagesServer::new(messages_controller));

        Ok(router)
    }
}
