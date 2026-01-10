use crate::data::entities::policy_template;
use crate::data::entities::policy_template::NewPolicyTemplateModel;
use crate::data::repo_traits::catalog_db_errors::CatalogAgentRepoErrors;
use urn::Urn;

#[async_trait::async_trait]
pub trait PolicyTemplatesRepositoryTrait: Send + Sync {
    async fn get_all_policy_templates(
        &self,
        limit: Option<u64>,
        page: Option<u64>,
    ) -> anyhow::Result<Vec<policy_template::Model>, CatalogAgentRepoErrors>;
    async fn get_batch_policy_templates(
        &self,
        ids: &Vec<String>,
    ) -> anyhow::Result<Vec<policy_template::Model>, CatalogAgentRepoErrors>;
    async fn get_policy_templates_by_id(
        &self,
        template_id: &String,
    ) -> anyhow::Result<Vec<policy_template::Model>, CatalogAgentRepoErrors>;
    async fn get_policy_template_by_id_and_version(
        &self,
        template_id: &String,
        version: &String,
    ) -> anyhow::Result<Option<policy_template::Model>, CatalogAgentRepoErrors>;
    async fn create_policy_template(
        &self,
        new_policy_template: &NewPolicyTemplateModel,
    ) -> anyhow::Result<policy_template::Model, CatalogAgentRepoErrors>;
    async fn delete_policy_template_by_id_and_version(
        &self,
        template_id: &String,
        version: &String,
    ) -> anyhow::Result<(), CatalogAgentRepoErrors>;
}
