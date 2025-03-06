use crate::provider::core::data_service_resolver::data_service::DataServiceFacadeImpl;
use crate::provider::core::ds_protocol::ds_protocol::DSProtocolTransferProviderImpl;
use crate::provider::core::rainbow_entities::rainbow_entities::RainbowTransferProviderServiceImpl;
use crate::provider::http::ds_protocol::DSProtocolTransferProviderRouter;
use crate::provider::http::rainbow_entities::RainbowTransferProviderEntitiesRouter;
use crate::provider::setup::config::TransferProviderApplicationConfig;
use axum::{serve, Router};
use rainbow_dataplane::facade::facade::DataPlaneFacadeImpl;
use rainbow_db::transfer_provider::repo::sql::TransferProviderRepoForSql;
use rainbow_db::transfer_provider::repo::TransferProviderRepoFactory;
use sea_orm::Database;
use std::sync::Arc;
use tokio::net::TcpListener;
use tracing::info;

pub struct TransferProviderApplication;

impl TransferProviderApplication {
    pub async fn run(config: &TransferProviderApplicationConfig<'static>) -> anyhow::Result<()> {
        // db_connection
        let db_url = config.get_full_db_url();
        let db_connection = Database::connect(db_url).await.expect("Database can't connect");

        // Rainbow Entities Dependency injection
        let provider_repo = Arc::new(TransferProviderRepoForSql::create_repo(db_connection));
        let rainbow_entities_service = RainbowTransferProviderServiceImpl::new(provider_repo.clone());
        let rainbow_entities_router =
            RainbowTransferProviderEntitiesRouter::new(Arc::new(rainbow_entities_service)).router();
        // DSProtocol Dependency injection
        let data_plane_service = Arc::new(DataPlaneFacadeImpl::new());
        let data_service_facade = Arc::new(DataServiceFacadeImpl::new());
        let ds_protocol_service = DSProtocolTransferProviderImpl::new(provider_repo.clone(), data_service_facade, data_plane_service);
        let ds_protocol_router = DSProtocolTransferProviderRouter::new(Arc::new(ds_protocol_service)).router();
        // Router
        let transfer_provider_application_router =
            Router::new().merge(rainbow_entities_router).merge(ds_protocol_router);

        // Init server
        let server_message = format!("Starting provider server in {}", config.get_full_host_url(), );
        info!("{}", server_message);
        let listener = TcpListener::bind(format!(
            "{}:{}",
            config.get_host_url(),
            config.get_host_port()
        ))
            .await?;
        serve(listener, transfer_provider_application_router).await?;
        Ok(())
    }
}
