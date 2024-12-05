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

use crate::transfer_consumer::entities::transfer_callback;
use crate::transfer_consumer::entities::transfer_callback::Model;
use crate::transfer_consumer::repo::{EditTransferCallback, NewTransferCallback, TransferCallbackRepo};
use anyhow::bail;
use axum::async_trait;
use rainbow_common::config::database::get_db_connection;
use sea_orm::{ActiveModelTrait, ActiveValue, ColumnTrait, EntityTrait, QueryFilter, QuerySelect};
use uuid::Uuid;

pub struct TransferCallbackRepoForSql {}

#[async_trait]
impl TransferCallbackRepo for TransferCallbackRepoForSql {
    async fn get_all_transfer_callbacks(&self, limit: Option<u64>, page: Option<u64>) -> anyhow::Result<Vec<Model>> {
        let db_connection = get_db_connection().await;
        let transfer_callbacks = transfer_callback::Entity::find()
            .limit(limit.unwrap_or(100000))
            .offset(page.unwrap_or(0))
            .all(db_connection)
            .await;
        match transfer_callbacks {
            Ok(transfer_callbacks) => Ok(transfer_callbacks),
            Err(_) => bail!("Failed to fetch transfer callbacks"),
        }
    }

    async fn get_transfer_callbacks_by_id(&self, callback_id: Uuid) -> anyhow::Result<Option<Model>> {
        let db_connection = get_db_connection().await;
        let transfer_callback = transfer_callback::Entity::find_by_id(callback_id).one(db_connection).await;
        match transfer_callback {
            Ok(transfer_callback) => Ok(transfer_callback),
            Err(_) => bail!("Failed to fetch transfer callback"),
        }
    }

    async fn get_transfer_callbacks_by_consumer_id(&self, consumer_pid: Uuid) -> anyhow::Result<Option<Model>> {
        let db_connection = get_db_connection().await;
        let transfer_callback = transfer_callback::Entity::find()
            .filter(transfer_callback::Column::ConsumerPid.eq(consumer_pid))
            .one(db_connection)
            .await;
        match transfer_callback {
            Ok(transfer_callback) => Ok(transfer_callback),
            Err(_) => bail!("Failed to fetch transfer callback"),
        }
    }

    async fn put_transfer_callback(&self, callback_id: Uuid, new_transfer_callback: EditTransferCallback) -> anyhow::Result<Model> {
        let db_connection = get_db_connection().await;

        let old_model = transfer_callback::Entity::find_by_id(callback_id).one(db_connection).await;
        let old_model = match old_model {
            Ok(old_model) => match old_model {
                Some(old_model) => old_model,
                None => bail!("Failed to fetch old model"),
            },
            Err(_) => bail!("Failed to fetch old model"),
        };

        let mut old_active_model: transfer_callback::ActiveModel = old_model.into();
        if let Some(provider_pid) = new_transfer_callback.provider_pid {
            old_active_model.provider_pid = ActiveValue::Set(Option::from(provider_pid));
        }
        if let Some(consumer_pid) = new_transfer_callback.consumer_pid {
            old_active_model.consumer_pid = ActiveValue::Set(consumer_pid);
        }
        if let Some(data_plane_id) = new_transfer_callback.data_plane_id {
            old_active_model.data_plane_id = ActiveValue::Set(Option::from(data_plane_id));
        }
        if let Some(data_address) = new_transfer_callback.data_address {
            old_active_model.data_address = ActiveValue::Set(Option::from(data_address));
        }
        old_active_model.updated_at = ActiveValue::Set(Some(chrono::Utc::now().naive_utc()));

        let model = old_active_model.update(db_connection).await;
        match model {
            Ok(model) => Ok(model),
            Err(_) => bail!("Failed to update model"),
        }
    }

    async fn put_transfer_callback_by_consumer(&self, consumer_pid: Uuid, new_transfer_callback: EditTransferCallback) -> anyhow::Result<Model> {
        let db_connection = get_db_connection().await;

        let old_model = transfer_callback::Entity::find()
            .filter(transfer_callback::Column::ConsumerPid.eq(consumer_pid))
            .one(db_connection).await;
        let old_model = match old_model {
            Ok(old_model) => match old_model {
                Some(old_model) => old_model,
                None => bail!("Failed to fetch old model"),
            },
            Err(_) => bail!("Failed to fetch old model"),
        };

        let mut old_active_model: transfer_callback::ActiveModel = old_model.into();
        if let Some(provider_pid) = new_transfer_callback.provider_pid {
            old_active_model.provider_pid = ActiveValue::Set(Option::from(provider_pid));
        }
        if let Some(consumer_pid) = new_transfer_callback.consumer_pid {
            old_active_model.consumer_pid = ActiveValue::Set(consumer_pid);
        }
        if let Some(data_plane_id) = new_transfer_callback.data_plane_id {
            old_active_model.data_plane_id = ActiveValue::Set(Option::from(data_plane_id));
        }
        if let Some(data_address) = new_transfer_callback.data_address {
            old_active_model.data_address = ActiveValue::Set(Option::from(data_address));
        }
        old_active_model.updated_at = ActiveValue::Set(Some(chrono::Utc::now().naive_utc()));

        let model = old_active_model.update(db_connection).await;
        match model {
            Ok(model) => Ok(model),
            Err(_) => bail!("Failed to update model"),
        }
    }

    async fn create_transfer_callback(&self, new_transfer_callback: NewTransferCallback) -> anyhow::Result<Model> {
        let db_connection = get_db_connection().await;
        let model = transfer_callback::ActiveModel {
            id: ActiveValue::Set(Uuid::new_v4()),
            consumer_pid: ActiveValue::Set(Uuid::new_v4()),
            provider_pid: ActiveValue::Set(None),
            created_at: ActiveValue::Set(chrono::Utc::now().naive_utc()),
            updated_at: ActiveValue::Set(None),
            data_plane_id: ActiveValue::Set(None),
            data_address: ActiveValue::Set(new_transfer_callback.data_address),
        };
        let transfer_callback =
            transfer_callback::Entity::insert(model).exec_with_returning(db_connection).await;

        match transfer_callback {
            Ok(transfer_callback) => Ok(transfer_callback),
            Err(_) => bail!("Failed to create model"),
        }
    }

    async fn delete_transfer_callback(&self, callback_id: Uuid) -> anyhow::Result<()> {
        let db_connection = get_db_connection().await;
        let transfer_callback =
            transfer_callback::Entity::delete_by_id(callback_id).exec(db_connection).await;
        match transfer_callback {
            Ok(delete_result) => match delete_result.rows_affected {
                0 => bail!("Not found"),
                _ => Ok(()),
            },
            Err(_) => bail!("Failed to fetch transfer callback"),
        }
    }
}