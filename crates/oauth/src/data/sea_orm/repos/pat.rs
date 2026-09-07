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

use chrono::Utc;
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use uuid::Uuid;
use ymir::errors::{Outcome, RepoIntoErrors};

use crate::data::repositories::pat::{PatRepository, PatRepositoryError};
use crate::data::sea_orm::orm::pat as orm;
use crate::entities::pat::PersonalAccessToken;

pub(crate) struct SeaOrmPatRepository {
    db: Arc<DatabaseConnection>,
}

impl SeaOrmPatRepository {
    pub fn new(db: Arc<DatabaseConnection>) -> Self {
        Self { db }
    }
}

#[async_trait::async_trait]
impl PatRepository for SeaOrmPatRepository {
    async fn create(&self, pat: &PersonalAccessToken) -> Outcome<PersonalAccessToken> {
        orm::ActiveModel::from_domain(pat)
            .insert(self.db.as_ref())
            .await
            .map_err(|e| PatRepositoryError::Db(Box::new(e)).into_errors())
            .and_then(orm::Model::into_domain)
    }

    async fn get_by_id(&self, id: Uuid) -> Outcome<Option<PersonalAccessToken>> {
        orm::Entity::find_by_id(id)
            .one(self.db.as_ref())
            .await
            .map_err(|e| PatRepositoryError::Db(Box::new(e)).into_errors())?
            .map(orm::Model::into_domain)
            .transpose()
    }

    async fn get_by_hash(&self, token_hash: &str) -> Outcome<Option<PersonalAccessToken>> {
        orm::Entity::find()
            .filter(orm::Column::TokenHash.eq(token_hash))
            .one(self.db.as_ref())
            .await
            .map_err(|e| PatRepositoryError::Db(Box::new(e)).into_errors())?
            .map(orm::Model::into_domain)
            .transpose()
    }

    async fn list_by_tenant(&self, tenant_id: &str) -> Outcome<Vec<PersonalAccessToken>> {
        let models = orm::Entity::find()
            .filter(orm::Column::TenantId.eq(tenant_id))
            .all(self.db.as_ref())
            .await
            .map_err(|e| PatRepositoryError::Db(Box::new(e)).into_errors())?;

        models.into_iter().map(orm::Model::into_domain).collect()
    }

    async fn revoke(&self, id: Uuid) -> Outcome<()> {
        let existing = orm::Entity::find_by_id(id)
            .one(self.db.as_ref())
            .await
            .map_err(|e| PatRepositoryError::Db(Box::new(e)).into_errors())?
            .ok_or_else(|| PatRepositoryError::NotFound.into_errors())?;

        let mut active: orm::ActiveModel = existing.into();
        active.revoked = Set(true);
        active
            .update(self.db.as_ref())
            .await
            .map_err(|e| PatRepositoryError::Db(Box::new(e)).into_errors())?;
        Ok(())
    }

    async fn update_last_used(&self, id: Uuid) -> Outcome<()> {
        let existing = orm::Entity::find_by_id(id)
            .one(self.db.as_ref())
            .await
            .map_err(|e| PatRepositoryError::Db(Box::new(e)).into_errors())?
            .ok_or_else(|| PatRepositoryError::NotFound.into_errors())?;

        let mut active: orm::ActiveModel = existing.into();
        active.last_used_at = Set(Some(Utc::now().into()));
        active
            .update(self.db.as_ref())
            .await
            .map_err(|e| PatRepositoryError::Db(Box::new(e)).into_errors())?;
        Ok(())
    }
}
