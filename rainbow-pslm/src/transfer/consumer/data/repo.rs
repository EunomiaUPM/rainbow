use crate::setup::config::GLOBAL_CONFIG;
use crate::transfer::consumer::data::models::{TransferCallbacksModel, TransferCallbacksModelNewState};
use crate::transfer::consumer::data::repo_memory::TransferConsumerDataRepoMemory;
use crate::transfer::consumer::data::repo_mongo::TransferConsumerDataRepoMongo;
use crate::transfer::consumer::data::repo_postgres::TransferConsumerDataRepoPostgres;
use crate::transfer::protocol::messages::DataAddress;
use once_cell::sync::Lazy;
use uuid::Uuid;

pub trait TransferConsumerDataRepo {
    fn get_all_callbacks(&self, limit: Option<i64>) -> anyhow::Result<Vec<TransferCallbacksModel>>;
    fn get_callback_by_id(
        &self,
        callback_id: Uuid,
    ) -> anyhow::Result<Option<TransferCallbacksModel>>;
    fn get_callback_by_consumer_id(
        &self,
        consumer_pid_in: Uuid,
    ) -> anyhow::Result<Option<TransferCallbacksModel>>;
    fn create_callback(&self) -> anyhow::Result<TransferCallbacksModel>;
    fn create_callback_with_data_address(&self, data_address_in: DataAddress) -> anyhow::Result<TransferCallbacksModel>;
    fn update_callback(
        &self,
        callback_id: Uuid,
        new_state: TransferCallbacksModelNewState,
    ) -> anyhow::Result<Option<TransferCallbacksModel>>;
    fn delete_callback(&self, callback_id: Uuid) -> anyhow::Result<()>;
}

pub static TRANSFER_CONSUMER_REPO: Lazy<Box<dyn TransferConsumerDataRepo + Send + Sync>> =
    Lazy::new(|| {
        let repo_type = GLOBAL_CONFIG.get().unwrap().db_type.clone();

        match repo_type.as_str() {
            "postgres" => Box::new(TransferConsumerDataRepoPostgres::new()),
            "mongo" => Box::new(TransferConsumerDataRepoMongo::new()),
            "memory" => Box::new(TransferConsumerDataRepoMemory::new()),
            _ => panic!("Unknown REPO_TYPE: {}", repo_type),
        }
    });
