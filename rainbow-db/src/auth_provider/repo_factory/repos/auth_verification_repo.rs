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
use crate::auth_provider::entities::auth_verification::{Column, Entity, Model, NewModel};
use crate::auth_provider::repo_factory::repos::GenericRepo;
use crate::auth_provider::repo_factory::traits::{AuthVerificationRepoTrait, BasicRepoTrait};
use axum::async_trait;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, QueryFilter};

#[derive(Clone)]
pub struct AuthVerificationProviderRepo {
    inner: GenericRepo<Entity, NewModel>,
}

impl AuthVerificationProviderRepo {
    pub fn new(db_connection: DatabaseConnection) -> Self {
        Self { inner: GenericRepo::new(db_connection) }
    }
}

#[async_trait]
impl BasicRepoTrait<Model, NewModel> for AuthVerificationProviderRepo {
    async fn get_all(&self, limit: Option<u64>, offset: Option<u64>) -> anyhow::Result<Vec<Model>> {
        self.inner.get_all(limit, offset).await
    }

    async fn get_by_id(&self, id: &str) -> anyhow::Result<Option<Model>> {
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
impl AuthVerificationRepoTrait for AuthVerificationProviderRepo {
    async fn get_by_state(&self, state: &str) -> anyhow::Result<Option<Model>> {
        let model = Entity::find().filter(Column::State.eq(state)).one(&self.inner.db_connection).await?;
        Ok(model)
    }

    async fn create_extra(&self, model: Model) -> anyhow::Result<Model> {
        let active_model = model.into_active_model();
        let model = active_model.insert(&self.inner.db_connection).await?;
        Ok(model)
    }
}
