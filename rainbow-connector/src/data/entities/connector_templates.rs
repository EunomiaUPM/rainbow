use sea_orm::entity::prelude::*;
use sea_orm::ActiveValue;
use serde::{Deserialize, Serialize};
use serde_json::Value as Json;
use urn::UrnBuilder;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "connector_templates")]
#[serde(rename_all = "camelCase")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub name: String,
    #[sea_orm(primary_key, auto_increment = false)]
    pub version: String,
    pub author: String,
    pub created_at: DateTimeWithTimeZone,
    pub spec: Json,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::connector_instances::Entity")]
    ConnectorInstance,
}

impl Related<super::connector_instances::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ConnectorInstance.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Clone)]
pub struct NewConnectorTemplateModel {
    pub name: Option<String>,
    pub version: Option<String>,
    pub author: Option<String>,
    pub spec: Json,
}

impl From<NewConnectorTemplateModel> for ActiveModel {
    fn from(dto: NewConnectorTemplateModel) -> Self {
        let new_urn = UrnBuilder::new(
            "connector-template",
            uuid::Uuid::new_v4().to_string().as_str(),
        )
        .build()
        .expect("UrnBuilder failed");

        Self {
            name: ActiveValue::Set(dto.name.clone().unwrap_or(new_urn.to_string()).to_string()),
            version: ActiveValue::Set(dto.name.clone().unwrap_or("1.0".to_string()).to_string()),
            author: ActiveValue::Set(dto.name.clone().unwrap_or("admin".to_string()).to_string()),
            created_at: ActiveValue::Set(chrono::Utc::now().into()),
            spec: ActiveValue::Set(dto.spec),
        }
    }
}
