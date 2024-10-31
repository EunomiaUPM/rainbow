use crate::transfer::consumer::data::models::TransferCallbacksModel;
use crate::transfer::consumer::data::repo::{TransferConsumerDataRepo, TRANSFER_CONSUMER_REPO};
use uuid::Uuid;

pub async fn create_new_callback() -> anyhow::Result<TransferCallbacksModel> {
    let cb = TRANSFER_CONSUMER_REPO.create_callback()?;
    Ok(cb)
}

pub async fn does_callback_exist(id: Uuid) -> anyhow::Result<bool> {
    let cb = TRANSFER_CONSUMER_REPO.get_callback_by_id(id)?;
    match cb {
        Some(_) => Ok(true),
        None => Ok(false),
    }
}
