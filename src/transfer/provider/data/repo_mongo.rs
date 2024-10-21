use crate::transfer::protocol::messages::{TransferMessageTypes, TransferState};
use crate::transfer::provider::data::models::{
    TransferMessageModel, TransferProcessModel,
};
use crate::transfer::provider::data::repo::TransferProviderDataRepo;
use uuid::Uuid;

pub struct TransferProviderDataRepoMongo {
    client: Option<String>,
}
impl TransferProviderDataRepoMongo {
    pub fn new() -> Self {
        todo!();
    }
}
impl TransferProviderDataRepo for TransferProviderDataRepoMongo {
    fn get_all_transfer_processes(
        &self,
        limit: Option<i64>,
    ) -> anyhow::Result<Vec<TransferProcessModel>> {
        todo!()
    }

    fn get_transfer_process_by_consumer_pid(
        &self,
        consumer_pid_in: Uuid,
    ) -> anyhow::Result<Option<TransferProcessModel>> {
        todo!()
    }

    fn get_transfer_process_by_provider_pid(
        &self,
        provider_pid_in: Uuid,
    ) -> anyhow::Result<Option<TransferProcessModel>> {
        todo!()
    }

    fn get_transfer_process_by_data_plane_process(&self, data_plane_process: Uuid) -> anyhow::Result<Option<TransferProcessModel>> {
        todo!()
    }

    fn create_transfer_process(
        &self,
        transfer_process: TransferProcessModel,
    ) -> anyhow::Result<()> {
        todo!()
    }

    fn update_transfer_process_by_provider_pid(
        &self,
        provider_pid_in: &Uuid,
        new_state: TransferState,
        new_data_plane_id: Option<Uuid>,
    ) -> anyhow::Result<Option<TransferProcessModel>> {
        todo!()
    }

    fn get_all_transfer_messages(
        &self,
        limit: Option<i64>,
    ) -> anyhow::Result<Vec<TransferMessageModel>> {
        todo!()
    }

    fn get_all_transfer_messages_by_type(
        &self,
        message_type_in: TransferMessageTypes,
        limit: Option<i64>,
    ) -> anyhow::Result<Vec<TransferMessageModel>> {
        todo!()
    }

    fn get_transfer_message_by_id(
        &self,
        message_id_in: Uuid,
    ) -> anyhow::Result<Option<TransferMessageModel>> {
        todo!()
    }

    fn create_transfer_message(&self, message: TransferMessageModel) -> anyhow::Result<()> {
        todo!()
    }
}
