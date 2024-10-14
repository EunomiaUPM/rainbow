use crate::transfer::consumer::data::models::{TransferCallbacksModel, TransferCallbacksModelNewState};
use crate::transfer::consumer::data::repo::TransferConsumerDataRepo;
use uuid::Uuid;

pub struct TransferConsumerDataRepoMemory {
    client: Option<String>,
}
impl TransferConsumerDataRepoMemory {
    pub fn new() -> Self {
        todo!();
    }
}
impl TransferConsumerDataRepo for TransferConsumerDataRepoMemory {
    fn get_all_callbacks(&self, limit: Option<i64>) -> anyhow::Result<Vec<TransferCallbacksModel>> {
        todo!()
    }
    fn get_callback_by_id(
        &self,
        callback_id: Uuid,
    ) -> anyhow::Result<Option<TransferCallbacksModel>> {
        todo!()
    }

    fn get_callback_by_consumer_id(
        &self,
        consumer_pid_in: Uuid,
    ) -> anyhow::Result<Option<TransferCallbacksModel>> {
        todo!()
    }

    fn create_callback(&self) -> anyhow::Result<TransferCallbacksModel> {
        todo!()
    }
    fn update_callback(
        &self,
        callback_id: Uuid,
        new_state: TransferCallbacksModelNewState,
    ) -> anyhow::Result<Option<TransferCallbacksModel>> {
        todo!()
    }

    fn delete_callback(&self, callback_id: Uuid) -> anyhow::Result<()> {
        todo!()
    }
}
