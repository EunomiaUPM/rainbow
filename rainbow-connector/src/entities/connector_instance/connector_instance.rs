use crate::data::entities::connector_instances;
use crate::data::factory_trait::ConnectorRepoTrait;
use crate::entities::auth_config::AuthenticationConfig;
use crate::entities::connector_instance::{
    ConnectorInstanceDto, ConnectorInstanceTrait, ConnectorInstantiationDto,
};
use crate::entities::connector_template::ConnectorMetadata;
use crate::entities::interaction::InteractionConfig;
use crate::facades::distribution_resolver_facade::DistributionFacadeTrait;
use log::error;
use rainbow_common::errors::{CommonErrors, ErrorLog};
use std::str::FromStr;
use std::sync::Arc;
use urn::Urn;

pub struct ConnectorInstanceEntitiesService {
    repo: Arc<dyn ConnectorRepoTrait>,
    distribution_facade: Arc<dyn DistributionFacadeTrait>,
}

impl ConnectorInstanceEntitiesService {
    pub fn new(
        repo: Arc<dyn ConnectorRepoTrait>,
        distribution_facade: Arc<dyn DistributionFacadeTrait>,
    ) -> Self {
        Self { repo, distribution_facade }
    }

    fn map_model_to_dto(model: connector_instances::Model) -> anyhow::Result<ConnectorInstanceDto> {
        let auth_config: AuthenticationConfig =
            serde_json::from_value(model.authentication.clone()).map_err(|e| {
                let err = CommonErrors::parse_new(&format!(
                    "Error deserializing authentication config: {}",
                    e
                ));
                error!("{}", err.log());
                err
            })?;

        let interaction_config: InteractionConfig =
            serde_json::from_value(model.interaction.clone()).map_err(|e| {
                let err = CommonErrors::parse_new(&format!(
                    "Error deserializing interaction config: {}",
                    e
                ));
                error!("{}", err.log());
                err
            })?;

        let urn = Urn::from_str(&model.id).map_err(|e| {
            let err = CommonErrors::parse_new(&format!("Error parsing URN: {}", e));
            error!("{}", err.log());
            err
        })?;

        let distribution_urn = Urn::from_str(&model.distribution_id).map_err(|e| {
            let err = CommonErrors::parse_new(&format!("Error parsing Distribution URN: {}", e));
            error!("{}", err.log());
            err
        })?;

        Ok(ConnectorInstanceDto {
            id: urn,
            metadata: ConnectorMetadata {
                name: Some(model.template_name),
                author: None,                          // Not available in instance model
                version: Some(model.template_version), // available in instance model
                created_at: Some(model.created_at),
            },
            authentication_config: auth_config,
            interaction: interaction_config,
            distribution_id: distribution_urn,
        })
    }
}

#[async_trait::async_trait]
impl ConnectorInstanceTrait for ConnectorInstanceEntitiesService {
    async fn get_instance_by_id(&self, id: &Urn) -> anyhow::Result<Option<ConnectorInstanceDto>> {
        let id_str = id.to_string();
        let instance =
            self.repo.get_instances_repo().get_instance_by_id(&id_str).await.map_err(|e| {
                let err = CommonErrors::database_new(&e.to_string());
                error!("{}", err.log());
                err
            })?;

        match instance {
            Some(model) => Ok(Some(Self::map_model_to_dto(model)?)),
            None => Ok(None),
        }
    }

    async fn get_instance_by_distribution(
        &self,
        distribution_id: &Urn,
    ) -> anyhow::Result<Option<ConnectorInstanceDto>> {
        let dist_id_str = distribution_id.to_string();

        let result = self
            .repo
            .get_instances_repo()
            .get_instances_by_distribution(&dist_id_str)
            .await
            .map_err(|e| {
                let err = CommonErrors::database_new(&e.to_string());
                error!("{}", err.log());
                err
            })?;

        match result {
            Some(model) => Ok(Some(Self::map_model_to_dto(model)?)),
            None => Ok(None),
        }
    }

    async fn upsert_instance(
        &self,
        instance_dto: &mut ConnectorInstantiationDto,
    ) -> anyhow::Result<ConnectorInstanceDto> {
        // fetch template
        let template = self
            .repo
            .get_templates_repo()
            .get_template_by_name_and_version(
                &instance_dto.template_name,
                &instance_dto.template_version,
            )
            .await
            .map_err(|e| {
                let err = CommonErrors::database_new(&e.to_string());
                error!("{}", err.log());
                err
            })?;

        let template_model = match template {
            Some(t) => t,
            None => {
                let err = CommonErrors::missing_resource_new(
                    "template",
                    &format!(
                        "Template {} {} not found",
                        instance_dto.template_name, instance_dto.template_version
                    ),
                );
                error!("{}", err.log());
                return Err(anyhow::anyhow!(err));
            }
        };

        // fetch distribution
        let distribution_id = instance_dto.distribution_id.to_string();
        let _ =
            self.distribution_facade.resolve_distribution_by_id(&distribution_id).await.map_err(
                |e| {
                    let err = CommonErrors::parse_new(&format!(
                        "Error resolving associated distribution: {}",
                        e
                    ));
                    error!("{}", err.log());
                    err
                },
            )?;

        // edit or create
        let _ = self
            .repo
            .get_instances_repo()
            .get_instances_by_distribution(&distribution_id)
            .await
            .map_err(|e| {
                let err = CommonErrors::database_new(&e.to_string());
                error!("{}", err.log());
                err
            })?;

        // meter relation

        // prepare data
        let metadata_json = serde_json::to_value(&instance_dto.metadata).map_err(|e| {
            let err = CommonErrors::parse_new(&format!("Error serializing metadata: {}", e));
            error!("{}", err.log());
            err
        })?;

        let params_json = serde_json::to_value(&instance_dto.parameters).map_err(|e| {
            let err = CommonErrors::parse_new(&format!("Error serializing parameters: {}", e));
            error!("{}", err.log());
            err
        })?;

        let auth_json = template_model.spec["authentication"].clone();
        let inter_json = template_model.spec["interaction"].clone();

        // persist instance
        let new_instance = connector_instances::NewConnectorInstanceModel {
            id: None,
            template_name: instance_dto.template_name.clone(),
            template_version: instance_dto.template_version.clone(),
            distribution_id: distribution_id,
            metadata: metadata_json,
            configuration_parameters: params_json,
            authentication: auth_json,
            interaction: inter_json,
        };
        let saved_model =
            self.repo.get_instances_repo().create_instance(&new_instance).await.map_err(|e| {
                let err = CommonErrors::database_new(&e.to_string());
                error!("{}", err.log());
                err
            })?;

        // persist relation

        // 5. Return DTO
        Self::map_model_to_dto(saved_model)
    }

    async fn delete_instance_by_id(&self, id: &Urn) -> anyhow::Result<()> {
        let id_str = id.to_string();
        self.repo.get_instances_repo().delete_instance_by_id(&id_str).await.map_err(|e| {
            let err = CommonErrors::database_new(&e.to_string());
            error!("{}", err.log());
            err
        })?;
        Ok(())
    }
}
