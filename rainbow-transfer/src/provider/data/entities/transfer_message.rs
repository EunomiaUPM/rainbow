use rainbow_common::protocol::transfer::{TransferMessageTypesForDb, TransferRoles};
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "transfer_messages")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub transfer_process_id: Uuid,
    pub created_at: chrono::NaiveDateTime,
    pub message_type: TransferMessageTypesForDb,
    pub from: TransferRoles,
    pub to: TransferRoles,
    pub content: serde_json::Value,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::transfer_process::Entity",
        from = "Column::TransferProcessId",
        to = "super::transfer_process::Column::ProviderPid"
    )]
    TransferProcess,
}

impl Related<super::transfer_process::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TransferProcess.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
