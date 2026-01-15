use sea_orm::entity::prelude::*;
use sea_orm::ActiveValue;
use serde::{Deserialize, Serialize};
use serde_json::Value as Json;
use urn::{Urn, UrnBuilder};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "connector_instances")]
#[serde(rename_all = "camelCase")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: String,
    pub template_id: String,
    pub template_version: String,
    pub distribution_id: String,
    pub created_at: DateTimeWithTimeZone,
    pub configuration_values: Json,
    pub runtime_context: Json,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::connector_templates::Entity",
        from = "(Column::TemplateId, Column::TemplateVersion)",
        to = "(super::connector_templates::Column::Id, super::connector_templates::Column::Version)"
    )]
    ConnectorTemplate,
}

impl Related<super::connector_templates::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ConnectorTemplate.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Clone)]
pub struct NewConnectorInstanceModel {
    pub id: Option<Urn>,
    pub template_id: String,
    pub template_version: String,
    pub distribution_id: String,
    pub configuration_values: Json,
    pub runtime_context: Json,
}

impl From<NewConnectorInstanceModel> for ActiveModel {
    fn from(dto: NewConnectorInstanceModel) -> Self {
        let new_urn = UrnBuilder::new(
            "connector-instance",
            uuid::Uuid::new_v4().to_string().as_str(),
        )
        .build()
        .expect("UrnBuilder failed");
        let now = chrono::Utc::now().into();

        Self {
            id: ActiveValue::Set(dto.id.clone().unwrap_or(new_urn).to_string()),
            template_id: ActiveValue::Set(dto.template_id),
            template_version: ActiveValue::Set(dto.template_version),
            distribution_id: ActiveValue::Set(dto.distribution_id),
            created_at: ActiveValue::Set(now),
            configuration_values: ActiveValue::Set(dto.configuration_values),
            runtime_context: ActiveValue::Set(dto.runtime_context),
        }
    }
}
