use crate::transfer::protocol::messages::{TransferMessageTypes, TransferState};
use crate::transfer::provider::data::models::{
    DataPlaneProcessModel, TransferMessageModel, TransferProcessModel,
};
use once_cell::sync::Lazy;

use crate::config::GLOBAL_CONFIG;
use crate::transfer::provider::data::repo_memory::TransferProviderDataRepoMemory;
use crate::transfer::provider::data::repo_mongo::TransferProviderDataRepoMongo;
use crate::transfer::provider::data::repo_postgres::TransferProviderDataRepoPostgres;
use uuid::Uuid;
// https://chatgpt.com/c/67069598-decc-800f-a673-3b6b84aeeca9

pub trait TransferProviderDataRepo {
    fn get_all_transfer_processes(
        &self,
        limit: Option<i64>,
    ) -> anyhow::Result<Vec<TransferProcessModel>>;
    fn get_transfer_process_by_consumer_pid(
        &self,
        consumer_pid_in: Uuid,
    ) -> anyhow::Result<Option<TransferProcessModel>>;
    fn get_transfer_process_by_provider_pid(
        &self,
        provider_pid_in: Uuid,
    ) -> anyhow::Result<Option<TransferProcessModel>>;
    fn create_transfer_process(&self, transfer_process: TransferProcessModel)
                               -> anyhow::Result<()>;
    fn update_transfer_process_by_provider_pid(
        &self,
        provider_pid_in: &Uuid,
        new_state: TransferState,
    ) -> anyhow::Result<Option<TransferProcessModel>>;
    fn get_all_transfer_messages(
        &self,
        limit: Option<i64>,
    ) -> anyhow::Result<Vec<TransferMessageModel>>;
    fn get_all_transfer_messages_by_type(
        &self,
        message_type_in: TransferMessageTypes,
        limit: Option<i64>,
    ) -> anyhow::Result<Vec<TransferMessageModel>>;
    fn get_transfer_message_by_id(
        &self,
        message_id_in: Uuid,
    ) -> anyhow::Result<Option<TransferMessageModel>>;
    fn create_transfer_message(&self, message: TransferMessageModel) -> anyhow::Result<()>;
    fn create_data_plane_process(
        &self,
        data_plane_process: DataPlaneProcessModel,
    ) -> anyhow::Result<DataPlaneProcessModel>;
    fn update_data_plane_process(
        &self,
        data_plane_process_id: Uuid,
        new_state: bool,
    ) -> anyhow::Result<Option<DataPlaneProcessModel>>;
    fn get_data_plane_process_by_id(
        &self,
        data_plane_id: Uuid,
    ) -> anyhow::Result<Option<DataPlaneProcessModel>>;
    fn get_data_plane_process_by_transfer_process_id(
        &self,
        transfer_process_id: Uuid,
    ) -> anyhow::Result<Option<DataPlaneProcessModel>>;
    fn delete_data_plane_process_by_id(&self, data_plane_id: Uuid) -> anyhow::Result<()>;
}

pub static TRANSFER_PROVIDER_REPO: Lazy<Box<dyn TransferProviderDataRepo + Send + Sync>> = Lazy::new(|| {
    let repo_type = GLOBAL_CONFIG.get().unwrap().db_type.clone();

    match repo_type.as_str() {
        "postgres" => Box::new(TransferProviderDataRepoPostgres::new()),
        "mongo" => Box::new(TransferProviderDataRepoMongo::new()),
        "memory" => Box::new(TransferProviderDataRepoMemory::new()),
        _ => panic!("Unknown REPO_TYPE: {}", repo_type),
    }
});
