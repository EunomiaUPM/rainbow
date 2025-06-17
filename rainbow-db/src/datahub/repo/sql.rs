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
use crate::datahub::entities::{policy_relations, policy_templates};
use crate::datahub::repo::{
    DatahubConnectorRepoFactory, NewPolicyRelationModel, NewPolicyTemplateModel, PolicyRelationsRepo,
    PolicyTemplatesRepo, PolicyTemplatesRepoErrors,
};
use axum::async_trait;
use rainbow_common::protocol::datahub_proxy::datahub_proxy_types::DatahubDataset;
use sea_orm::{ActiveValue, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QueryOrder, QuerySelect};
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

    async fn get_all_templates_by_dataset_id(
        &self,
        dataset_id: String,
    ) -> anyhow::Result<Vec<policy_templates::Model>, PolicyTemplatesRepoErrors> {
        // 1. Primero obtenemos todas las relaciones para el dataset_id
        let relations = policy_relations::Entity::find()
            .filter(policy_relations::Column::DatasetId.eq(dataset_id))
            .all(&self.db_connection)
            .await
            .map_err(|err| PolicyTemplatesRepoErrors::ErrorFetchingPolicyRelation(err.into()))?;

        // 2. Extraemos los IDs de los templates
        let template_ids: Vec<String> = relations.into_iter().map(|relation| relation.policy_template_id).collect();

        // 3. Obtenemos los templates correspondientes
        let templates = policy_templates::Entity::find()
            .filter(policy_templates::Column::Id.is_in(template_ids))
            .all(&self.db_connection)
            .await
            .map_err(|err| PolicyTemplatesRepoErrors::ErrorFetchingPolicyTemplate(err.into()))?;

        Ok(templates)
    }
}

#[async_trait]
impl PolicyRelationsRepo for DatahubConnectorRepoForSql {
    async fn create_policy_relation(
        &self,
        new_relation: NewPolicyRelationModel,
    ) -> anyhow::Result<policy_relations::Model, PolicyTemplatesRepoErrors> {
        let id = format!("relation_{}", chrono::Utc::now().timestamp());
        let policy_template_id = self.get_policy_template_by_id(new_relation.policy_template_id.clone()).await?;
        let policy_template_id = match policy_template_id {
            Some(policy_template_id) => policy_template_id,
            None => return Err(PolicyTemplatesRepoErrors::PolicyTemplateNotFound),
        };
        let model = policy_relations::ActiveModel {
            id: ActiveValue::Set(id.clone()),
            dataset_id: ActiveValue::Set(new_relation.dataset_id.clone()),
            policy_template_id: ActiveValue::Set(new_relation.policy_template_id.clone()),
            odrl_offer: ActiveValue::Set(new_relation.odrl_offer),
            created_at: ActiveValue::Set(chrono::Utc::now().naive_utc()),
        };

        match policy_relations::Entity::insert(model).exec_with_returning(&self.db_connection).await {
            Ok(relation) => Ok(relation),
            Err(err) => Err(PolicyTemplatesRepoErrors::ErrorCreatingPolicyRelation(
                err.into(),
            )),
        }
    }

    async fn delete_policy_relation_by_id(&self, relation_id: String) -> anyhow::Result<(), PolicyTemplatesRepoErrors> {
        match policy_relations::Entity::delete_by_id(relation_id).exec(&self.db_connection).await {
            Ok(_) => Ok(()),
            Err(err) => Err(PolicyTemplatesRepoErrors::ErrorDeletingPolicyRelation(
                err.into(),
            )),
        }
    }

    async fn get_all_policy_relations(
        &self,
        limit: Option<u64>,
        page: Option<u64>,
    ) -> anyhow::Result<Vec<policy_relations::Model>, PolicyTemplatesRepoErrors> {
        // Configurar la paginación
        let page = page.unwrap_or(1);
        let limit = limit.unwrap_or(10);
        let offset = (page - 1) * limit;

        // Construir la consulta
        match policy_relations::Entity::find()
            .order_by_desc(policy_relations::Column::CreatedAt)
            .limit(limit)
            .offset(offset)
            .all(&self.db_connection)
            .await
        {
            Ok(relations) => Ok(relations),
            Err(err) => Err(PolicyTemplatesRepoErrors::ErrorFetchingPolicyRelation(
                err.into(),
            )),
        }
    }

    async fn get_relation_by_id(
        &self,
        relation_id: String,
    ) -> anyhow::Result<Option<policy_relations::Model>, PolicyTemplatesRepoErrors> {
        // Buscar la plantilla por ID
        match policy_relations::Entity::find_by_id(relation_id).one(&self.db_connection).await {
            Ok(relation) => Ok(relation),
            Err(err) => Err(PolicyTemplatesRepoErrors::ErrorFetchingPolicyRelation(
                err.into(),
            )),
        }
    }

    async fn get_all_policy_relations_by_template_id(
        &self,
        template_id: String,
    ) -> anyhow::Result<Vec<policy_relations::Model>, PolicyTemplatesRepoErrors> {
        // Construir la consulta para encontrar todas las relaciones con el template_id específico
        match policy_relations::Entity::find()
            .filter(policy_relations::Column::PolicyTemplateId.eq(template_id))
            .order_by_desc(policy_relations::Column::CreatedAt)
            .all(&self.db_connection)
            .await
        {
            Ok(relations) => Ok(relations),
            Err(err) => Err(PolicyTemplatesRepoErrors::ErrorFetchingPolicyRelation(
                err.into(),
            )),
        }
    }
}
