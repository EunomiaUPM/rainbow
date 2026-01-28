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
use crate::data::into_active_trait::IntoActiveSet;
use crate::errors::{CommonErrors, ErrorLog};
use anyhow::bail;
use async_trait::async_trait;
use sea_orm::{
    ActiveModelTrait, DatabaseConnection, EntityTrait, IntoActiveModel, PrimaryKeyTrait,
    QuerySelect,
};
use tracing::log::error;

#[async_trait]
pub trait BasicRepoTrait<T, U>
where
    T: EntityTrait + Sync + Send + 'static,
    T::Model: Send
        + Sync
        + Clone
        + IntoActiveModel<T::ActiveModel>
        + IntoActiveSet<T::ActiveModel>
        + 'static,
    T::ActiveModel: ActiveModelTrait<Entity = T> + Send + Sync + 'static,
    U: IntoActiveSet<T::ActiveModel> + Send + Sync + 'static,
    <T as EntityTrait>::PrimaryKey: PrimaryKeyTrait<ValueType = String>,
{
    fn db(&self) -> &DatabaseConnection;

    async fn get_all(
        &self,
        limit: Option<u64>,
        offset: Option<u64>,
    ) -> anyhow::Result<Vec<T::Model>> {
        let models = T::find()
            .limit(limit.unwrap_or(100000))
            .offset(offset.unwrap_or(0))
            .all(self.db())
            .await;
        match models {
            Ok(data) => Ok(data),
            Err(e) => {
                let error = CommonErrors::database_new(&e.to_string());
                error!("{}", error.log());
                bail!(error)
            }
        }
    }

    async fn get_by_id(&self, id: &str) -> anyhow::Result<T::Model> {
        let model = T::find_by_id(id).one(self.db()).await;

        match model {
            Ok(Some(model)) => Ok(model),
            Ok(None) => {
                let error = CommonErrors::missing_resource_new(
                    id,
                    &format!("Missing resource with id: {}", id),
                );
                error!("{}", error.log());
                bail!(error)
            }
            Err(e) => {
                let error = CommonErrors::database_new(&e.to_string());
                error!("{}", error.log());
                bail!(error)
            }
        }
    }

    async fn create(&self, model: U) -> anyhow::Result<T::Model> {
        let active_model: T::ActiveModel = model.to_active();
        let model: T::Model = match active_model.insert(self.db()).await {
            Ok(model) => model,
            Err(e) => {
                let error = CommonErrors::database_new(&e.to_string());
                error!("{}", error.log());
                bail!(error)
            }
        };
        Ok(model)
    }

    async fn update(&self, model: T::Model) -> anyhow::Result<T::Model> {
        let active_model: T::ActiveModel = model.to_active();
        let new_model: T::Model = match active_model.update(self.db()).await {
            Ok(model) => model,
            Err(e) => {
                let error = CommonErrors::database_new(&e.to_string());
                error!("{}", error.log());
                bail!(error)
            }
        };
        Ok(new_model)
    }

    async fn delete(&self, id: &str) -> anyhow::Result<()> {
        let active_model: T::ActiveModel = match T::find_by_id(id).one(self.db()).await {
            Ok(Some(model)) => model.to_active(),
            Ok(None) => {
                let error = CommonErrors::missing_resource_new(
                    id,
                    &format!("Missing resource with id: {}", id),
                );
                error!("{}", error.log());
                bail!(error)
            }
            Err(e) => {
                let error = CommonErrors::database_new(&e.to_string());
                error!("{}", error.log());
                bail!(error)
            }
        };

        match active_model.delete(self.db()).await {
            Ok(_) => {}
            Err(e) => {
                let error = CommonErrors::database_new(&e.to_string());
                error!("{}", error.log());
                bail!(error)
            }
        };

        Ok(())
    }
}
