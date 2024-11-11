use crate::fake_contracts::data::schema::contract_agreements;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(
    Queryable,
    Identifiable,
    Selectable,
    Debug,
    PartialEq,
    Insertable,
    Clone,
    Serialize,
    Deserialize
)]
#[diesel(table_name=contract_agreements)]
#[diesel(primary_key(agreement_id))]
pub struct ContractAgreementsModel {
    pub agreement_id: Uuid,
    pub dataset_id: Uuid,
}
