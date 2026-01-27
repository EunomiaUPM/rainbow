use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "connector_distribution_relation")]
#[serde(rename_all = "camelCase")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub distribution_id: String,
    pub connector_instance_id: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::connector_instances::Entity",
        from = "Column::ConnectorInstanceId",
        to = "super::connector_instances::Column::Id"
    )]
    ConnectorInstance,
}

impl Related<super::connector_templates::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ConnectorInstance.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
