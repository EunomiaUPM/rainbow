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

use crate::mates::entities::busmates;
use crate::mates::entities::mates;
use crate::mates::repo::{MateRepoFactory, MateRepoTrait};
use anyhow::{anyhow, bail};
use axum::async_trait;
use chrono;
use rainbow_common::mates::mates::VerifyTokenRequest;
use rainbow_common::mates::BusMates;
use rainbow_common::mates::Mates;
use sea_orm::sea_query::OnConflict;
use sea_orm::{ActiveModelTrait, ActiveValue, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, QueryFilter, QuerySelect};

#[derive(Clone)]
pub struct MateRepoForSql {
    db_connection: DatabaseConnection,
}

impl MateRepoForSql {
    fn new(db_connection: DatabaseConnection) -> Self {
        Self { db_connection }
    }
}

impl MateRepoFactory for MateRepoForSql {
    fn create_repo(db_connection: DatabaseConnection) -> Self
    where
        Self: Sized,
    {
        Self::new(db_connection)
    }
}

#[async_trait]
impl MateRepoTrait for MateRepoForSql {
    async fn get_all_mates(
        &self,
        limit: Option<u64>,
        offset: Option<u64>,
    ) -> anyhow::Result<Vec<mates::Model>> {
        let mates = mates::Entity::find()
            .limit(limit.unwrap_or(100000))
            .offset(offset.unwrap_or(0))
            .all(&self.db_connection)
            .await;
        match mates {
            Ok(mates) => Ok(mates),
            Err(e) => bail!("Failed to fetch data: {}", e),
        }
    }

    async fn get_mate_by_id(&self, id: String) -> anyhow::Result<mates::Model> {
        let mate = mates::Entity::find_by_id(&id).one(&self.db_connection).await;

        match mate {
            Ok(Some(mate)) => Ok(mate),
            Ok(None) => bail!("NO authentication with id {}", id),
            Err(e) => bail!("Failed to fetch data: {}", e),
        }
    }

    async fn get_mate_me(&self) -> anyhow::Result<Option<mates::Model>> {
        let mate = mates::Entity::find()
            .filter(mates::Column::IsMe.eq(true))
            .one(&self.db_connection)
            .await
            .map_err(|e| anyhow!("No able to save {}", e.to_string()))?;
        Ok(mate)
    }

    async fn get_mate_by_token(&self, verify_token_request: VerifyTokenRequest) -> anyhow::Result<mates::Model> {
        let mate = mates::Entity::find()
            .filter(mates::Column::Token.eq(verify_token_request.token))
            .one(&self.db_connection)
            .await
            .map_err(|e| anyhow!("No able to fetch {}", e.to_string()))?
            .ok_or(anyhow!("No mate associated with token"))?;
        Ok(mate)
    }

    async fn create_mate(&self, mate: Mates) -> anyhow::Result<mates::Model> {
        let mate = mates::ActiveModel {
            participant_id: ActiveValue::Set(mate.participant_id),
            participant_slug: ActiveValue::Set(mate.participant_slug),
            participant_type: ActiveValue::Set(mate.participant_type),
            base_url: ActiveValue::Set(mate.base_url),
            token: ActiveValue::Set(mate.token),
            token_actions: ActiveValue::Set(mate.token_actions),
            saved_at: ActiveValue::Set(mate.saved_at),
            last_interaction: ActiveValue::Set(mate.last_interaction),
            is_me: ActiveValue::Set(mate.is_me),
        };

        let mate = match mates::Entity::insert(mate)
            .on_conflict(
                OnConflict::column(mates::Column::ParticipantId)
                    .update_columns([
                        mates::Column::BaseUrl,
                        mates::Column::Token,
                        mates::Column::TokenActions,
                        mates::Column::LastInteraction,
                    ])
                    .to_owned(),
            )
            .exec_with_returning(&self.db_connection)
            .await
        {
            Ok(mate) => mate,
            Err(e) => {
                println!("Failed to insert mate: {}", e);
                bail!("Failed to insert mate: {}", e)
            }
        };

        Ok(mate)
    }

    async fn update_mate(&self, mate: Mates) -> anyhow::Result<mates::Model> {
        let id = mate.participant_id;
        let mate = mates::Entity::find_by_id(&id).one(&self.db_connection).await;

        match mate {
            Ok(Some(mate)) => Ok(mate),
            Ok(None) => bail!("NO authentication with id {}", id),
            Err(e) => bail!("Failed to fetch data: {}", e),
        }
    }

    async fn delete_mate(&self, id: String) -> anyhow::Result<()> {
        let mut entry = match mates::Entity::find_by_id(&id).one(&self.db_connection).await {
            Ok(Some(entry)) => entry,
            Ok(None) => bail!("No entry found with ID: {}", id),
            Err(e) => bail!("Failed to fetch data: {}", e),
        };
        let ret = entry.clone();
        let active_model = entry.into_active_model();
        active_model.delete(&self.db_connection).await?;

        Ok(())
    }

    async fn get_all_busmates(
        &self,
        limit: Option<u64>,
        offset: Option<u64>,
    ) -> anyhow::Result<Vec<busmates::Model>> {
        let busmates = busmates::Entity::find()
            .limit(limit.unwrap_or(100000))
            .offset(offset.unwrap_or(0))
            .all(&self.db_connection)
            .await;
        match busmates {
            Ok(busmates) => Ok(busmates),
            Err(e) => bail!("Failed to fetch data: {}", e),
        }
    }

    async fn get_busmate_by_id(&self, id: String) -> anyhow::Result<busmates::Model> {
        let busmates = busmates::Entity::find_by_id(&id).one(&self.db_connection).await;

        match busmates {
            Ok(Some(busmates)) => Ok(busmates),
            Ok(None) => bail!("NO authentication with id {}", id),
            Err(e) => bail!("Failed to fetch data: {}", e),
        }
    }



    async fn create_busmate(&self, busmate: BusMates) -> anyhow::Result<busmates::Model> {
        let busmate = busmates::ActiveModel {
            id: ActiveValue::Set(busmate.id),
            participant_id: ActiveValue::Set(busmate.participant_id),
            token: ActiveValue::Set(busmate.token),
            token_actions: ActiveValue::Set(busmate.token_actions),
            saved_at: ActiveValue::Set(busmate.saved_at),
            last_interaction: ActiveValue::Set(busmate.last_interaction),
        };

        let busmate = match busmates::Entity::insert(busmate)
            .on_conflict(
                OnConflict::column(busmates::Column::Id)
                    .update_columns([
                        busmates::Column::Token,
                        busmates::Column::TokenActions,
                        busmates::Column::LastInteraction,
                    ])
                    .to_owned(),
            )
            .exec_with_returning(&self.db_connection)
            .await
        {
            Ok(busmate) => busmate,
            Err(e) => {
                println!("Failed to insert busmate: {}", e);
                bail!("Failed to insert busmate: {}", e)
            }
        };

        Ok(busmate)
    }

    async fn update_busmate(&self, busmate: BusMates) -> anyhow::Result<busmates::Model> {
        let id = busmate.participant_id;
        let busmate = busmates::Entity::find_by_id(&id).one(&self.db_connection).await;

        match busmate {
            Ok(Some(busmate)) => Ok(busmate),
            Ok(None) => bail!("NO authentication with id {}", id),
            Err(e) => bail!("Failed to fetch data: {}", e),
        }
    }

    async fn delete_busmate(&self, id: String) -> anyhow::Result<()> {
        let mut entry = match busmates::Entity::find_by_id(&id).one(&self.db_connection).await {
            Ok(Some(entry)) => entry,
            Ok(None) => bail!("No entry found with ID: {}", id),
            Err(e) => bail!("Failed to fetch data: {}", e),
        };
        let ret = entry.clone();
        let active_model = entry.into_active_model();
        active_model.delete(&self.db_connection).await?;

        Ok(())
    }
}
