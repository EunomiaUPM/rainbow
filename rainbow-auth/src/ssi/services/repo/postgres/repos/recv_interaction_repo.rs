/*
 * Copyright (C) 2025 - Universidad Politécnica de Madrid - UPM
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

use anyhow::bail;
use axum::async_trait;
use rainbow_common::data::BasicRepoTrait;
use rainbow_common::errors::{CommonErrors, ErrorLog};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use tracing::error;

use crate::ssi::data::entities::recv_interaction::{Column, Entity, Model, NewModel};
use crate::ssi::services::repo::subtraits::recv_interaction_trait::RecvInteractionTrait;

pub struct RecvInteractionRepo {
    db_connection: DatabaseConnection
}

impl RecvInteractionRepo {
    pub fn new(db_connection: DatabaseConnection) -> Self { Self { db_connection } }
}

impl BasicRepoTrait<Entity, NewModel> for RecvInteractionRepo {
    fn db(&self) -> &DatabaseConnection { &self.db_connection }
}

#[async_trait]
impl RecvInteractionTrait for RecvInteractionRepo {
    async fn get_by_reference(&self, reference: &str) -> anyhow::Result<Model> {
        match Entity::find().filter(Column::InteractRef.eq(reference)).one(self.db()).await {
            Ok(Some(data)) => Ok(data),
            Ok(None) => {
                let error = CommonErrors::missing_resource_new(
                    reference,
                    &format!("missing reference: {}", reference)
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

    async fn get_by_cont_id(&self, cont_id: &str) -> anyhow::Result<Model> {
        match Entity::find().filter(Column::ContinueId.eq(cont_id)).one(self.db()).await {
            Ok(Some(data)) => Ok(data),
            Ok(None) => {
                let error = CommonErrors::missing_resource_new(
                    cont_id,
                    &format!("missing cont_id: {}", cont_id)
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

    async fn get_by_some_id(&self, some_id: &str) -> anyhow::Result<Option<Model>> {
        match Entity::find_by_id(some_id).one(self.db()).await {
            Ok(data) => Ok(data),
            Err(e) => {
                let error = CommonErrors::database_new(&e.to_string());
                error!("{}", error.log());
                bail!(error)
            }
        }
    }
}
