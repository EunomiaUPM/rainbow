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

use crate::data::entities::minions::{Column, Entity, Model, NewModel};
use crate::data::repo_factory::repos::GenericRepo;
use crate::data::repo_factory::traits::{BasicRepoTrait, IntoActiveSet, MinionsRepoTrait};
use axum::async_trait;
use sea_orm::sea_query::OnConflict;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

#[derive(Clone)]
pub struct MinionsRepo {
    inner: GenericRepo<Entity, NewModel>,
}

impl MinionsRepo {
    pub fn new(db_connection: DatabaseConnection) -> Self {
        Self { inner: GenericRepo::new(db_connection) }
    }
}

#[async_trait]
impl BasicRepoTrait<Model, NewModel> for MinionsRepo {
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
impl MinionsRepoTrait for MinionsRepo {
    async fn get_me(&self) -> anyhow::Result<Option<Model>> {
        let me = Entity::find().filter(Column::IsMe.eq(true)).one(&self.inner.db_connection).await?;
        Ok(me)
    }

    async fn force_create(&self, minion: NewModel) -> anyhow::Result<Model> {
        let active_mate = minion.to_active();
        let mate = Entity::insert(active_mate)
            .on_conflict(
                OnConflict::column(Column::ParticipantId)
                    .update_columns([Column::BaseUrl, Column::LastInteraction, Column::VcUri, Column::ParticipantSlug])
                    .to_owned(),
            )
            .exec_with_returning(&self.inner.db_connection)
            .await?;
        Ok(mate)
    }
}
