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
use crate::datahub::repo::{DatahubConnectorRepoFactory, NewPolicyRelationModel, NewPolicyTemplateModel, PolicyRelationsRepo, PolicyTemplatesRepo, PolicyTemplatesRepoErrors};
use axum::async_trait;
use sea_orm::{DatabaseConnection, EntityTrait};
use sea_orm::QueryFilter;
use urn::Urn;
use sea_orm::{ActiveValue, Condition};

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

#[async_trait]
impl PolicyTemplatesRepo for DatahubConnectorRepoForSql {
    async fn get_all_policy_templates(&self, limit: Option<u64>, page: Option<u64>) -> anyhow::Result<Vec<policy_templates::Model>, PolicyTemplatesRepoErrors> {
        todo!()
    }

    async fn get_policy_template_by_id(&self, template_id: Urn) -> anyhow::Result<Option<policy_templates::Model>, PolicyTemplatesRepoErrors> {
        todo!()
    }

    async fn create_policy_template(&self, new_policy_template: NewPolicyTemplateModel) -> anyhow::Result<policy_templates::Model, PolicyTemplatesRepoErrors> {
        let id = format!("template_{}", chrono::Utc::now().timestamp());

        // Crear el ActiveModel
        let model = policy_templates::ActiveModel {
            id: ActiveValue::Set(id),
            title: ActiveValue::Set(new_policy_template.title),
            description: ActiveValue::Set(new_policy_template.description),
            content: ActiveValue::Set(new_policy_template.content),
            created_at: ActiveValue::Set(chrono::Utc::now().naive_utc()),
        };

        // Insertar en la base de datos y devolver el resultado
        match policy_templates::Entity::insert(model)
            .exec_with_returning(&self.db_connection)
            .await
        {
            Ok(template) => Ok(template),
            Err(err) => Err(PolicyTemplatesRepoErrors::ErrorCreatingPolicyTemplate(err.into())),
        }
    }

    async fn delete_policy_template_by_id(&self, template_id: String) -> anyhow::Result<(), PolicyTemplatesRepoErrors> {
    // Intentar eliminar la plantilla
        match policy_templates::Entity::delete_by_id(template_id)
            .exec(&self.db_connection)
            .await
        {
            Ok(_) => Ok(()),
            Err(err) => Err(PolicyTemplatesRepoErrors::ErrorDeletingPolicyTemplate(err.into())),
        }
    }
}

#[async_trait]
impl PolicyRelationsRepo for DatahubConnectorRepoForSql {
    async fn get_all_policy_relations(&self, limit: Option<u64>, page: Option<u64>) -> anyhow::Result<Vec<policy_relations::Model>, PolicyTemplatesRepoErrors> {
        todo!()
        // let policy_relations = policy_relations::Entity::find()
        //     .all(&self.db_connection)
        //     .await;
        // match policy_relations {
        //     Ok(policy_relations) => Ok(policy_relations),
        //     Err(e) => Err(PolicyTemplatesRepoErrors::ErrorFetchingPolicyTemplate(e.into()))
        // }
    }

    async fn get_all_policy_relations_by_template_id(&self, template_id: Urn) -> anyhow::Result<policy_relations::Model, PolicyTemplatesRepoErrors> {
        todo!()
        // let template_id = template_id.to_string();
        // let policy_relation = policy_relations::Entity::find_by_id(template_id)
        //     .one(&self.db_connection)
        //     .await
        //     .map_err(|e| Err(PolicyTemplatesRepoErrors::ErrorFetchingPolicyTemplate(e.into())))?;
        // match policy_relation {
        //     Some(policy_relation) => Ok(policy_relation),
        //     None => Err(PolicyTemplatesRepoErrors::PolicyTemplateNotFound)
        // }
    }

    async fn get_all_templates_by_dataset_id(&self, dataset_id: String) -> anyhow::Result<Vec<policy_templates::Model>, PolicyTemplatesRepoErrors> {
        todo!()
    }

    async fn get_relation_by_id(&self, policy_relation_id: Urn) -> anyhow::Result<policy_relations::Model, PolicyTemplatesRepoErrors> {
        todo!()
    }

    async fn create_policy_relation(&self, new_policy_relation: NewPolicyRelationModel) -> anyhow::Result<policy_relations::Model, PolicyTemplatesRepoErrors> {
        todo!()
    }

    async fn delete_policy_relation(&self, template_id: Urn) -> anyhow::Result<(), PolicyTemplatesRepoErrors> {
        todo!()
    }
}



