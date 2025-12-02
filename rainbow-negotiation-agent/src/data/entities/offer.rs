use rainbow_common::utils::get_urn;
use sea_orm::prelude::{DateTimeWithTimeZone, Json};
use sea_orm::{
    ActiveModelBehavior, ActiveValue, DeriveEntityModel, DerivePrimaryKey, DeriveRelation, EntityTrait, EnumIter,
    PrimaryKeyTrait, Related, RelationDef, RelationTrait,
};
use serde::{Deserialize, Serialize};
use urn::Urn;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "negotiation_agent_offers")]
#[serde(rename_all = "camelCase")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub negotiation_process_id: String,
    pub negotiation_message_id: String,
    pub offer_id: String,
    pub offer_content: Json,
    pub created_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::negotiation_process::Entity",
        from = "Column::NegotiationProcessId",
        to = "super::negotiation_process::Column::Id",
        on_delete = "Cascade"
    )]
    Process,
    #[sea_orm(
        belongs_to = "super::negotiation_message::Entity",
        from = "Column::NegotiationMessageId",
        to = "super::negotiation_message::Column::Id",
        on_delete = "Cascade"
    )]
    Message,
}

impl Related<super::negotiation_process::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Process.def()
    }
}

impl Related<super::negotiation_message::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Message.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

pub struct NewOfferModel {
    pub id: Option<Urn>,
    pub negotiation_process_id: Urn,
    pub negotiation_message_id: Urn,
    pub offer_id: String,
    pub offer_content: Json,
}

impl From<NewOfferModel> for ActiveModel {
    fn from(value: NewOfferModel) -> Self {
        Self {
            id: ActiveValue::Set(value.id.unwrap_or(get_urn(None)).to_string()),
            negotiation_process_id: ActiveValue::Set(value.negotiation_process_id.to_string()),
            negotiation_message_id: ActiveValue::Set(value.negotiation_message_id.to_string()),
            offer_id: ActiveValue::Set(value.offer_id),
            offer_content: ActiveValue::Set(value.offer_content),
            created_at: ActiveValue::Set(chrono::Utc::now().into()),
        }
    }
}
