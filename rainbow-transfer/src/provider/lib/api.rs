use crate::protocol::messages::TransferProcessMessage;
use crate::provider::data::entities::transfer_process;
use crate::setup::databases::get_db_connection;
use sea_orm::EntityTrait;

pub async fn get_all_transfers() -> anyhow::Result<Vec<TransferProcessMessage>> {
    let db_connection = get_db_connection().await;
    let transfer_processes_from_db = transfer_process::Entity::find().all(db_connection).await?;

    let mut transfer_processes = vec![];
    for tp in transfer_processes_from_db {
        transfer_processes.push(TransferProcessMessage::from(tp));
    }
    Ok(transfer_processes)
}
