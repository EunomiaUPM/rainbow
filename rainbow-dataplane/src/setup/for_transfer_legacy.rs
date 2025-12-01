use crate::coordinator::dataplane_access_controller::dataplane_access_controller::DataPlaneAccessControllerService;
use crate::coordinator::dataplane_access_controller::DataPlaneAccessControllerTrait;
use crate::coordinator::dataplane_process::dataplane_process_service::DataPlaneProcessService;
use crate::data::factory_sql::DataPlaneRepoForSql;
use crate::data::factory_trait::DataPlaneRepoTrait;
use crate::entities::data_plane_process::data_plane_process_entity::DataPlaneProcessEntityService;
use crate::entities::transfer_events::transfer_event_entity::TransferEventEntityService;
use crate::http::dataplane_info::DataPlaneRouter;
use crate::http::transfer_events::TransferEventsRouter;
use axum::Router;
use rainbow_common::config::global_config::ApplicationGlobalConfig;
use sea_orm::Database;
use std::sync::Arc;

pub struct DataplaneSetupLegacy {}
impl DataplaneSetupLegacy {
    pub fn new() -> Self {
        DataplaneSetupLegacy {}
    }
    pub async fn get_data_plane_repo(&self, config: &ApplicationGlobalConfig) -> Arc<dyn DataPlaneRepoTrait> {
        let db_url = config.database_config.as_db_url();
        let db_connection = Database::connect(&db_url).await.expect("Database can't connect");
        let dataplane_repo = Arc::new(DataPlaneRepoForSql::create_repo(db_connection.clone()));
        dataplane_repo
    }
    pub async fn get_data_plane_controller(
        &self,
        config: Arc<ApplicationGlobalConfig>,
    ) -> Arc<dyn DataPlaneAccessControllerTrait> {
        let dataplane_repo = self.get_data_plane_repo(config.as_ref()).await;
        let dataplane_process_entity = Arc::new(DataPlaneProcessEntityService::new(dataplane_repo.clone()));
        let dataplane_process_service = Arc::new(DataPlaneProcessService::new(
            dataplane_process_entity.clone(),
        ));
        let controller = Arc::new(DataPlaneAccessControllerService::new(
            config.clone(),
            dataplane_process_service.clone(),
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
        Router::new().nest("/info", dataplane_router).nest("/transfer-events", transfer_event_router)
    }
    pub fn build_testing_proxy(&self) -> Router {
        Router::new()
    }
}
