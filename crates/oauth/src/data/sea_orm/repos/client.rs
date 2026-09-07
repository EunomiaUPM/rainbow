/*
 * Copyright (C) 2026 - Universidad Politécnica de Madrid - UPM
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program. If not, see <https://www.gnu.org/licenses/>.
 */

use std::sync::Arc;

use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait};
use ymir::errors::{Outcome, RepoIntoErrors};

use crate::data::repositories::client::{ClientRepository, ClientRepositoryError};
use crate::data::sea_orm::orm::client as orm;
use crate::entities::client::Client;

pub(crate) struct SeaOrmClientRepository {
    db: Arc<DatabaseConnection>,
}

impl SeaOrmClientRepository {
    pub fn new(db: Arc<DatabaseConnection>) -> Self {
        Self { db }
    }
}

#[async_trait::async_trait]
impl ClientRepository for SeaOrmClientRepository {
    async fn get_all(&self) -> Outcome<Vec<Client>> {
        let models = orm::Entity::find()
            .all(self.db.as_ref())
            .await
            .map_err(|e| ClientRepositoryError::Db(Box::new(e)).into_errors())?;

        models.into_iter().map(orm::Model::into_domain).collect()
    }

    async fn get_by_client_id(&self, client_id: &str) -> Outcome<Option<Client>> {
        orm::Entity::find_by_id(client_id)
            .one(self.db.as_ref())
            .await
            .map_err(|e| ClientRepositoryError::Db(Box::new(e)).into_errors())?
            .map(orm::Model::into_domain)
            .transpose()
    }

    async fn create(&self, client: &Client) -> Outcome<Client> {
        orm::ActiveModel::from_domain(client)
            .insert(self.db.as_ref())
            .await
            .map_err(|e| ClientRepositoryError::Db(Box::new(e)).into_errors())
            .and_then(orm::Model::into_domain)
    }

    async fn delete(&self, client_id: &str) -> Outcome<()> {
        orm::Entity::delete_by_id(client_id)
            .exec(self.db.as_ref())
            .await
            .map_err(|e| ClientRepositoryError::Db(Box::new(e)).into_errors())?;
        Ok(())
    }
}
