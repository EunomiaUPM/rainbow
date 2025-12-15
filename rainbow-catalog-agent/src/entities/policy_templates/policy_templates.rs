use crate::data::factory_trait::CatalogAgentRepoTrait;
use crate::entities::data_services::DataServiceDto;
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
            let dto: PolicyTemplateDto = c.into();
            dtos.push(dto);
        }
        Ok(dtos)
    }

    async fn get_batch_policy_templates(&self, ids: &Vec<Urn>) -> anyhow::Result<Vec<PolicyTemplateDto>> {
        let policy_templates =
            self.repo.get_policy_template_repo().get_batch_policy_templates(ids).await.map_err(|e| {
                let err = CommonErrors::database_new(&e.to_string());
                error!("{}", err.log());
                err
            })?;
        let mut dtos = Vec::with_capacity(policy_templates.len());
        for c in policy_templates {
            let dto: PolicyTemplateDto = c.into();
            dtos.push(dto);
        }
        Ok(dtos)
    }

    async fn get_policy_template_by_id(&self, template_id: &Urn) -> anyhow::Result<Option<PolicyTemplateDto>> {
        let policy_template =
            self.repo.get_policy_template_repo(template_id).get_policy_template_by_id().await.map_err(|e| {
                let err = CommonErrors::database_new(&e.to_string());
                error!("{}", err.log());
                err
            })?;
        let dto = policy_template.into();
        Ok(dto)
    }

    async fn create_policy_template(
        &self,
        new_policy_template: &NewPolicyTemplateDto,
    ) -> anyhow::Result<PolicyTemplateDto> {
        let new_model = new_policy_template.into();
        let policy_template =
            self.repo.get_policy_template_repo().create_policy_template(&new_model).await.map_err(|e| {
                let err = CommonErrors::database_new(&e.to_string());
                error!("{}", err.log());
                err
            })?;
        let dto = policy_template.into();
        Ok(dto)
    }

    async fn delete_policy_template_by_id(&self, template_id: &Urn) -> anyhow::Result<()> {
        self.repo.get_policy_template_repo().delete_policy_template_by_id(template_id).await.map_err(|e| {
            let err = CommonErrors::database_new(&e.to_string());
            error!("{}", err.log());
            err
        })?;
        Ok(())
    }
}
