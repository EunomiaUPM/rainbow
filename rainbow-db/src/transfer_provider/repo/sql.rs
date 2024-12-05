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


use crate::transfer_provider::entities::agreements;
use crate::transfer_provider::entities::transfer_message;
use crate::transfer_provider::entities::transfer_message::Model;
use crate::transfer_provider::entities::transfer_process;
use crate::transfer_provider::repo::{
    AgreementsRepo, EditAgreementModel, EditTransferProcessModel, NewAgreementModel,
    NewTransferMessageModel, NewTransferProcessModel, TransferMessagesRepo, TransferProcessRepo,
};
use anyhow::bail;
use axum::async_trait;
use rainbow_common::config::database::get_db_connection;
use rainbow_common::protocol::transfer::{TransferMessageTypesForDb, TransferStateForDb};
use sea_orm::{ActiveModelTrait, ActiveValue, ColumnTrait, EntityTrait, QueryFilter, QuerySelect};
use uuid::Uuid;

pub struct TransferProviderRepoForSql {}

// TODO create impl From everywhere!!

#[async_trait]
impl TransferProcessRepo for TransferProviderRepoForSql {
    async fn get_all_transfer_processes(
        &self,
        limit: Option<u64>,
        page: Option<u64>,
    ) -> anyhow::Result<Vec<transfer_process::Model>> {
        let db_connection = get_db_connection().await;
        let transfer_process = transfer_process::Entity::find()
            .limit(limit.unwrap_or(100000))
            .offset(page.unwrap_or(0))
            .all(db_connection)
            .await;
        match transfer_process {
            Ok(transfer_process) => Ok(transfer_process),
            Err(_) => bail!("Failed to fetch transfer process"),
        }
    }

    async fn get_transfer_process_by_provider(
        &self,
        pid: Uuid,
    ) -> anyhow::Result<Option<transfer_process::Model>> {
        let db_connection = get_db_connection().await;
        let transfer_process = transfer_process::Entity::find_by_id(pid).one(db_connection).await;
        match transfer_process {
            Ok(transfer_process) => Ok(transfer_process),
            Err(_) => bail!("Failed to fetch transfer process"),
        }
    }

    async fn get_transfer_process_by_consumer(
        &self,
        pid: Uuid,
    ) -> anyhow::Result<Option<transfer_process::Model>> {
        let db_connection = get_db_connection().await;
        let transfer_process = transfer_process::Entity::find()
            .filter(transfer_process::Column::ConsumerPid.eq(pid))
            .one(db_connection)
            .await;
        match transfer_process {
            Ok(transfer_process) => Ok(transfer_process),
            Err(_) => bail!("Failed to fetch transfer process"),
        }
    }

    async fn get_transfer_process_by_data_plane(&self, pid: Uuid) -> anyhow::Result<Option<transfer_process::Model>> {
        let db_connection = get_db_connection().await;
        let transfer_process = transfer_process::Entity::find()
            .filter(transfer_process::Column::DataPlaneId.eq(pid))
            .one(db_connection)
            .await;
        match transfer_process {
            Ok(transfer_process) => Ok(transfer_process),
            Err(_) => bail!("Failed to fetch transfer process"),
        }
    }

    async fn put_transfer_process(
        &self,
        pid: Uuid,
        new_transfer_process: EditTransferProcessModel,
    ) -> anyhow::Result<transfer_process::Model> {
        let db_connection = get_db_connection().await;

        let old_model = transfer_process::Entity::find_by_id(pid).one(db_connection).await;
        let old_model = match old_model {
            Ok(old_model) => match old_model {
                Some(old_model) => old_model,
                None => bail!("Failed to fetch old model"),
            },
            Err(_) => bail!("Failed to fetch old model"),
        };

        let mut old_active_model: transfer_process::ActiveModel = old_model.into();
        if let Some(provider_pid) = new_transfer_process.provider_pid {
            old_active_model.provider_pid = ActiveValue::Set(provider_pid);
        }
        if let Some(consumer_pid) = new_transfer_process.consumer_pid {
            old_active_model.consumer_pid = ActiveValue::Set(Some(consumer_pid));
        }
        if let Some(agreement_id) = new_transfer_process.agreement_id {
            old_active_model.agreement_id = ActiveValue::Set(agreement_id);
        }
        if let Some(data_plane_id) = new_transfer_process.data_plane_id {
            old_active_model.data_plane_id = ActiveValue::Set(Some(data_plane_id));
        }
        if let Some(state) = new_transfer_process.state {
            old_active_model.state = ActiveValue::Set(state);
        }
        old_active_model.updated_at = ActiveValue::Set(Some(chrono::Utc::now().naive_utc()));
        let model = old_active_model.update(db_connection).await;
        match model {
            Ok(model) => Ok(model),
            Err(_) => bail!("Failed to update model"),
        }
    }

    async fn create_transfer_process(
        &self,
        new_transfer_process: NewTransferProcessModel,
    ) -> anyhow::Result<transfer_process::Model> {
        let db_connection = get_db_connection().await;
        let model = transfer_process::ActiveModel {
            provider_pid: ActiveValue::Set(new_transfer_process.provider_pid),
            consumer_pid: ActiveValue::Set(Some(new_transfer_process.consumer_pid)),
            agreement_id: ActiveValue::Set(new_transfer_process.agreement_id),
            data_plane_id: ActiveValue::Set(Some(new_transfer_process.data_plane_id)),
            state: ActiveValue::Set(TransferStateForDb::REQUESTED),
            created_at: ActiveValue::Set(chrono::Utc::now().naive_utc()),
            updated_at: ActiveValue::Set(None),
        };
        let transfer_process =
            transfer_process::Entity::insert(model).exec_with_returning(db_connection).await;

        match transfer_process {
            Ok(transfer_process) => Ok(transfer_process),
            Err(_) => bail!("Failed to create model"),
        }
    }

    async fn delete_transfer_process(&self, pid: Uuid) -> anyhow::Result<()> {
        let db_connection = get_db_connection().await;
        let transfer_process =
            transfer_process::Entity::delete_by_id(pid).exec(db_connection).await;
        match transfer_process {
            Ok(delete_result) => match delete_result.rows_affected {
                0 => bail!("Not found"),
                _ => Ok(()),
            },
            Err(_) => bail!("Failed to fetch transfer process"),
        }
    }
}

#[async_trait]
impl TransferMessagesRepo for TransferProviderRepoForSql {
    async fn get_all_transfer_messages(
        &self,
        limit: Option<u64>,
        page: Option<u64>,
    ) -> anyhow::Result<Vec<transfer_message::Model>> {
        let db_connection = get_db_connection().await;
        let transfer_message = transfer_message::Entity::find()
            .limit(limit.unwrap_or(100000))
            .offset(page.unwrap_or(0))
            .all(db_connection)
            .await;
        match transfer_message {
            Ok(transfer_message) => Ok(transfer_message),
            Err(_) => bail!("Failed to fetch transfer messages"),
        }
    }

    async fn get_all_transfer_messages_by_provider(&self, pid: Uuid) -> anyhow::Result<Vec<Model>> {
        let db_connection = get_db_connection().await;
        let transfer_message = transfer_message::Entity::find()
            .filter(transfer_message::Column::TransferProcessId.eq(pid))
            .all(db_connection)
            .await;
        match transfer_message {
            Ok(transfer_message) => Ok(transfer_message),
            Err(_) => bail!("Failed to fetch transfer messages"),
        }
    }

    async fn get_transfer_message_by_id(
        &self,
        pid: Uuid,
    ) -> anyhow::Result<Option<transfer_message::Model>> {
        let db_connection = get_db_connection().await;
        let transfer_message = transfer_message::Entity::find_by_id(pid).one(db_connection).await;
        match transfer_message {
            Ok(transfer_message) => Ok(transfer_message),
            Err(_) => bail!("Failed to fetch transfer message"),
        }
    }

    async fn put_transfer_message(
        &self,
        pid: Uuid,
        new_transfer_process: transfer_message::ActiveModel,
    ) -> anyhow::Result<Option<transfer_message::Model>> {
        Ok(None)
    }

    async fn create_transfer_message(
        &self,
        pid: Uuid,
        new_transfer_message: NewTransferMessageModel,
    ) -> anyhow::Result<transfer_message::Model> {
        let db_connection = get_db_connection().await;

        let message_type = TransferMessageTypesForDb::try_from(new_transfer_message.message_type);
        let message_type = match message_type {
            Ok(message_type) => message_type,
            Err(_) => bail!("Failed to parse message type"),
        };

        let model = transfer_message::ActiveModel {
            id: ActiveValue::Set(Uuid::new_v4()),
            transfer_process_id: ActiveValue::Set(pid),
            created_at: ActiveValue::Set(chrono::Utc::now().naive_utc()),
            message_type: ActiveValue::Set(message_type),
            from: ActiveValue::Set(new_transfer_message.from),
            to: ActiveValue::Set(new_transfer_message.to),
            content: ActiveValue::Set(new_transfer_message.content),
        };

        let model =
            transfer_message::Entity::insert(model).exec_with_returning(db_connection).await;
        match model {
            Ok(model) => Ok(model),
            Err(_) => bail!("Failed to create message"),
        }
    }

    async fn delete_transfer_message(&self, pid: Uuid) -> anyhow::Result<()> {
        let db_connection = get_db_connection().await;
        let transfer_message =
            transfer_message::Entity::delete_by_id(pid).exec(db_connection).await;
        match transfer_message {
            Ok(delete_result) => match delete_result.rows_affected {
                0 => bail!("Not found"),
                _ => Ok(()),
            },
            Err(_) => bail!("Failed to fetch transfer message"),
        }
    }
}

#[async_trait]
impl AgreementsRepo for TransferProviderRepoForSql {
    async fn get_all_agreements(
        &self,
        limit: Option<u64>,
        page: Option<u64>,
    ) -> anyhow::Result<Vec<agreements::Model>> {
        let db_connection = get_db_connection().await;
        let agreements = agreements::Entity::find()
            .limit(limit.unwrap_or(100000))
            .offset(page.unwrap_or(0))
            .all(db_connection)
            .await;
        match agreements {
            Ok(agreements) => Ok(agreements),
            Err(_) => bail!("Failed to fetch agreements"),
        }
    }

    async fn get_agreement_by_id(&self, id: Uuid) -> anyhow::Result<Option<agreements::Model>> {
        let db_connection = get_db_connection().await;
        let agreement = agreements::Entity::find_by_id(id).one(db_connection).await;
        match agreement {
            Ok(agreement) => Ok(agreement),
            Err(_) => bail!("Failed to fetch agreement"),
        }
    }

    async fn put_agreement(
        &self,
        id: Uuid,
        new_agreement: EditAgreementModel,
    ) -> anyhow::Result<agreements::Model> {
        let db_connection = get_db_connection().await;

        let old_model = agreements::Entity::find_by_id(id).one(db_connection).await;
        let old_model = match old_model {
            Ok(old_model) => match old_model {
                Some(old_model) => old_model,
                None => bail!("Not found"),
            },
            Err(_) => bail!("Failed to fetch old model"),
        };

        let mut old_active_model: agreements::ActiveModel = old_model.into();
        if let Some(data_service_id) = new_agreement.data_service_id {
            old_active_model.data_service_id = ActiveValue::Set(data_service_id);
        }
        if let Some(identity) = new_agreement.identity {
            old_active_model.identity = ActiveValue::Set(Some(identity));
        }
        if let Some(identity_token) = new_agreement.identity_token {
            old_active_model.identity_token = ActiveValue::Set(Some(identity_token));
        }

        let model = old_active_model.update(db_connection).await;
        match model {
            Ok(model) => Ok(model),
            Err(_) => bail!("Failed to update model"),
        }
    }

    async fn create_agreement(
        &self,
        new_agreement: NewAgreementModel,
    ) -> anyhow::Result<agreements::Model> {
        let db_connection = get_db_connection().await;

        let model = agreements::ActiveModel {
            agreement_id: ActiveValue::Set(Uuid::new_v4()),
            data_service_id: ActiveValue::Set(new_agreement.data_service_id),
            identity: ActiveValue::Set(new_agreement.identity),
            identity_token: ActiveValue::Set(new_agreement.identity_token),
        };

        let agreement = agreements::Entity::insert(model).exec_with_returning(db_connection).await;
        match agreement {
            Ok(agreement) => Ok(agreement),
            Err(_) => bail!("Failed to create agreement"),
        }
    }

    async fn delete_agreement(&self, id: Uuid) -> anyhow::Result<()> {
        let db_connection = get_db_connection().await;
        let agreement = agreements::Entity::delete_by_id(id).exec(db_connection).await;
        match agreement {
            Ok(delete_result) => match delete_result.rows_affected {
                0 => bail!("Not found"),
                _ => Ok(()),
            },
            Err(_) => bail!("Failed to fetch agreement"),
        }
    }
}
