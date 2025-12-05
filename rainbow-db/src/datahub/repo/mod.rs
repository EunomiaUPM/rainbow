/*
 *
 *  * Copyright (C) 2025 - Universidad Politécnica de Madrid - UPM
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

use super::entities::policy_templates;
use anyhow::Error;
use axum::async_trait;
use rainbow_common::policy_templates::CreatePolicyTemplateRequest;
use sea_orm::DatabaseConnection;
use serde_json::to_value;
use thiserror::Error;

pub mod sql;

pub trait DatahubConnectorRepoFactory: PolicyTemplatesRepo + Send + Sync + 'static {
    fn create_repo(db_connection: DatabaseConnection) -> Self
    where
        Self: Sized;
}

pub struct NewPolicyTemplateModel {
    pub title: Option<String>,
    pub description: Option<String>,
    pub content: serde_json::Value,
    pub operand_options: Option<serde_json::Value>,
}

impl From<CreatePolicyTemplateRequest> for NewPolicyTemplateModel {
    fn from(value: CreatePolicyTemplateRequest) -> Self {
        Self {
            title: Some(value.title),
            description: Some(value.description),
            content: to_value(value.content).unwrap(),
            operand_options: Some(to_value(value.template_operands).unwrap()),
        }
    }
}

#[async_trait]
pub trait PolicyTemplatesRepo {
    async fn get_all_policy_templates(
        &self,
        limit: Option<u64>,
        page: Option<u64>,
    ) -> anyhow::Result<Vec<policy_templates::Model>, PolicyTemplatesRepoErrors>;
    async fn get_policy_template_by_id(
        &self,
        template_id: String,
    ) -> anyhow::Result<Option<policy_templates::Model>, PolicyTemplatesRepoErrors>;
    async fn create_policy_template(
        &self,
        new_policy_template: NewPolicyTemplateModel,
    ) -> anyhow::Result<policy_templates::Model, PolicyTemplatesRepoErrors>;
    async fn delete_policy_template_by_id(&self, template_id: String) -> anyhow::Result<(), PolicyTemplatesRepoErrors>;
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
