use crate::protocol::messages::TransferStateForDb;
use sea_orm::entity::prelude::*;
use sea_orm::entity::prelude::*;
use sea_orm::sea_query::ValueType;


#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "transfer_processes")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub provider_pid: Uuid,
    pub consumer_pid: Option<Uuid>,
    pub agreement_id: Uuid,
    pub data_plane_id: Option<Uuid>,
    pub subscription_id: Option<String>,
    pub state: TransferStateForDb,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: Option<chrono::NaiveDateTime>,
    pub data_plane_address: Option<String>,
    pub next_hop_address: Option<serde_json::Value>
}
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::transfer_message::Entity")]
    TransferMessages,
}

impl Related<super::transfer_message::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TransferMessages.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}