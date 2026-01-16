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
use rainbow_common::data::{BasicRepoTrait, IntoActiveSet};
use rainbow_common::errors::{CommonErrors, ErrorLog};
use sea_orm::sea_query::OnConflict;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use tracing::error;

use crate::ssi::data::entities::business_mates::{Column, Entity, Model, NewModel};
use crate::ssi::services::repo::subtraits::business_mates_trait::BusinessMatesRepoTrait;

#[derive(Clone)]
pub struct BusinessMatesRepo {
    db_connection: DatabaseConnection
}

impl BusinessMatesRepo {
    pub fn new(db_connection: DatabaseConnection) -> Self { Self { db_connection } }
}

impl BasicRepoTrait<Entity, NewModel> for BusinessMatesRepo {
    fn db(&self) -> &DatabaseConnection { &self.db_connection }
}

#[async_trait]
impl BusinessMatesRepoTrait for BusinessMatesRepo {
    async fn get_by_token(&self, token: &str) -> anyhow::Result<Model> {
        match Entity::find().filter(Column::Token.eq(token)).one(self.db()).await {
            Ok(Some(data)) => Ok(data),
            Ok(None) => {
                let error =
                    CommonErrors::missing_resource_new(token, &format!("missing token: {}", token));
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

    async fn force_create(&self, mate: NewModel) -> anyhow::Result<Model> {
        let active_mate = mate.to_active();
        match Entity::insert(active_mate)
            .on_conflict(
                OnConflict::column(Column::ParticipantId)
                    .update_columns([Column::Token, Column::LastInteraction])
                    .to_owned()
            )
            .exec_with_returning(self.db())
            .await
        {
            Ok(data) => Ok(data),
            Err(e) => {
                let error = CommonErrors::database_new(&e.to_string());
                error!("{}", error.log());
                bail!(error)
            }
        }
    }
}
