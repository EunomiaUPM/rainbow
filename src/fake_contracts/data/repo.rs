use crate::db::get_db_relational_connection_r2d2;
use crate::fake_contracts::data::models::ContractAgreementsModel;
use crate::fake_contracts::data::schema::contract_agreements::agreement_id;
use crate::fake_contracts::data::schema::contract_agreements::dsl::contract_agreements;
use diesel::prelude::*;
use diesel::row::NamedRow;
use diesel::{OptionalExtension, QueryDsl, RunQueryDsl, SelectableHelper};
use uuid::Uuid;

pub fn create_agreement_repo(dataset_id: Uuid) -> anyhow::Result<ContractAgreementsModel> {
    let connection = &mut get_db_relational_connection_r2d2().get()?;
    let transaction = diesel::insert_into(contract_agreements)
        .values(ContractAgreementsModel {
            agreement_id: Uuid::new_v4(),
            dataset_id,
        })
        .returning(ContractAgreementsModel::as_select())
        .get_result(connection)?;

    Ok(transaction)
}

pub fn get_agreement_by_id_repo(
    agreement_id_in: Uuid,
) -> anyhow::Result<Option<ContractAgreementsModel>> {
    let connection = &mut get_db_relational_connection_r2d2().get()?;
    let transaction = contract_agreements
        .filter(agreement_id.eq(agreement_id_in))
        .select(ContractAgreementsModel::as_select())
        .first(connection)
        .optional()?;

    Ok(transaction)
}

pub fn delete_agreement_repo(agreement_id_in: Uuid) -> anyhow::Result<()> {
    let connection = &mut get_db_relational_connection_r2d2().get()?;
    let _ = diesel::delete(contract_agreements.filter(agreement_id.eq(agreement_id_in)))
        .execute(connection)?;
    Ok(())
}
