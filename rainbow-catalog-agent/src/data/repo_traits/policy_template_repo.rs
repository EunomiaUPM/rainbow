use crate::data::entities::policy_template;
use crate::data::entities::policy_template::NewPolicyTemplateModel;
use anyhow::Error;
use thiserror::Error;
use urn::Urn;

#[async_trait::async_trait]
pub trait PolicyTemplatesRepositoryTrait: Send + Sync {
    async fn get_all_policy_templates(
        &self,
        limit: Option<u64>,
        page: Option<u64>,
    ) -> anyhow::Result<Vec<policy_template::Model>, PolicyTemplatesRepoErrors>;
    async fn get_batch_policy_templates(
        &self,
        ids: &Vec<Urn>,
    ) -> anyhow::Result<Vec<policy_template::Model>, PolicyTemplatesRepoErrors>;
    async fn get_policy_template_by_id(
        &self,
        template_id: &Urn,
    ) -> anyhow::Result<Option<policy_template::Model>, PolicyTemplatesRepoErrors>;
    async fn create_policy_template(
        &self,
        new_policy_template: &NewPolicyTemplateModel,
    ) -> anyhow::Result<policy_template::Model, PolicyTemplatesRepoErrors>;
    async fn delete_policy_template_by_id(&self, template_id: &Urn) -> anyhow::Result<(), PolicyTemplatesRepoErrors>;
}

#[derive(Error, Debug)]
pub enum PolicyTemplatesRepoErrors {
    #[error("PolicyTemplate not found")]
    PolicyTemplateNotFound,
    #[error("Error fetching policy template. {0}")]
    ErrorFetchingPolicyTemplate(Error),
    #[error("Error creating policy template. {0}")]
    ErrorCreatingPolicyTemplate(Error),
    #[error("Error deleting policy template. {0}")]
    ErrorDeletingPolicyTemplate(Error),
}
