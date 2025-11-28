use crate::data::factory_trait::DataPlaneRepoTrait;
use crate::data::repo_sql::data_plane_fields_repo::DataPlaneFieldRepoForSql;
use crate::data::repo_sql::data_plane_process_repo::DataPlaneProcessRepoForSql;
use crate::data::repo_sql::transfer_event_repo::TransferEventRepoForSql;
use crate::data::repo_traits::data_plane_fields_repo::DataPlaneFieldRepoTrait;
use crate::data::repo_traits::data_plane_process_repo::DataPlaneProcessRepoTrait;
use crate::data::repo_traits::transfer_event_repo::TransferEventRepo;
use sea_orm::DatabaseConnection;
use std::sync::Arc;

pub struct DataPlaneRepoForSql {
    dataplane_process_repo: Arc<dyn DataPlaneProcessRepoTrait>,
    dataplane_fields_repo: Arc<dyn DataPlaneFieldRepoTrait>,
    transfer_events_repo: Arc<dyn TransferEventRepo>,
}

impl DataPlaneRepoForSql {
    pub fn create_repo(db_connection: DatabaseConnection) -> Self {
        Self {
            dataplane_process_repo: Arc::new(DataPlaneProcessRepoForSql::new(db_connection.clone())),
            dataplane_fields_repo: Arc::new(DataPlaneFieldRepoForSql::new(db_connection.clone())),
            transfer_events_repo: Arc::new(TransferEventRepoForSql::new(db_connection.clone())),
        }
    }
}

impl DataPlaneRepoTrait for DataPlaneRepoForSql {
    fn get_data_plane_process_repo(&self) -> Arc<dyn DataPlaneProcessRepoTrait> {
        self.dataplane_process_repo.clone()
    }

    fn get_data_plane_fields_repo(&self) -> Arc<dyn DataPlaneFieldRepoTrait> {
        self.dataplane_fields_repo.clone()
    }

    fn get_transfer_events_repo(&self) -> Arc<dyn TransferEventRepo> {
        self.transfer_events_repo.clone()
    }
}
