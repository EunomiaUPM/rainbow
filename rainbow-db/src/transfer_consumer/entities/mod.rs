pub mod transfer_callback;

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "transfer_callbacks")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub consumer_pid: Uuid,
    pub provider_pid: Option<String>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: Option<chrono::NaiveDateTime>,
    pub data_address: serde_json::Value,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
