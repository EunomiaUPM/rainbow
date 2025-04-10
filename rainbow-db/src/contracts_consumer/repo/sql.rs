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

use super::super::entities::cn_process;
use crate::contracts_consumer::repo::{CnErrors, ContractNegotiationConsumerProcessRepo, ContractNegotiationConsumerRepoFactory, EditContractNegotiationProcess, NewContractNegotiationProcess};
use axum::async_trait;
use rainbow_common::utils::get_urn;
use sea_orm::{ActiveModelTrait, ActiveValue, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QuerySelect};
use urn::Urn;

pub struct ContractNegotiationConsumerRepoForSql {
    db_connection: DatabaseConnection,
}

impl ContractNegotiationConsumerRepoForSql {
    fn new(db_connection: DatabaseConnection) -> Self {
        Self { db_connection }
    }
}

impl ContractNegotiationConsumerRepoFactory for ContractNegotiationConsumerRepoForSql {
    fn create_repo(database_connection: DatabaseConnection) -> Self {
        Self::new(database_connection)
    }
}

#[async_trait]
impl ContractNegotiationConsumerProcessRepo for ContractNegotiationConsumerRepoForSql {
    async fn get_all_cn_processes(
        &self,
        limit: Option<u64>,
        page: Option<u64>,
    ) -> anyhow::Result<Vec<cn_process::Model>, CnErrors> {
        let cn_processes = cn_process::Entity::find()
            .limit(limit.unwrap_or(10000))
            .offset(page.unwrap_or(0))
            .all(&self.db_connection)
            .await
            .map_err(|err| CnErrors::ErrorFetchingCNProcess(err.into()))?;
        Ok(cn_processes)
    }

    async fn get_cn_process_by_provider_id(
        &self,
        provider_id: Urn,
    ) -> anyhow::Result<Option<cn_process::Model>, CnErrors> {
        let cn_processes = cn_process::Entity::find()
            .filter(cn_process::Column::ProviderId.eq(provider_id.as_str()))
            .one(&self.db_connection)
            .await
            .map_err(|err| CnErrors::ErrorFetchingCNProcess(err.into()))?;
        Ok(cn_processes)
    }

    async fn get_cn_process_by_consumer_id(
        &self,
        consumer_id: Urn,
    ) -> anyhow::Result<Option<cn_process::Model>, CnErrors> {
        let cn_processes = cn_process::Entity::find()
            .filter(cn_process::Column::ConsumerId.eq(consumer_id.as_str()))
            .one(&self.db_connection)
            .await
            .map_err(|err| CnErrors::ErrorFetchingCNProcess(err.into()))?;
        Ok(cn_processes)
    }


    async fn get_cn_process_by_cn_id(
        &self,
        cn_process_id: Urn,
    ) -> anyhow::Result<Option<cn_process::Model>, CnErrors> {
        let cn_process = cn_process::Entity::find_by_id(cn_process_id.as_str())
            .one(&self.db_connection)
            .await
            .map_err(|err| CnErrors::ErrorFetchingCNProcess(err.into()))?;
        Ok(cn_process)
    }

    async fn put_cn_process(
        &self,
        cn_process_id: Urn,
        edit_cn_process: EditContractNegotiationProcess,
    ) -> anyhow::Result<cn_process::Model, CnErrors> {
        let old_model = cn_process::Entity::find_by_id(cn_process_id.as_str())
            .one(&self.db_connection)
            .await
            .map_err(|err| CnErrors::ErrorFetchingCNProcess(err.into()))?
            .ok_or(CnErrors::CNProcessNotFound)?;

        let mut old_active_model: cn_process::ActiveModel = old_model.into();
        old_active_model.updated_at = ActiveValue::Set(Some(chrono::Utc::now().naive_utc()));

        let model = old_active_model
            .update(&self.db_connection)
            .await
            .map_err(|err| CnErrors::ErrorUpdatingCNProcess(err.into()))?;
        Ok(model)
    }

    async fn create_cn_process(
        &self,
        new_cn_process: NewContractNegotiationProcess,
    ) -> anyhow::Result<cn_process::Model, CnErrors> {
        let model = cn_process::ActiveModel {
            cn_process_id: ActiveValue::Set(get_urn(None).to_string()),
            provider_id: ActiveValue::Set(Option::from(
                get_urn(new_cn_process.provider_id).to_string(),
            )),
            consumer_id: ActiveValue::Set(Option::from(
                get_urn(new_cn_process.consumer_id).to_string(),
            )),
            created_at: ActiveValue::Set(chrono::Utc::now().naive_utc()),
            updated_at: ActiveValue::Set(None),
        };

        let cn_process = cn_process::Entity::insert(model)
            .exec_with_returning(&self.db_connection)
            .await
            .map_err(|err| CnErrors::ErrorCreatingCNProcess(err.into()))?;
        Ok(cn_process)
    }

    async fn delete_cn_process(&self, cn_process_id: Urn) -> anyhow::Result<(), CnErrors> {
        match cn_process::Entity::delete_by_id(cn_process_id.as_str())
            .exec(&self.db_connection)
            .await
            .map_err(|err| CnErrors::ErrorDeletingCNProcess(err.into()))?
            .rows_affected
        {
            0 => Err(CnErrors::CNProcessNotFound),
            _ => Ok(()),
        }
    }
}