use crate::coordinator::data_source_connector::data_source_connector::DataSourceConnector;
use crate::coordinator::dataplane_access_controller::dataplane_access_controller::DataPlaneAccessControllerService;
use crate::coordinator::dataplane_access_controller::DataPlaneAccessControllerTrait;
use crate::data::factory_sql::DataPlaneRepoForSql;
use crate::data::factory_trait::DataPlaneRepoTrait;
use crate::entities::data_plane_process::data_plane_process_entity::DataPlaneProcessEntityService;
use crate::entities::transfer_events::transfer_event_entity::TransferEventEntityService;
use crate::http::dataplane_info::DataPlaneRouter;
use crate::http::transfer_events::TransferEventsRouter;
use crate::testing_proxy::http::http::TestingHTTPProxy;
use axum::Router;
use rainbow_common::config::services::TransferConfig;
use rainbow_common::config::traits::DatabaseConfigTrait;
use rainbow_common::vault::vault_rs::VaultService;
use rainbow_common::vault::VaultTrait;
use sea_orm::Database;
use std::ops::Deref;
use std::sync::Arc;

pub struct DataplaneSetup {}
impl DataplaneSetup {
    pub fn new() -> Self {
        DataplaneSetup {}
    }
    pub async fn get_data_plane_repo(
        &self,
        config: &TransferConfig,
        vault: Arc<VaultService>,
    ) -> Arc<dyn DataPlaneRepoTrait> {
        let db_connection = vault.get_db_connection(config.clone()).await;
        let dataplane_repo = Arc::new(DataPlaneRepoForSql::create_repo(db_connection.clone()));
        dataplane_repo
    }
    pub async fn get_data_plane_controller(
        &self,
        config: Arc<TransferConfig>,
        vault: Arc<VaultService>,
    ) -> Arc<dyn DataPlaneAccessControllerTrait> {
        let db_connection = vault.get_db_connection(config.deref().clone()).await;
        let dataplane_repo = self.get_data_plane_repo(config.as_ref(), vault.clone()).await;
        let dataplane_process_entity = Arc::new(DataPlaneProcessEntityService::new(dataplane_repo.clone()));
        let dataplane_source_connector = Arc::new(DataSourceConnector::new());
        let controller = Arc::new(DataPlaneAccessControllerService::new(
            dataplane_source_connector.clone(),
            dataplane_process_entity.clone(),
            config.clone(),
        ));
        controller
    }
    pub async fn build_control_router(&self, config: &TransferConfig, vault: Arc<VaultService>) -> Router {
        let dataplane_repo = self.get_data_plane_repo(config, vault.clone()).await;
        let dataplane_process_entity = Arc::new(DataPlaneProcessEntityService::new(dataplane_repo.clone()));
        let transfer_event_entity = Arc::new(TransferEventEntityService::new(dataplane_repo.clone()));
        let dataplane_router = DataPlaneRouter::new(
            dataplane_process_entity.clone(),
            transfer_event_entity.clone(),
        )
        .router();
        let transfer_event_router = TransferEventsRouter::new(
            dataplane_process_entity.clone(),
            transfer_event_entity.clone(),
        )
        .router();
        Router::new().merge(dataplane_router).merge(transfer_event_router)
    }
    pub async fn build_testing_proxy(&self, config: &TransferConfig, vault: Arc<VaultService>) -> Router {
        let dataplane_repo = self.get_data_plane_repo(config, vault.clone()).await;
        let dataplane_process_entity = Arc::new(DataPlaneProcessEntityService::new(dataplane_repo.clone()));
        TestingHTTPProxy::new(dataplane_process_entity.clone()).router()
    }
}
