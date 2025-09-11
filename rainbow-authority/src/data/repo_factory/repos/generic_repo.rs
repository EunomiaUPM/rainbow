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

use super::super::traits::{BasicRepoTrait, IntoActiveSet};
use anyhow::bail;
use axum::async_trait;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, IntoActiveModel, QuerySelect};

#[derive(Clone)]
pub struct GenericRepo<T, U>
where
    T: EntityTrait,
{
    pub db_connection: DatabaseConnection,
    _marker: std::marker::PhantomData<(T, U)>,
}

impl<T, U> GenericRepo<T, U>
where
    T: EntityTrait,
{
    pub fn new(db_connection: DatabaseConnection) -> Self {
        Self { db_connection, _marker: std::marker::PhantomData }
    }
}

#[async_trait]
impl<T, U> BasicRepoTrait<T::Model, U> for GenericRepo<T, U>
where
    T: EntityTrait + Sync + Send,
    T::Model: Send + Sync + Clone + IntoActiveModel<T::ActiveModel> + IntoActiveSet<T::ActiveModel>,
    T::ActiveModel: ActiveModelTrait<Entity = T> + Send + Sync,
    U: IntoActiveSet<T::ActiveModel> + Send + Sync,
    <T as EntityTrait>::PrimaryKey: sea_orm::PrimaryKeyTrait<ValueType = String>,
{
    async fn get_all(&self, limit: Option<u64>, offset: Option<u64>) -> anyhow::Result<Vec<T::Model>> {
        let models =
            T::find().limit(limit.unwrap_or(100000)).offset(offset.unwrap_or(0)).all(&self.db_connection).await;
        match models {
            Ok(auths) => Ok(auths),
            Err(e) => bail!("Failed to fetch data: {}", e),
        }
    }

    async fn get_by_id(&self, id: &str) -> anyhow::Result<Option<T::Model>> {
        let model = T::find_by_id(id).one(&self.db_connection).await;

        match model {
            Ok(Some(model)) => Ok(Some(model)),
            Ok(None) => Ok(None),
            Err(e) => bail!("Failed to fetch data: {}", e),
        }
    }

    async fn create(&self, model: U) -> anyhow::Result<T::Model> {
        let active_model: T::ActiveModel = model.to_active();
        let model: T::Model = active_model.insert(&self.db_connection).await?;
        Ok(model)
    }

    async fn update(&self, model: T::Model) -> anyhow::Result<T::Model> {
        let mut active_model: T::ActiveModel = model.to_active();
        let new_model: T::Model = active_model.update(&self.db_connection).await?;
        Ok(new_model)
    }

    async fn delete(&self, id: &str) -> anyhow::Result<()> {
        let mut active_model: T::ActiveModel = match T::find_by_id(id).one(&self.db_connection).await {
            Ok(Some(model)) => model.into_active_model(),
            Ok(None) => bail!("No entry found with ID: {}", id),
            Err(e) => bail!("Failed to fetch data: {}", e),
        };

        active_model.delete(&self.db_connection).await?;

        Ok(())
    }
}
