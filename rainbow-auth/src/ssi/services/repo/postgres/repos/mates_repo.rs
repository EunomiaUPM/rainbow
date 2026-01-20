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
use sea_orm::QueryFilter;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait};
use tracing::error;
use urn::Urn;

use crate::ssi::data::entities::mates::{Column, Entity, Model, NewModel};
use crate::ssi::services::repo::subtraits::MatesTrait;

pub struct MatesRepo {
    db_connection: DatabaseConnection
}

impl MatesRepo {
    pub fn new(db_connection: DatabaseConnection) -> Self { Self { db_connection } }
}

impl BasicRepoTrait<Entity, NewModel> for MatesRepo {
    fn db(&self) -> &DatabaseConnection { &self.db_connection }
}

#[async_trait]
impl MatesTrait for MatesRepo {
    async fn get_me(&self) -> anyhow::Result<Model> {
        match Entity::find().filter(Column::IsMe.eq(true)).one(self.db()).await {
            Ok(Some(data)) => Ok(data),
            Ok(None) => {
                let error =
                    CommonErrors::missing_resource_new("me", "missing myself in the database");
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
                    .update_columns([
                        Column::BaseUrl,
                        Column::LastInteraction,
                        Column::Token,
                        Column::ParticipantSlug
                    ])
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

    async fn get_batch(&self, ids: &Vec<Urn>) -> anyhow::Result<Vec<Model>> {
        let ids = ids.iter().map(|i| i.to_string()).collect::<Vec<String>>();
        let mates = Entity::find().filter(Column::ParticipantId.is_in(ids)).all(self.db()).await?;
        Ok(mates)
    }
}
