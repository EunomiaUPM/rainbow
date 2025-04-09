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

use crate::dataplane::entities::data_plane_process;
use crate::ssi_auth_provider::entities::ssi_auth_data;
use crate::ssi_auth_provider::entities::ssi_auth_data::Status;
use crate::ssi_auth_provider::repo::AuthDataRepoTrait;
use anyhow::bail;
use axum::async_trait;
use rainbow_common::config::database::get_db_connection;
use rand::{distributions::Alphanumeric, Rng};
use sea_orm::{ActiveModelTrait, ActiveValue, EntityTrait, IntoActiveModel, QuerySelect};

use chrono;

pub struct AuthDataRepo;

#[async_trait]
impl AuthDataRepoTrait for AuthDataRepo {
    async fn get_all_ssi_auth_data(
        &self,
        limit: Option<u64>,
        offset: Option<u64>,
    ) -> anyhow::Result<Vec<ssi_auth_data::Model>> {
        let db_connection = get_db_connection().await;
        let auths = ssi_auth_data::Entity::find()
            .limit(limit.unwrap_or_else(|| 100000))
            .offset(offset.unwrap_or_else(|| 0))
            .all(db_connection)
            .await;
        match auths {
            Ok(auths) => Ok(auths),
            Err(e) => bail!("Failed to fetch ssi auth data: {}", e),
        }
    }

    async fn get_ssi_auth_data_by_id(
        &self,
        id: i64,
    ) -> anyhow::Result<Option<ssi_auth_data::Model>> {
        let db_connection = get_db_connection().await;
        let ssi_auth = ssi_auth_data::Entity::find_by_id(id).one(db_connection).await;
        match ssi_auth {
            Ok(ssi_auth) => Ok(ssi_auth),
            Err(e) => bail!("Failed to fetch ssi auth data: {}", e),
        }
    }

    async fn create_ssi_auth_data(&self) -> anyhow::Result<ssi_auth_data::Model> {
        let db_connection = get_db_connection().await;

        let state: String =
            rand::thread_rng().sample_iter(&Alphanumeric).take(12).map(char::from).collect();

        let nonce: String =
            rand::thread_rng().sample_iter(&Alphanumeric).take(36).map(char::from).collect();

        let model = ssi_auth_data::ActiveModel {
            nonce: ActiveValue::Set(nonce),
            status: ActiveValue::Set(Status::Ongoing),
            state: ActiveValue::Set(state),
            created_at: ActiveValue::Set(chrono::Utc::now().naive_utc()),
            ended_at: ActiveValue::Set(None),
            ..Default::default()
        };

        let ssi_auth =
            ssi_auth_data::Entity::insert(model).exec_with_returning(db_connection).await;

        match ssi_auth {
            Ok(ssi_auth) => Ok(ssi_auth),
            Err(e) => bail!("Failed to create model: {}", e),
        }
    }

    async fn update_status_ssi_auth_data(
        &self,
        id: i64,
        status: Status,
    ) -> anyhow::Result<ssi_auth_data::Model> {
        let db_connection = get_db_connection().await;

        let mut entry = match ssi_auth_data::Entity::find_by_id(id).one(db_connection).await {
            Ok(Some(entry)) => entry.into_active_model(),
            Ok(None) => bail!("No entry found with ID: {}", id),
            Err(e) => bail!("Failed to fetch ssi auth data: {}", e),
        };

        entry.status = ActiveValue::Set(status);
        entry.ended_at = ActiveValue::Set(Some(chrono::Utc::now().naive_utc()));

        let upd_entry = entry.update(db_connection).await;

        match upd_entry {
            Ok(upd_entry) => Ok(upd_entry),
            Err(e) => bail!("Failed to update status: {}", e),
        }
    }

    async fn delete_ssi_auth_data(&self, id: i64) -> anyhow::Result<ssi_auth_data::Model> {
        let db_connection = get_db_connection().await;

        let mut entry = match ssi_auth_data::Entity::find_by_id(id).one(db_connection).await {
            Ok(Some(entry)) => entry,
            Ok(None) => bail!("No entry found with ID: {}", id),
            Err(e) => bail!("Failed to fetch ssi auth data: {}", e),
        };
        let ret = entry.clone();
        let active_model = entry.into_active_model();
        active_model.delete(db_connection).await?;

        Ok(ret)
    }
}
