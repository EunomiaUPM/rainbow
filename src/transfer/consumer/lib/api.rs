use crate::transfer::consumer::data::models::TransferCallbacksModel;
use crate::transfer::consumer::data::repo::TRANSFER_CONSUMER_REPO;
use uuid::Uuid;

pub async fn get_all_callbacks() -> anyhow::Result<Vec<TransferCallbacksModel>> {
    let callbacks = TRANSFER_CONSUMER_REPO.get_all_callbacks(None)?;
    Ok(callbacks)
}

pub async fn get_callback_by_id(callback_id: Uuid) -> anyhow::Result<Option<TransferCallbacksModel>> {
    let callbacks = TRANSFER_CONSUMER_REPO.get_callback_by_id(callback_id)?;
    Ok(callbacks)
}
