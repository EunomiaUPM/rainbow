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

use crate::dataplane::entities::data_plane_field;
use crate::dataplane::entities::data_plane_process;
use crate::dataplane::entities::data_plane_process::Model;
use crate::dataplane::repo::{
    DataPlaneFieldRepo, DataPlaneProcessRepo, EditDataPlaneField, EditDataPlaneProcess,
    NewDataPlaneField, NewDataPlaneProcess,
};
use anyhow::bail;
use axum::async_trait;
use rainbow_common::config::database::get_db_connection;
use rainbow_common::utils::get_urn;
use sea_orm::{ActiveModelTrait, ActiveValue, ColumnTrait, EntityTrait, QueryFilter, QuerySelect};
use urn::Urn;

pub struct DataPlaneRepoForSql {}

#[async_trait]
impl DataPlaneProcessRepo for DataPlaneRepoForSql {
    async fn get_all_data_plane_processes(
        &self,
        limit: Option<u64>,
        offset: Option<u64>,
    ) -> anyhow::Result<Vec<data_plane_process::Model>> {
        let db_connection = get_db_connection().await;
        let data_plane_processes = data_plane_process::Entity::find()
            .limit(limit.unwrap_or(100000))
            .offset(offset.unwrap_or(0))
            .all(db_connection)
            .await;
        match data_plane_processes {
            Ok(data_plane_processes) => Ok(data_plane_processes),
            Err(_) => bail!("Failed to fetch data plane processes"),
        }
    }

    async fn get_data_plane_process_by_id(
        &self,
        data_plane_process_id: Urn,
    ) -> anyhow::Result<Option<data_plane_process::Model>> {
        let db_connection = get_db_connection().await;
        let data_plane_process_id = data_plane_process_id.to_string();
        let data_plane_process =
            data_plane_process::Entity::find_by_id(data_plane_process_id).one(db_connection).await;
        match data_plane_process {
            Ok(data_plane_process) => Ok(data_plane_process),
            Err(_) => bail!("Failed to fetch data plane process"),
        }
    }

    async fn get_data_plane_process_by_id_in_url(&self, id: Urn) -> anyhow::Result<Option<Model>> {
        let db_connection = get_db_connection().await;
        let id = id.to_string();
        let data_plane_process = data_plane_process::Entity::find()
            .filter(data_plane_process::Column::Address.contains(id))
            .one(db_connection)
            .await;

        match data_plane_process {
            Ok(data_plane_process) => Ok(data_plane_process),
            Err(_) => bail!("Failed to fetch data plane process"),
        }
    }

    async fn put_data_plane_process(
        &self,
        data_plane_process_id: Urn,
        new_data_plane_process: EditDataPlaneProcess,
    ) -> anyhow::Result<data_plane_process::Model> {
        let db_connection = get_db_connection().await;
        let data_plane_process_id = data_plane_process_id.to_string();

        let old_model =
            data_plane_process::Entity::find_by_id(data_plane_process_id).one(db_connection).await;
        let old_model = match old_model {
            Ok(old_model) => match old_model {
                Some(old_model) => old_model,
                None => bail!("Failed to fetch old model"),
            },
            Err(_) => bail!("Failed to fetch old model"),
        };

        let mut old_active_model: data_plane_process::ActiveModel = old_model.into();
        if let Some(address) = new_data_plane_process.address {
            old_active_model.address = ActiveValue::Set(address);
        }
        old_active_model.updated_at = ActiveValue::Set(Some(chrono::Utc::now().naive_utc()));

        let model = old_active_model.update(db_connection).await;
        match model {
            Ok(model) => Ok(model),
            Err(_) => bail!("Failed to update model"),
        }
    }

    async fn create_data_plane_process(
        &self,
        new_data_plane_process: NewDataPlaneProcess,
    ) -> anyhow::Result<data_plane_process::Model> {
        let db_connection = get_db_connection().await;
        let model = data_plane_process::ActiveModel {
            id: ActiveValue::Set(new_data_plane_process.id.to_string()),
            role: ActiveValue::Set(new_data_plane_process.role),
            address: ActiveValue::Set(new_data_plane_process.address),
            dct_action_format: ActiveValue::Set(new_data_plane_process.dct_action_format),
            dct_action_protocol: ActiveValue::Set(new_data_plane_process.dct_action_protocol),
            created_at: ActiveValue::Set(chrono::Utc::now().naive_utc()),
            updated_at: ActiveValue::Set(None),
        };
        let data_plane_process =
            data_plane_process::Entity::insert(model).exec_with_returning(db_connection).await;

        match data_plane_process {
            Ok(data_plane_process) => Ok(data_plane_process),
            Err(_) => bail!("Failed to create model"),
        }
    }

    async fn delete_data_plane_process(&self, data_plane_process_id: Urn) -> anyhow::Result<()> {
        let db_connection = get_db_connection().await;
        let data_plane_process_id = data_plane_process_id.to_string();
        let data_plane_p = data_plane_process::Entity::delete_by_id(data_plane_process_id)
            .exec(db_connection)
            .await;
        match data_plane_p {
            Ok(delete_result) => match delete_result.rows_affected {
                0 => bail!("Not found"),
                _ => Ok(()),
            },
            Err(_) => bail!("Failed to fetch transfer callback"),
        }
    }
}

#[async_trait]
impl DataPlaneFieldRepo for DataPlaneRepoForSql {
    async fn get_all_data_plane_fields(
        &self,
        limit: Option<u64>,
        offset: Option<u64>,
    ) -> anyhow::Result<Vec<data_plane_field::Model>> {
        let db_connection = get_db_connection().await;
        let data_plane_fields = data_plane_field::Entity::find()
            .limit(limit.unwrap_or(100000))
            .offset(offset.unwrap_or(0))
            .all(db_connection)
            .await;
        match data_plane_fields {
            Ok(data_plane_fields) => Ok(data_plane_fields),
            Err(_) => bail!("Failed to fetch data plane fields"),
        }
    }

    async fn get_all_data_plane_fields_by_process(
        &self,
        data_plane_process_id: Urn,
    ) -> anyhow::Result<Vec<data_plane_field::Model>> {
        let db_connection = get_db_connection().await;
        let data_plane_process_id = data_plane_process_id.to_string();
        let data_plane_process = match data_plane_process::Entity::find_by_id(data_plane_process_id)
            .one(db_connection)
            .await
        {
            Ok(data_plane_process) => match data_plane_process {
                Some(data_plane_process) => data_plane_process,
                None => bail!("Data plane process not found"),
            },
            Err(_) => bail!("Failed to fetch data plane fields"),
        };
        let data_plane_fields = data_plane_field::Entity::find()
            .filter(data_plane_field::Column::DataPlaneProcessId.eq(data_plane_process.id))
            .all(db_connection)
            .await;

        match data_plane_fields {
            Ok(data_plane_fields) => Ok(data_plane_fields),
            Err(_) => bail!("Failed to fetch data plane fields"),
        }
    }

    async fn get_data_plane_field_by_id(
        &self,
        data_plane_field_id: Urn,
    ) -> anyhow::Result<Option<data_plane_field::Model>> {
        let db_connection = get_db_connection().await;
        let data_plane_field_id = data_plane_field_id.to_string();
        let data_plane_f =
            data_plane_field::Entity::find_by_id(data_plane_field_id).one(db_connection).await;
        match data_plane_f {
            Ok(data_plane_f) => Ok(data_plane_f),
            Err(_) => bail!("Failed to fetch data plane field"),
        }
    }

    async fn put_data_plane_field_by_id(
        &self,
        data_plane_field_id: Urn,
        new_data_plane_field: EditDataPlaneField,
    ) -> anyhow::Result<data_plane_field::Model> {
        let db_connection = get_db_connection().await;
        let data_plane_field_id = data_plane_field_id.to_string();

        let old_model =
            data_plane_field::Entity::find_by_id(data_plane_field_id).one(db_connection).await;
        let old_model = match old_model {
            Ok(old_model) => match old_model {
                Some(old_model) => old_model,
                None => bail!("Failed to fetch old model"),
            },
            Err(_) => bail!("Failed to fetch old model"),
        };

        let mut old_active_model: data_plane_field::ActiveModel = old_model.into();
        if let Some(value) = new_data_plane_field.value {
            old_active_model.value = ActiveValue::Set(value);
        }

        let model = old_active_model.update(db_connection).await;
        match model {
            Ok(model) => Ok(model),
            Err(_) => bail!("Failed to update model"),
        }
    }

    async fn create_data_plane_field(
        &self,
        new_data_plane_field: NewDataPlaneField,
    ) -> anyhow::Result<data_plane_field::Model> {
        let db_connection = get_db_connection().await;
        let model = data_plane_field::ActiveModel {
            id: ActiveValue::Set(get_urn(None).to_string()),
            key: ActiveValue::Set(new_data_plane_field.key),
            value: ActiveValue::Set(new_data_plane_field.value),
            data_plane_process_id: ActiveValue::Set(
                new_data_plane_field.data_plane_process_id.to_string(),
            ),
        };
        let transfer_callback =
            data_plane_field::Entity::insert(model).exec_with_returning(db_connection).await;

        match transfer_callback {
            Ok(transfer_callback) => Ok(transfer_callback),
            Err(_) => bail!("Failed to create model"),
        }
    }

    async fn delete_data_plane_field(&self, data_plane_field_id: Urn) -> anyhow::Result<()> {
        let db_connection = get_db_connection().await;
        let data_plane_field_id = data_plane_field_id.to_string();
        let transfer_callback =
            data_plane_field::Entity::delete_by_id(data_plane_field_id).exec(db_connection).await;
        match transfer_callback {
            Ok(delete_result) => match delete_result.rows_affected {
                0 => bail!("Not found"),
                _ => Ok(()),
            },
            Err(_) => bail!("Failed to fetch data plane field"),
        }
    }
}
