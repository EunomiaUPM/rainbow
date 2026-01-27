// use clap::builder::Str;
// use rainbow_db::datahub::repo::sql::PolicyTemplateDatasetRelation;
// use crate::core::rainbow_entities::PolicyTemplatesToDatahubDatasetRelationTrait;
// use async_trait::async_trait;
// use rainbow_common::config::provider_config::ApplicationProviderConfig;
// use std::sync::Arc;
// use urn::Urn;
// use crate::http::rainbow_entities::policy_templates::PolicyTemplate;
// use crate::core::datahub_proxy::datahub_proxy_types::{DatahubDataset, Platform, DatahubDomain, GlossaryTerm, DomainProperties};
// use sea_orm::{DatabaseConnection, EntityTrait};
// use sea_orm::{ActiveValue, Condition};
// use rainbow_db::datahub::repo::{DatahubConnectorRepoFactory, NewPolicyRelationModel, NewPolicyTemplateModel, PolicyRelationsRepo, PolicyTemplatesRepo, PolicyTemplatesRepoErrors, NewDataHubDatasetModel, DatahubDatasetsRepo, DatahubDatasetsRepoErrors};
// use std::str::FromStr;
// use std::collections::HashMap;

// pub struct PolicyTemplatesToDatahubDatasetRelationService<T>
// where
//     T: PolicyTemplatesRepo + PolicyRelationsRepo + Send + Sync + 'static,
// {
//     config: ApplicationProviderConfig,
//     repo: Arc<T>,
// }

// impl<T> PolicyTemplatesToDatahubDatasetRelationService<T>
// where
//     T: PolicyTemplatesRepo + PolicyRelationsRepo + Send + Sync + 'static,
// {
//     pub fn new(config: ApplicationProviderConfig, repo: Arc<T>) -> Self {
//         Self { config, repo }
//     }
// }

// #[async_trait]
// impl<T> PolicyTemplatesToDatahubDatasetRelationTrait for PolicyTemplatesToDatahubDatasetRelationService<T>
// where
//     T: PolicyTemplatesRepo + PolicyRelationsRepo + Send + Sync + 'static,
// {
//     // async fn add_policy_template_to_dataset(
//     //     &self,
//     //     template_id: String,
//     //     dataset_id: String,
//     // ) -> anyhow::Result<PolicyTemplateDatasetRelation> {
//     //     // 1. Verificar que el template existe
//     //             // todo!()

//     //     let template = self.repo.get_policy_template_by_id(template_id.clone())
//     //         .await
//     //         .map_err(|e| anyhow::anyhow!("Error fetching policy template: {}", e))?
//     //         .ok_or_else(|| anyhow::anyhow!("Policy template not found"))?;

//     //     // 2. Crear la relación
//     //     let new_relation = NewPolicyRelationModel {
//     //         dataset_id: dataset_id.clone(),
//     //         policy_template_id: template_id.clone(),
//     //         extra_content: None,
//     //     };

//     //     let relation = self.repo.create_policy_relation(new_relation)
//     //         .await
//     //         .map_err(|e| anyhow::anyhow!("Error creating policy relation: {}", e))?;

//     //     // 3. Construir la respuesta
//     //     let policy_template = PolicyTemplate {
//     //         id: template.id,
//     //         content: template.content,
//     //         created_at: template.created_at,
//     //     };

//     //     // 4. Obtener los datos del dataset (esto debería venir de tu servicio de DataHub)
//     //     let dataset = DatahubDataset {
//     //         urn: dataset_id,
//     //         name: "Dataset Name".to_string(), // Esto debería venir de tu servicio de DataHub
//     //         platform: Platform::default(), // Esto debería venir de tu servicio de DataHub
//     //         description: None,
//     //         tag_names: vec![],
//     //         custom_properties: vec![],
//     //         domain: DatahubDomain {
//     //             urn: "Default".to_string(),
//     //             properties: DomainProperties {
//     //                 name: "Default Domain".to_string(),
//     //                 description: None,
//     //             },
//     //         },
//     //         glossary_terms: None,
//     //     };

//     //     Ok(PolicyTemplateDatasetRelation {
//     //         relation_id: relation.id,
//     //         datahub_dataset: dataset,
//     //         policy_template,
//     //     })
//     // }

//     // async fn remove_policy_template_from_dataset(&self, template_dataset_relation: Urn) -> anyhow::Result<()> {
//     //     todo!()
//     // }

//     // async fn get_policies_by_dataset(&self, dataset_id: String) -> anyhow::Result<Vec<PolicyTemplateDatasetRelation>> {
//     //     todo!()
//     // }


// }
