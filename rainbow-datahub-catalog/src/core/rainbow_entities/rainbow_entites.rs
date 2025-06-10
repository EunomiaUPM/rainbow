use crate::core::rainbow_entities::policy_templates_types::PolicyTemplateDatasetRelation;
use crate::core::rainbow_entities::PolicyTemplatesToDatahubDatasetRelationTrait;
use axum::async_trait;
use rainbow_common::config::provider_config::ApplicationProviderConfig;
use std::sync::Arc;
use urn::Urn;
use rainbow_common::protocol::policy_templates::policy_templates::PolicyTemplate;
use crate::core::datahub_proxy::datahub_proxy_types::{DatahubDataset, Platform, DatahubDomain, GlossaryTerm, DomainProperties};
use sea_orm::{DatabaseConnection, EntityTrait};
use sea_orm::{ActiveValue, Condition};
use rainbow_db::datahub::repo::{DatahubConnectorRepoFactory, NewPolicyRelationModel, NewPolicyTemplateModel, PolicyRelationsRepo, PolicyTemplatesRepo, PolicyTemplatesRepoErrors, NewDataHubDatasetModel, DatahubDatasetsRepo, DatahubDatasetsRepoErrors};
use std::str::FromStr;
use std::collections::HashMap;

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

    async fn create_policy_relation(&self, new_relation: NewPolicyRelationModel) -> anyhow::Result<PolicyTemplateDatasetRelation> {
        // Primero creamos la relación en la base de datos
        let relation = self.repo.create_policy_relation(new_relation).await?;
        
        // Obtenemos el template
        let template = self.repo.get_policy_template_by_id(relation.policy_template_id.clone())
            .await?
            .ok_or_else(|| anyhow::anyhow!("Policy template not found"))?;
        
        // Convertimos el template al formato esperado
        let policy_template = PolicyTemplate {
            id: Urn::from_str(&format!("urn:li:policyTemplate:{}", template.id))?,
            content: template.content,
            created_at: template.created_at,
        };

        // Creamos un dataset básico con los campos requeridos
        let datahub_dataset = DatahubDataset {
            urn: relation.dataset_id,
            name: "Dataset".to_string(),
            platform: Platform { name: "unknown".to_string() },
            description: None,
            tag_names: vec![],
            custom_properties: vec![],
            domain: DatahubDomain {
                urn: "urn:li:domain:unknown".to_string(),
                properties: DomainProperties {
                    name: "Unknown".to_string(),
                    description: None,
                },
            },
            glossary_terms: None,
        };
        
        Ok(PolicyTemplateDatasetRelation {
            relation_id: relation.id,
            datahub_dataset,
            policy_template,
        })
    }
}
