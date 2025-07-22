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

use crate::datahub::entities::policy_templates;
use crate::datahub::repo::{
    DatahubConnectorRepoFactory, NewPolicyTemplateModel, PolicyTemplatesRepo, PolicyTemplatesRepoErrors,
};
use axum::async_trait;
use rainbow_common::protocol::datahub_proxy::datahub_proxy_types::DatahubDataset;
use sea_orm::{ActiveValue, DatabaseConnection, EntityTrait, QueryOrder, QuerySelect};
use serde::Serialize;
use serde_json::Value;
use std::fmt::Debug;

pub struct DatahubConnectorRepoForSql {
    db_connection: DatabaseConnection,
}

impl DatahubConnectorRepoForSql {
    pub fn new(db_connection: DatabaseConnection) -> Self {
        Self { db_connection }
    }
}

impl DatahubConnectorRepoFactory for DatahubConnectorRepoForSql {
    fn create_repo(db_connection: DatabaseConnection) -> Self
    where
        Self: Sized,
    {
        Self::new(db_connection)
    }
}

#[derive(Debug, Serialize)]
pub struct PolicyTemplate {
    pub id: String,
    pub content: Value,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Debug, Serialize)]
pub struct PolicyTemplateDatasetRelation {
    pub relation_id: String,
    pub datahub_dataset: DatahubDataset,
    pub policy_template: PolicyTemplate,
}

#[async_trait]
impl PolicyTemplatesRepo for DatahubConnectorRepoForSql {
    async fn get_all_policy_templates(
        &self,
        limit: Option<u64>,
        page: Option<u64>,
    ) -> anyhow::Result<Vec<policy_templates::Model>, PolicyTemplatesRepoErrors> {
        // Configurar la paginación
        let page = page.unwrap_or(1);
        let limit = limit.unwrap_or(10);
        let offset = (page - 1) * limit;

        // Construir la consulta
        match policy_templates::Entity::find()
            .order_by_desc(policy_templates::Column::CreatedAt)
            .limit(limit)
            .offset(offset)
            .all(&self.db_connection)
            .await
        {
            Ok(templates) => Ok(templates),
            Err(err) => Err(PolicyTemplatesRepoErrors::ErrorFetchingPolicyTemplate(
                err.into(),
            )),
        }
    }

    async fn get_policy_template_by_id(
        &self,
        template_id: String,
    ) -> anyhow::Result<Option<policy_templates::Model>, PolicyTemplatesRepoErrors> {
        // Buscar la plantilla por ID
        match policy_templates::Entity::find_by_id(template_id).one(&self.db_connection).await {
            Ok(template) => Ok(template),
            Err(err) => Err(PolicyTemplatesRepoErrors::ErrorFetchingPolicyTemplate(
                err.into(),
            )),
        }
    }

    async fn create_policy_template(
        &self,
        new_policy_template: NewPolicyTemplateModel,
    ) -> anyhow::Result<policy_templates::Model, PolicyTemplatesRepoErrors> {
        let id = format!("template_{}", chrono::Utc::now().timestamp());

        // Crear el ActiveModel
        let model = policy_templates::ActiveModel {
            id: ActiveValue::Set(id),
            title: ActiveValue::Set(new_policy_template.title),
            description: ActiveValue::Set(new_policy_template.description),
            content: ActiveValue::Set(new_policy_template.content),
            operand_options: ActiveValue::Set(new_policy_template.operand_options),
            created_at: ActiveValue::Set(chrono::Utc::now().naive_utc()),
        };

        // Insertar en la base de datos y devolver el resultado
        match policy_templates::Entity::insert(model).exec_with_returning(&self.db_connection).await {
            Ok(template) => Ok(template),
            Err(err) => Err(PolicyTemplatesRepoErrors::ErrorCreatingPolicyTemplate(
                err.into(),
            )),
        }
    }

    async fn delete_policy_template_by_id(&self, template_id: String) -> anyhow::Result<(), PolicyTemplatesRepoErrors> {
        // Intentar eliminar la plantilla
        match policy_templates::Entity::delete_by_id(template_id).exec(&self.db_connection).await {
            Ok(_) => Ok(()),
            Err(err) => Err(PolicyTemplatesRepoErrors::ErrorDeletingPolicyTemplate(
                err.into(),
            )),
        }
    }
}
