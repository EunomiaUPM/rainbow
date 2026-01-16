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

use crate::ssi::common::data::entities::recv_verification::{Column, Entity, Model, NewModel};
use anyhow::bail;
use axum::async_trait;
use rainbow_common::data::{BasicRepoTrait, IntoActiveSet};
use rainbow_common::errors::{CommonErrors, ErrorLog};
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use tracing::error;
use crate::ssi::common::services::repo::subtraits::recv_verification_trait::RecvVerificationTrait;

#[derive(Clone)]
pub struct RecvVerificationRepo {
    db_connection: DatabaseConnection,
}

impl RecvVerificationRepo {
    pub fn new(db_connection: DatabaseConnection) -> Self {
        Self { db_connection }
    }
}

impl BasicRepoTrait<Entity, NewModel> for RecvVerificationRepo {
    fn db(&self) -> &DatabaseConnection {
        &self.db_connection
    }
}

#[async_trait]
impl RecvVerificationTrait for RecvVerificationRepo {
    async fn get_by_state(&self, state: &str) -> anyhow::Result<Model> {
        match Entity::find().filter(Column::State.eq(state)).one(self.db()).await {
            Ok(Some(data)) => Ok(data),
            Ok(None) => {
                let error = CommonErrors::missing_resource_new(state, &format!("missing state: {}", state));
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

    async fn create_from_basic(&self, model: Model) -> anyhow::Result<Model> {
        let active_model = model.to_active();
        match active_model.insert(self.db()).await {
            Ok(data) => Ok(data),
            Err(e) => {
                let error = CommonErrors::database_new(&e.to_string());
                error!("{}", error.log());
                bail!(error)
            }
        }
    }
}
