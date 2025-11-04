/*
 *
 *  * Copyright (C) 2025 - Universidad Politécnica de Madrid - UPM
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

use crate::transfer_provider::entities::transfer_message;
use crate::transfer_provider::entities::transfer_process;
use crate::transfer_provider::entities::transfer_process::Model;
use crate::transfer_provider::repo::{
    EditTransferMessageModel, EditTransferProcessModel, NewTransferMessageModel, NewTransferProcessModel,
    TransferMessagesRepo, TransferProcessRepo, TransferProviderRepoErrors, TransferProviderRepoFactory,
};
use axum::async_trait;
use rainbow_common::protocol::transfer::TransferState;
use rainbow_common::utils::get_urn;
use sea_orm::{ActiveModelTrait, ActiveValue, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QuerySelect};
use urn::Urn;

pub struct TransferProviderRepoForSql {
    db_connection: DatabaseConnection,
}

impl TransferProviderRepoForSql {
    fn new(db_connection: DatabaseConnection) -> Self {
        Self { db_connection }
    }
}

impl TransferProviderRepoFactory for TransferProviderRepoForSql {
    fn create_repo(database_connection: DatabaseConnection) -> Self {
        Self::new(database_connection)
    }
}

#[async_trait]
impl TransferProcessRepo for TransferProviderRepoForSql {
    async fn get_all_transfer_processes(
        &self,
        limit: Option<u64>,
        page: Option<u64>,
    ) -> anyhow::Result<Vec<transfer_process::Model>, TransferProviderRepoErrors> {
        let transfer_process = transfer_process::Entity::find()
            .limit(limit.unwrap_or(100000))
            .offset(page.unwrap_or(0))
            .all(&self.db_connection)
            .await;
        match transfer_process {
            Ok(transfer_process) => Ok(transfer_process),
            Err(e) => Err(TransferProviderRepoErrors::ErrorFetchingProviderTransferProcess(e.into())),
        }
    }

    async fn get_batch_transfer_processes(
        &self,
        transfer_ids: &Vec<Urn>,
    ) -> Result<Vec<Model>, TransferProviderRepoErrors> {
        let transfer_ids = transfer_ids.iter().map(|t| t.to_string()).collect::<Vec<_>>();
        let transfer_process = transfer_process::Entity::find()
            .filter(transfer_process::Column::ProviderPid.is_in(transfer_ids))
            .all(&self.db_connection)
            .await;
        match transfer_process {
            Ok(transfer_process) => Ok(transfer_process),
            Err(e) => Err(TransferProviderRepoErrors::ErrorFetchingProviderTransferProcess(e.into())),
        }
    }

    async fn get_transfer_process_by_provider(
        &self,
        pid: Urn,
    ) -> anyhow::Result<Option<transfer_process::Model>, TransferProviderRepoErrors> {
        let pid = pid.to_string();
        let transfer_process = transfer_process::Entity::find_by_id(pid).one(&self.db_connection).await;
        match transfer_process {
            Ok(transfer_process) => Ok(transfer_process),
            Err(e) => Err(TransferProviderRepoErrors::ErrorFetchingProviderTransferProcess(e.into())),
        }
    }

    async fn get_transfer_process_by_consumer(
        &self,
        pid: Urn,
    ) -> anyhow::Result<Option<transfer_process::Model>, TransferProviderRepoErrors> {
        let pid = pid.to_string();
        let transfer_process = transfer_process::Entity::find()
            .filter(transfer_process::Column::ConsumerPid.eq(pid))
            .one(&self.db_connection)
            .await;
        match transfer_process {
            Ok(transfer_process) => Ok(transfer_process),
            Err(e) => Err(TransferProviderRepoErrors::ErrorFetchingProviderTransferProcess(e.into())),
        }
    }

    async fn get_transfer_process_by_data_plane(
        &self,
        pid: Urn,
    ) -> anyhow::Result<Option<transfer_process::Model>, TransferProviderRepoErrors> {
        let pid = pid.to_string();
        let transfer_process = transfer_process::Entity::find()
            .filter(transfer_process::Column::DataPlaneId.eq(pid))
            .one(&self.db_connection)
            .await;
        match transfer_process {
            Ok(transfer_process) => Ok(transfer_process),
            Err(e) => Err(TransferProviderRepoErrors::ErrorFetchingProviderTransferProcess(e.into())),
        }
    }

    async fn put_transfer_process(
        &self,
        pid: Urn,
        edit_transfer_process: EditTransferProcessModel,
    ) -> anyhow::Result<transfer_process::Model, TransferProviderRepoErrors> {
        let pid = pid.to_string();

        let old_model = transfer_process::Entity::find_by_id(pid).one(&self.db_connection).await;
        let old_model = match old_model {
            Ok(old_model) => match old_model {
                Some(old_model) => old_model,
                None => return Err(TransferProviderRepoErrors::ProviderTransferProcessNotFound),
            },
            Err(e) => return Err(TransferProviderRepoErrors::ErrorFetchingProviderTransferProcess(e.into())),
        };

        let mut old_active_model: transfer_process::ActiveModel = old_model.into();
        if let Some(provider_pid) = edit_transfer_process.provider_pid {
            old_active_model.provider_pid = ActiveValue::Set(provider_pid.to_string());
        }
        if let Some(consumer_pid) = edit_transfer_process.consumer_pid {
            old_active_model.consumer_pid = ActiveValue::Set(Some(consumer_pid.to_string()));
        }
        if let Some(agreement_id) = edit_transfer_process.agreement_id {
            old_active_model.agreement_id = ActiveValue::Set(agreement_id.to_string());
        }
        if let Some(data_plane_id) = edit_transfer_process.callback_address {
            old_active_model.data_plane_id = ActiveValue::Set(Some(data_plane_id.to_string()));
        }
        if let Some(state) = edit_transfer_process.state {
            old_active_model.state = ActiveValue::Set(state.to_string());
        }
        if let Some(state_attribute) = edit_transfer_process.state_attribute {
            old_active_model.state_attribute = ActiveValue::Set(Option::from(state_attribute.to_string()));
        }
        old_active_model.updated_at = ActiveValue::Set(Some(chrono::Utc::now().naive_utc()));
        let model = old_active_model.update(&self.db_connection).await;
        match model {
            Ok(model) => Ok(model),
            Err(e) => return Err(TransferProviderRepoErrors::ErrorUpdatingProviderTransferProcess(e.into())),
        }
    }

    async fn create_transfer_process(
        &self,
        new_transfer_process: NewTransferProcessModel,
    ) -> anyhow::Result<transfer_process::Model, TransferProviderRepoErrors> {
        let model = transfer_process::ActiveModel {
            provider_pid: ActiveValue::Set(new_transfer_process.provider_pid.to_string()),
            consumer_pid: ActiveValue::Set(Some(new_transfer_process.consumer_pid.to_string())),
            agreement_id: ActiveValue::Set(new_transfer_process.agreement_id.to_string()),
            data_plane_id: ActiveValue::Set(Some(new_transfer_process.callback_address.to_string())),
            associated_consumer: ActiveValue::Set(new_transfer_process.associated_consumer),
            state: ActiveValue::Set(TransferState::REQUESTED.to_string()),
            state_attribute: ActiveValue::Set(None),
            created_at: ActiveValue::Set(chrono::Utc::now().naive_utc()),
            updated_at: ActiveValue::Set(None),
        };
        let transfer_process = transfer_process::Entity::insert(model).exec_with_returning(&self.db_connection).await;

        match transfer_process {
            Ok(transfer_process) => Ok(transfer_process),
            Err(e) => return Err(TransferProviderRepoErrors::ErrorCreatingProviderTransferProcess(e.into())),
        }
    }

    async fn delete_transfer_process(&self, pid: Urn) -> anyhow::Result<(), TransferProviderRepoErrors> {
        let pid = pid.to_string();

        let transfer_process = transfer_process::Entity::delete_by_id(pid).exec(&self.db_connection).await;
        match transfer_process {
            Ok(delete_result) => match delete_result.rows_affected {
                0 => Err(TransferProviderRepoErrors::ProviderTransferProcessNotFound),
                _ => Ok(()),
            },
            Err(e) => Err(TransferProviderRepoErrors::ErrorDeletingProviderTransferProcess(e.into())),
        }
    }
}

#[async_trait]
impl TransferMessagesRepo for TransferProviderRepoForSql {
    async fn get_all_transfer_messages(
        &self,
        limit: Option<u64>,
        page: Option<u64>,
    ) -> anyhow::Result<Vec<transfer_message::Model>, TransferProviderRepoErrors> {
        let transfer_message = transfer_message::Entity::find()
            .limit(limit.unwrap_or(100000))
            .offset(page.unwrap_or(0))
            .all(&self.db_connection)
            .await;
        match transfer_message {
            Ok(transfer_message) => Ok(transfer_message),
            Err(e) => Err(TransferProviderRepoErrors::ErrorFetchingProviderTransferMessage(e.into())),
        }
    }

    async fn get_all_transfer_messages_by_provider(
        &self,
        pid: Urn,
    ) -> anyhow::Result<Vec<transfer_message::Model>, TransferProviderRepoErrors> {
        let transfer_process = self
            .get_transfer_process_by_provider(pid.clone())
            .await
            .map_err(|e| TransferProviderRepoErrors::ErrorFetchingProviderTransferProcess(e.into()))?
            .ok_or(TransferProviderRepoErrors::ProviderTransferProcessNotFound)?;

        let transfer_message = transfer_message::Entity::find()
            .filter(transfer_message::Column::TransferProcessId.eq(pid.to_string()))
            .all(&self.db_connection)
            .await;
        match transfer_message {
            Ok(transfer_message) => Ok(transfer_message),
            Err(e) => Err(TransferProviderRepoErrors::ErrorFetchingProviderTransferMessage(e.into())),
        }
    }

    async fn get_transfer_message_by_id(
        &self,
        pid: Urn,
        mid: Urn,
    ) -> anyhow::Result<Option<transfer_message::Model>, TransferProviderRepoErrors> {
        let transfer_process = self
            .get_transfer_process_by_provider(pid.clone())
            .await
            .map_err(|e| TransferProviderRepoErrors::ErrorFetchingProviderTransferProcess(e.into()))?
            .ok_or(TransferProviderRepoErrors::ProviderTransferProcessNotFound)?;

        let transfer_message = transfer_message::Entity::find_by_id(mid.to_string()).one(&self.db_connection).await;
        match transfer_message {
            Ok(transfer_message) => Ok(transfer_message),
            Err(e) => Err(TransferProviderRepoErrors::ErrorFetchingProviderTransferMessage(e.into())),
        }
    }

    async fn put_transfer_message(
        &self,
        pid: Urn,
        edit_transfer_process: EditTransferMessageModel,
    ) -> anyhow::Result<Option<transfer_message::Model>, TransferProviderRepoErrors> {
        Ok(None)
    }

    async fn create_transfer_message(
        &self,
        pid: Urn,
        new_transfer_message: NewTransferMessageModel,
    ) -> anyhow::Result<transfer_message::Model, TransferProviderRepoErrors> {
        let pid = pid.to_string();

        let model = transfer_message::ActiveModel {
            id: ActiveValue::Set(get_urn(None).to_string()),
            transfer_process_id: ActiveValue::Set(pid),
            created_at: ActiveValue::Set(chrono::Utc::now().naive_utc()),
            message_type: ActiveValue::Set(new_transfer_message.message_type),
            from: ActiveValue::Set(new_transfer_message.from.to_string()),
            to: ActiveValue::Set(new_transfer_message.to.to_string()),
            content: ActiveValue::Set(new_transfer_message.content),
        };

        let model = transfer_message::Entity::insert(model).exec_with_returning(&self.db_connection).await;
        match model {
            Ok(model) => Ok(model),
            Err(e) => Err(TransferProviderRepoErrors::ErrorCreatingProviderTransferMessage(e.into())),
        }
    }

    async fn delete_transfer_message(&self, pid: Urn) -> anyhow::Result<(), TransferProviderRepoErrors> {
        let pid = pid.to_string();

        let transfer_message = transfer_message::Entity::delete_by_id(pid).exec(&self.db_connection).await;
        match transfer_message {
            Ok(delete_result) => match delete_result.rows_affected {
                0 => Err(TransferProviderRepoErrors::ProviderTransferMessageNotFound),
                _ => Ok(()),
            },
            Err(e) => Err(TransferProviderRepoErrors::ErrorDeletingProviderTransferMessage(e.into())),
        }
    }
}
