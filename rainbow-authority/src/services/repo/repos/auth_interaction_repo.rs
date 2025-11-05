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
use crate::data::entities::auth_interaction::{Column, Entity, Model, NewModel};
use crate::errors::{ErrorLogTrait, Errors};
use crate::services::repo::repos::GenericRepo;
use crate::services::repo::traits::{AuthInteractionRepoTrait, BasicRepoTrait};
use anyhow::bail;
use axum::async_trait;
use sea_orm::{ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter};
use tracing::error;

#[derive(Clone)]
pub struct AuthInteractionRepo {
    inner: GenericRepo<Entity, NewModel>,
}

impl AuthInteractionRepo {
    pub fn new(db_connection: DatabaseConnection) -> Self {
        Self { inner: GenericRepo::new(db_connection) }
    }
}

#[async_trait]
impl BasicRepoTrait<Model, NewModel> for AuthInteractionRepo {
    async fn get_all(&self, limit: Option<u64>, offset: Option<u64>) -> anyhow::Result<Vec<Model>> {
        self.inner.get_all(limit, offset).await
    }

    async fn get_by_id(&self, id: &str) -> anyhow::Result<Model> {
        self.inner.get_by_id(id).await
    }

    async fn create(&self, model: NewModel) -> anyhow::Result<Model> {
        self.inner.create(model).await
    }

    async fn update(&self, model: Model) -> anyhow::Result<Model> {
        self.inner.update(model).await
    }

    async fn delete(&self, id: &str) -> anyhow::Result<()> {
        self.inner.delete(id).await
    }
}

#[async_trait]
impl AuthInteractionRepoTrait for AuthInteractionRepo {
    async fn get_by_reference(&self, reference: &str) -> anyhow::Result<Model> {
        let model = match Entity::find().filter(Column::InteractRef.eq(reference)).one(&self.inner.db_connection).await
        {
            Ok(Some(data)) => data,
            Ok(None) => {
                let error = Errors::missing_resource_new(
                    reference.to_string(),
                    format!("Missing resource with reference: {}", reference),
                );
                error!("{}", error.log());
                bail!(error)
            }
            Err(e) => {
                let error = Errors::database_new(e.to_string());
                error!("{}", error.log());
                bail!(error)
            }
        };
        Ok(model)
    }

    async fn get_by_cont_id(&self, cont_id: &str) -> anyhow::Result<Model> {
        let model = match Entity::find().filter(Column::ContinueId.eq(cont_id)).one(&self.inner.db_connection).await {
            Ok(Some(data)) => data,
            Ok(None) => {
                let error = Errors::missing_resource_new(
                    cont_id.to_string(),
                    format!("Missing resource with cont_id: {}", cont_id),
                );
                error!("{}", error.log());
                bail!(error)
            }
            Err(e) => {
                let error = Errors::database_new(e.to_string());
                error!("{}", error.log());
                bail!(error)
            }
        };
        Ok(model)
    }
}
