use crate::data::repo_traits::data_plane_fields_repo::DataPlaneFieldRepoTrait;
use crate::data::repo_traits::data_plane_process_repo::DataPlaneProcessRepoTrait;
use crate::data::repo_traits::transfer_event_repo::TransferEventRepo;
use std::sync::Arc;

pub trait DataPlaneRepoTrait: Send + Sync + 'static {
    fn get_data_plane_process_repo(&self) -> Arc<dyn DataPlaneProcessRepoTrait>;
    fn get_data_plane_fields_repo(&self) -> Arc<dyn DataPlaneFieldRepoTrait>;
    fn get_transfer_events_repo(&self) -> Arc<dyn TransferEventRepo>;
}
