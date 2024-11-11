use crate::fake_contracts::data::models::ContractAgreementsModel;
use crate::fake_contracts::data::repo::{create_agreement_repo, get_agreement_by_id_repo};
use uuid::Uuid;

pub fn create_agreement(dataset_id: Uuid) -> anyhow::Result<ContractAgreementsModel> {
    let agreement = create_agreement_repo(dataset_id)?;
    Ok(agreement)
}

pub fn get_agreement_by_id(agreement_id: Uuid) -> anyhow::Result<Option<ContractAgreementsModel>> {
    let agreement = get_agreement_by_id_repo(agreement_id)?;
    Ok(agreement)
}
