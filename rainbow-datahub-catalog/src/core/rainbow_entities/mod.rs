use crate::core::rainbow_entities::policy_templates_types::PolicyTemplateDatasetRelation;
// use crate::core::rainbow_entities::policy_templates_types::PolicyTemplateDatasetRelation;
use axum::async_trait;
use urn::Urn;

pub mod policy_templates_types;
pub mod rainbow_entites;

#[mockall::automock]
#[async_trait]
pub trait PolicyTemplatesToDatahubDatasetRelationTrait {
    async fn add_policy_template_to_dataset(&self, template_id: Urn, dataset_id: String) -> anyhow::Result<PolicyTemplateDatasetRelation>;
    async fn remove_policy_template_from_dataset(&self, template_dataset_relation: Urn) -> anyhow::Result<()>;
    async fn get_policies_by_dataset(&self, dataset_id: String) -> anyhow::Result<Vec<PolicyTemplateDatasetRelation>>;
}