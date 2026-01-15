use crate::data::entities::connector_templates;
use crate::data::entities::connector_templates::NewConnectorTemplateModel;
use crate::data::factory_trait::ConnectorRepoTrait;
use crate::entities::auth_config::AuthenticationConfig;
use crate::entities::common::parameters::ParameterDefinition;
use crate::entities::connector_template::{ConnectorMetadata, ConnectorTemplateDto, ConnectorTemplateEntitiesTrait};
use crate::entities::interaction::InteractionConfig;
use log::error;
use rainbow_common::errors::{CommonErrors, ErrorLog};
use std::sync::Arc;

pub struct ConnectorTemplateEntitiesService {
    repo: Arc<dyn ConnectorRepoTrait>,
}

impl ConnectorTemplateEntitiesService {
    pub fn new(repo: Arc<dyn ConnectorRepoTrait>) -> Self {
        Self { repo }
    }

    fn map_model_to_dto(model: connector_templates::Model) -> anyhow::Result<ConnectorTemplateDto> {
        let spec = model.spec;
        let authentication: AuthenticationConfig = serde_json::from_value(
            spec.get("authentication")
                .ok_or_else(|| anyhow::anyhow!("Missing 'authentication' in template spec"))?
                .clone(),
        )
        .map_err(|e| anyhow::anyhow!("Error deserializing authentication config: {}", e))?;

        let interaction: InteractionConfig = serde_json::from_value(
            spec.get("interaction").ok_or_else(|| anyhow::anyhow!("Missing 'interaction' in template spec"))?.clone(),
        )
        .map_err(|e| anyhow::anyhow!("Error deserializing interaction config: {}", e))?;

        let parameters: Vec<ParameterDefinition> = serde_json::from_value(
            spec.get("parameters").ok_or_else(|| anyhow::anyhow!("Missing 'parameters' in template spec"))?.clone(),
        )
        .map_err(|e| anyhow::anyhow!("Error deserializing parameters: {}", e))?;

        Ok(ConnectorTemplateDto {
            metadata: ConnectorMetadata {
                name: model.name,
                author: model.author,
                version: model.version,
                created_at: model.created_at,
            },
            authentication,
            interaction,
            parameters,
        })
    }
}

#[async_trait::async_trait]
impl ConnectorTemplateEntitiesTrait for ConnectorTemplateEntitiesService {
    async fn get_all_templates(
        &self,
        limit: Option<u64>,
        page: Option<u64>,
    ) -> anyhow::Result<Vec<ConnectorTemplateDto>> {
        let models = self.repo.get_templates_repo().get_all_templates(limit, page).await.map_err(|e| {
            let err = CommonErrors::database_new(&e.to_string());
            error!("{}", err.log());
            err
        })?;

        let mut dtos = Vec::with_capacity(models.len());
        for model in models {
            dtos.push(Self::map_model_to_dto(model)?);
        }

        Ok(dtos)
    }

    async fn get_templates_by_id(&self, template_id: &String) -> anyhow::Result<Vec<ConnectorTemplateDto>> {
        let models = self.repo.get_templates_repo().get_templates_by_id(template_id).await.map_err(|e| {
            let err = CommonErrors::database_new(&e.to_string());
            error!("{}", err.log());
            err
        })?;

        let mut dtos = Vec::with_capacity(models.len());
        for model in models {
            dtos.push(Self::map_model_to_dto(model)?);
        }
        Ok(dtos)
    }

    async fn get_template_by_name_and_version(
        &self,
        name: &String,
        version: &String,
    ) -> anyhow::Result<Option<ConnectorTemplateDto>> {
        let result =
            self.repo.get_templates_repo().get_template_by_name_and_version(name, version).await.map_err(|e| {
                let err = CommonErrors::database_new(&e.to_string());
                error!("{}", err.log());
                err
            })?;

        match result {
            Some(model) => Ok(Some(Self::map_model_to_dto(model)?)),
            None => Ok(None),
        }
    }

    async fn create_template(&self, new_template: &ConnectorTemplateDto) -> anyhow::Result<ConnectorTemplateDto> {
        let new_model: NewConnectorTemplateModel = new_template.clone().try_into().map_err(|e: anyhow::Error| {
            let err = CommonErrors::parse_new(&format!("Error preparing template model: {}", e));
            error!("{}", err.log());
            err
        })?;

        let saved_model = self.repo.get_templates_repo().create_template(&new_model).await.map_err(|e| {
            let err = CommonErrors::database_new(&e.to_string());
            error!("{}", err.log());
            err
        })?;

        Self::map_model_to_dto(saved_model)
    }

    async fn delete_template_by_name_and_version(&self, name: &String, version: &String) -> anyhow::Result<()> {
        self.repo.get_templates_repo().delete_template_by_name_and_version(name, version).await.map_err(|e| {
            let err = CommonErrors::database_new(&e.to_string());
            error!("{}", err.log());
            err
        })?;

        Ok(())
    }
}
