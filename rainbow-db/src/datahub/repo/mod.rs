/*
 *
 *  * Copyright (C) 2024 - Universidad Politécnica de Madrid - UPM
 *  *
 *  * This program is free software: you can redistribute it and/or modify
 *  * it under the terms of the GNU General Public License as published by
 *  * the Free Software Foundation, either version 3 of the License, or
 *  * (at your option) any later version.
 *  *
 *  * This program is distributed in the hope that it will be useful,
 *  * but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *  * GNU General Public License for more details.
 *  *
 *  * You should have received a copy of the GNU General Public License
 *  * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 *
 */

use super::entities::{policy_relations, policy_templates, datahub_datasets};
use crate::transfer_provider::repo::{TransferMessagesRepo, TransferProcessRepo};
use anyhow::Error;
use axum::async_trait;
use sea_orm::DatabaseConnection;
use thiserror::Error;
use urn::Urn;


pub mod sql;

pub trait DatahubConnectorRepoFactory: PolicyTemplatesRepo + PolicyRelationsRepo + Send + Sync + 'static {
    fn create_repo(db_connection: DatabaseConnection) -> Self
    where
        Self: Sized;
}

pub struct NewPolicyTemplateModel {
    pub title: Option<String>,
    pub description: Option<String>,
    pub content: serde_json::Value,
}

#[async_trait]
pub trait PolicyTemplatesRepo {
    async fn get_all_policy_templates(&self, limit: Option<u64>, page: Option<u64>) -> anyhow::Result<Vec<policy_templates::Model>, PolicyTemplatesRepoErrors>;
    async fn get_policy_template_by_id(&self, template_id: String) -> anyhow::Result<Option<policy_templates::Model>, PolicyTemplatesRepoErrors>;
    async fn create_policy_template(&self, new_policy_template: NewPolicyTemplateModel) -> anyhow::Result<policy_templates::Model, PolicyTemplatesRepoErrors>;
    async fn delete_policy_template_by_id(&self, template_id: String) -> anyhow::Result<(), PolicyTemplatesRepoErrors>;
}

pub struct NewPolicyRelationModel {
    pub dataset_id: String,
    pub policy_template_id: String,
    pub extra_content: Option<serde_json::Value>,
}

#[async_trait]
pub trait PolicyRelationsRepo {
    // async fn get_all_policy_relations(&self, limit: Option<u64>, page: Option<u64>) -> anyhow::Result<Vec<policy_relations::Model>, PolicyTemplatesRepoErrors>;
    // async fn get_all_policy_relations_by_template_id(&self, template_id: Urn) -> anyhow::Result<policy_relations::Model, PolicyTemplatesRepoErrors>;
    // async fn get_all_templates_by_dataset_id(&self, dataset_id: String) -> anyhow::Result<Vec<policy_templates::Model>, PolicyTemplatesRepoErrors>;
    // async fn get_relation_by_id(&self, policy_relation_id: Urn) -> anyhow::Result<policy_relations::Model, PolicyTemplatesRepoErrors>;
    async fn create_policy_relation(&self, new_policy_relation: NewPolicyRelationModel) -> anyhow::Result<policy_relations::Model, PolicyTemplatesRepoErrors>;
    async fn delete_policy_relation_by_id(&self, template_id: String) -> anyhow::Result<(), PolicyTemplatesRepoErrors>;

}

pub struct NewDataHubDatasetModel {
    pub urn: String,
    pub name: String,
}

#[async_trait]
pub trait DatahubDatasetsRepo {
    async fn create_datahub_dataset(&self, new_dataset: NewDataHubDatasetModel) -> anyhow::Result<datahub_datasets::Model, DatahubDatasetsRepoErrors>;
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

    #[error("PolicyRelation not found")]
    PolicyRelationNotFound,
    #[error("Error fetching policy relation. {0}")]
    ErrorFetchingPolicyRelation(Error),
    #[error("Error creating policy relation. {0}")]
    ErrorCreatingPolicyRelation(Error),
    #[error("Error deleting policy relation. {0}")]
    ErrorDeletingPolicyRelation(Error),
}

#[derive(Error, Debug)]
pub enum DatahubDatasetsRepoErrors {
    #[error("Error creating dataset. {0}")]
    ErrorCreatingDataset(Error),
}
