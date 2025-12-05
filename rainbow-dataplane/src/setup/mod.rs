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
use rainbow_common::config::global_config::ApplicationGlobalConfig;
use rainbow_common::config::provider_config::{ApplicationProviderConfig, ApplicationProviderConfigTrait};
use sea_orm::Database;
use std::sync::Arc;

pub struct DataplaneSetup {}
impl DataplaneSetup {
    pub fn new() -> Self {
        DataplaneSetup {}
    }
    pub async fn get_data_plane_repo(&self, config: &ApplicationGlobalConfig) -> Arc<dyn DataPlaneRepoTrait> {
        let application_global_config: ApplicationProviderConfig = config.clone().into();
        let db_url = application_global_config.get_full_db_url();
        let db_connection = Database::connect(db_url).await.expect("Database can't connect");
        let dataplane_repo = Arc::new(DataPlaneRepoForSql::create_repo(db_connection.clone()));
        dataplane_repo
    }
    pub async fn get_data_plane_controller(
        &self,
        config: Arc<ApplicationGlobalConfig>,
    ) -> Arc<dyn DataPlaneAccessControllerTrait> {
        let dataplane_repo = self.get_data_plane_repo(config.as_ref()).await;
        let dataplane_process_entity = Arc::new(DataPlaneProcessEntityService::new(dataplane_repo.clone()));
        let dataplane_source_connector = Arc::new(DataSourceConnector::new());
        let controller = Arc::new(DataPlaneAccessControllerService::new(
            dataplane_source_connector.clone(),
            dataplane_process_entity.clone(),
            config.clone(),
        ));
        controller
    }
    pub async fn build_control_router(&self, config: &ApplicationGlobalConfig) -> Router {
        let dataplane_repo = self.get_data_plane_repo(config).await;
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
    pub async fn build_testing_proxy(&self, config: &ApplicationGlobalConfig) -> Router {
        let dataplane_repo = self.get_data_plane_repo(config).await;
        let dataplane_process_entity = Arc::new(DataPlaneProcessEntityService::new(dataplane_repo.clone()));
        TestingHTTPProxy::new(dataplane_process_entity.clone()).router()
    }
}
