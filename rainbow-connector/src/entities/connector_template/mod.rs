pub(crate) mod connector_template;
pub(crate) mod validator;

use crate::data::entities::connector_templates::NewConnectorTemplateModel;
use crate::entities::auth_config::AuthenticationConfig;
use crate::entities::common::parameter_visitor::ParameterVisitor;
use crate::entities::common::parameters::{ParameterDefinition, TemplateVisitable};
use crate::entities::interaction::InteractionConfig;
use sea_orm::prelude::DateTimeWithTimeZone;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConnectorMetadata {
    pub name: Option<String>,
    pub author: Option<String>,
    pub description: Option<String>,
    pub version: Option<String>,
    pub created_at: Option<DateTimeWithTimeZone>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectorTemplateDto {
    #[serde(flatten)]
    pub metadata: ConnectorMetadata,
    pub authentication: AuthenticationConfig,
    pub interaction: InteractionConfig,
    pub parameters: Vec<ParameterDefinition>,
}

impl TemplateVisitable for ConnectorTemplateDto {
    fn accept<V: ParameterVisitor>(&mut self, visitor: &mut V) -> Result<(), V::Error> {
        visitor.enter_scope("authentication");
        self.authentication.accept(visitor)?;
        visitor.exit_scope();

        visitor.enter_scope("interaction");
        self.interaction.accept(visitor)?;
        visitor.exit_scope();

        Ok(())
    }
}

impl TryFrom<ConnectorTemplateDto> for NewConnectorTemplateModel {
    type Error = anyhow::Error;

    fn try_from(value: ConnectorTemplateDto) -> Result<Self, Self::Error> {
        let authentication = serde_json::to_value(value.authentication)?;
        let interaction = serde_json::to_value(value.interaction)?;
        let parameters = serde_json::to_value(value.parameters)?;
        Ok(Self {
            name: Option::from(value.metadata.name.clone()),
            version: Option::from(value.metadata.version.clone()),
            author: Option::from(value.metadata.author.clone()),
            spec: json!({
                "authentication": authentication,
                "interaction": interaction,
                "parameters": parameters,
            }),
        })
    }
}

#[async_trait::async_trait]
pub trait ConnectorTemplateEntitiesTrait: Send + Sync {
    async fn get_all_templates(
        &self,
        limit: Option<u64>,
        page: Option<u64>,
    ) -> anyhow::Result<Vec<ConnectorTemplateDto>>;
    async fn get_templates_by_id(
        &self,
        template_id: &String,
    ) -> anyhow::Result<Vec<ConnectorTemplateDto>>;
    async fn get_template_by_name_and_version(
        &self,
        name: &String,
        version: &String,
    ) -> anyhow::Result<Option<ConnectorTemplateDto>>;
    async fn create_template(
        &self,
        new_template: &mut ConnectorTemplateDto,
    ) -> anyhow::Result<ConnectorTemplateDto>;
    async fn delete_template_by_name_and_version(
        &self,
        name: &String,
        version: &String,
    ) -> anyhow::Result<()>;
}
