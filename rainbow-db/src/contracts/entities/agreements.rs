use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "contract_agreements")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub agreement_id: Uuid,
    pub data_service_id: Uuid,
    pub identity: Option<String>,
    pub identity_token: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
