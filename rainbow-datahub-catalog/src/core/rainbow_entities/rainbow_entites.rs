use crate::core::rainbow_entities::policy_templates_types::PolicyTemplateDatasetRelation;
use crate::core::rainbow_entities::PolicyTemplatesToDatahubDatasetRelationTrait;
use axum::async_trait;
use rainbow_common::config::provider_config::ApplicationProviderConfig;
use rainbow_db::datahub::repo::{PolicyRelationsRepo, PolicyTemplatesRepo};
use std::sync::Arc;
use urn::Urn;

pub struct PolicyTemplatesToDatahubDatasetRelationService<T>
where
    T: PolicyTemplatesRepo + PolicyRelationsRepo + Send + Sync + 'static,
{
    config: ApplicationProviderConfig,
    repo: Arc<T>,
}

impl<T> PolicyTemplatesToDatahubDatasetRelationService<T>
where
    T: PolicyTemplatesRepo + PolicyRelationsRepo + Send + Sync + 'static,
{
    pub fn new(config: ApplicationProviderConfig, repo: Arc<T>) -> Self {
        Self { config, repo }
    }
}

#[async_trait]
impl<T> PolicyTemplatesToDatahubDatasetRelationTrait for PolicyTemplatesToDatahubDatasetRelationService<T>
where
    T: PolicyTemplatesRepo + PolicyRelationsRepo + Send + Sync + 'static,
{
    async fn add_policy_template_to_dataset(
        &self,
        template_id: Urn,
        dataset_id: String,
    ) -> anyhow::Result<PolicyTemplateDatasetRelation> {
        todo!()
    }

    async fn remove_policy_template_from_dataset(&self, template_dataset_relation: Urn) -> anyhow::Result<()> {
        todo!()
    }

    async fn get_policies_by_dataset(&self, dataset_id: String) -> anyhow::Result<Vec<PolicyTemplateDatasetRelation>> {
        todo!()
    }
}
