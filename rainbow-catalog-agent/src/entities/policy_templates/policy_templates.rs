use crate::data::entities::policy_template::NewPolicyTemplateModel;
use crate::data::factory_trait::CatalogAgentRepoTrait;
use crate::entities::policy_templates::{NewPolicyTemplateDto, PolicyTemplateDto, PolicyTemplateEntityTrait};
use rainbow_common::errors::{CommonErrors, ErrorLog};
use std::sync::Arc;
use tracing::error;
use urn::Urn;

pub struct PolicyTemplateEntities {
    repo: Arc<dyn CatalogAgentRepoTrait>,
}

impl PolicyTemplateEntities {
    pub fn new(repo: Arc<dyn CatalogAgentRepoTrait>) -> Self {
        Self { repo }
    }
}

#[async_trait::async_trait]
impl PolicyTemplateEntityTrait for PolicyTemplateEntities {
    async fn get_all_policy_templates(
        &self,
        limit: Option<u64>,
        page: Option<u64>,
    ) -> anyhow::Result<Vec<PolicyTemplateDto>> {
        let policy_templates =
            self.repo.get_policy_template_repo().get_all_policy_templates(limit, page).await.map_err(|e| {
                let err = CommonErrors::database_new(&e.to_string());
                error!("{}", err.log());
                err
            })?;
        let mut dtos = Vec::with_capacity(policy_templates.len());
        for c in policy_templates {
            let dto: PolicyTemplateDto = PolicyTemplateDto::try_from(c)?;
            dtos.push(dto);
        }
        Ok(dtos)
    }

    async fn get_batch_policy_templates(&self, ids: &Vec<String>) -> anyhow::Result<Vec<PolicyTemplateDto>> {
        let policy_templates =
            self.repo.get_policy_template_repo().get_batch_policy_templates(ids).await.map_err(|e| {
                let err = CommonErrors::database_new(&e.to_string());
                error!("{}", err.log());
                err
            })?;
        let mut dtos = Vec::with_capacity(policy_templates.len());
        for c in policy_templates {
            let dto: PolicyTemplateDto = PolicyTemplateDto::try_from(c)?;
            dtos.push(dto);
        }
        Ok(dtos)
    }

    async fn get_policies_template_by_id(&self, template_id: &String) -> anyhow::Result<Vec<PolicyTemplateDto>> {
        let policy_templates =
            self.repo.get_policy_template_repo().get_policy_templates_by_id(template_id).await.map_err(|e| {
                let err = CommonErrors::database_new(&e.to_string());
                error!("{}", err.log());
                err
            })?;
        let mut dtos = Vec::with_capacity(policy_templates.len());
        for c in policy_templates {
            let dto: PolicyTemplateDto = PolicyTemplateDto::try_from(c)?;
            dtos.push(dto);
        }
        Ok(dtos)
    }

    async fn get_policies_template_by_version_and_id(
        &self,
        template_id: &String,
        version_id: &String,
    ) -> anyhow::Result<Option<PolicyTemplateDto>> {
        let policy_templates = self
            .repo
            .get_policy_template_repo()
            .get_policy_template_by_id_and_version(template_id, version_id)
            .await
            .map_err(|e| {
                let err = CommonErrors::database_new(&e.to_string());
                error!("{}", err.log());
                err
            })?;
        let dto: Option<PolicyTemplateDto> = policy_templates.map(TryInto::try_into).transpose()?;
        Ok(dto)
    }

    async fn create_policy_template(
        &self,
        new_policy_template: &NewPolicyTemplateDto,
    ) -> anyhow::Result<PolicyTemplateDto> {
        new_policy_template.validate_dto().map_err(|e| {
            let err = CommonErrors::parse_new(&e.to_string());
            error!("{}", err.log());
            err
        })?;
        let new_model: NewPolicyTemplateModel = new_policy_template.clone().try_into()?;
        let policy_template =
            self.repo.get_policy_template_repo().create_policy_template(&new_model).await.map_err(|e| {
                let err = CommonErrors::database_new(&e.to_string());
                error!("{}", err.log());
                err
            })?;
        let dto: PolicyTemplateDto = PolicyTemplateDto::try_from(policy_template)?;
        Ok(dto)
    }

    async fn delete_policy_template_by_version_and_id(
        &self,
        template_id: &String,
        version_id: &String,
    ) -> anyhow::Result<()> {
        let _ = self
            .repo
            .get_policy_template_repo()
            .delete_policy_template_by_id_and_version(template_id, version_id)
            .await
            .map_err(|e| {
                let err = CommonErrors::database_new(&e.to_string());
                error!("{}", err.log());
                err
            })?;
        Ok(())
    }
}
