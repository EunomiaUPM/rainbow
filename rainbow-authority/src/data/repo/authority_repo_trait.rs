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

use axum::async_trait;
use sea_orm::{ActiveValue, DatabaseConnection};
use crate::data::entities::auth;
use crate::data::repo::authority_factory::{AuthorityRepoFactory, AuthorityRepoTrait};
use crate::data::repo::basic_repo_trait::BasicRepoTrait;

#[derive(Clone)]
pub struct AuthorityRepoForSql {
    db_connection: DatabaseConnection,
}

impl AuthorityRepoForSql {
    fn new(db_connection: DatabaseConnection) -> Self {
        Self { db_connection }
    }
}

impl AuthorityRepoFactory for AuthorityRepoForSql {
    fn create_repo(db_connection: DatabaseConnection) -> Box<dyn AuthorityRepoTrait> {
        todo!()
    }
}

#[async_trait]
impl<T> BasicRepoTrait<T> for AuthorityRepoForSql {
    async fn get_all(&self, limit: Option<u64>, offset: Option<u64>) -> anyhow::Result<Vec<T>> {

        let model = auth::ActiveModel {
            id: ActiveValue::Set(model.id),
            client: ActiveValue::Set(model.client),
            actions: ActiveValue::Set(model.actions),
            status: ActiveValue::Set(model.status),
            token: ActiveValue::Set(model.token),
            created_at: ActiveValue::Set(model.created_at),
            ended_at: ActiveValue::Set(model.ended_at),
        };

        let new_model = auth::Entity::insert(model).exec_with_returning(&self.db_connection).await?;
        Ok(new_model)
    }

    async fn get_by_id(&self, id: &str) -> anyhow::Result<Option<T>> {
        todo!()
    }

    async fn create(&self, model: T) -> anyhow::Result<T> {
        todo!()
    }

    async fn delete(&self, id: &str) -> anyhow::Result<()> {
        todo!()
    }

    async fn edit(&self, model: T) -> anyhow::Result<T> {
        todo!()
    }
}