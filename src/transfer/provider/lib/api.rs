use crate::transfer::provider::data::models::TransferProcessModel;

pub async fn get_all_transfers() -> anyhow::Result<Vec<TransferProcessModel>> {
    Ok(vec![])
}
