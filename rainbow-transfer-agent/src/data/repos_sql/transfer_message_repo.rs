use crate::data::entities::transfer_message;
use crate::data::entities::transfer_message::NewTransferMessageModel;
use crate::data::repo_traits::transfer_message_repo::{TransferMessageRepoErrors, TransferMessageRepoTrait};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QueryOrder, QuerySelect};
use urn::Urn;

pub struct TransferMessageRepoForSql {
    db_connection: DatabaseConnection,
}

impl TransferMessageRepoForSql {
    pub fn new(db_connection: DatabaseConnection) -> Self {
        Self { db_connection }
    }
}

#[async_trait::async_trait]
impl TransferMessageRepoTrait for TransferMessageRepoForSql {
    async fn get_all_transfer_messages(
        &self,
        limit: Option<u64>,
        page: Option<u64>,
    ) -> anyhow::Result<Vec<transfer_message::Model>, TransferMessageRepoErrors> {
        let messages = transfer_message::Entity::find()
            .limit(limit.unwrap_or(20))
            .offset(page.map(|p| p * limit.unwrap_or(20)).unwrap_or(0))
            .order_by_desc(transfer_message::Column::CreatedAt) // Default: los más nuevos primero
            .all(&self.db_connection)
            .await;

        match messages {
            Ok(messages) => Ok(messages),
            Err(e) => Err(TransferMessageRepoErrors::ErrorFetchingTransferMessage(
                e.into(),
            )),
        }
    }

    async fn get_messages_by_process_id(
        &self,
        process_id: &Urn,
    ) -> anyhow::Result<Vec<transfer_message::Model>, TransferMessageRepoErrors> {
        let pid = process_id.to_string();
        let messages = transfer_message::Entity::find()
            .filter(transfer_message::Column::TransferAgentProcessId.eq(pid))
            .order_by_asc(transfer_message::Column::CreatedAt)
            .all(&self.db_connection)
            .await;

        match messages {
            Ok(messages) => Ok(messages),
            Err(e) => Err(TransferMessageRepoErrors::ErrorFetchingTransferMessage(
                e.into(),
            )),
        }
    }

    async fn get_transfer_message_by_id(
        &self,
        id: &Urn,
    ) -> anyhow::Result<Option<transfer_message::Model>, TransferMessageRepoErrors> {
        let mid = id.to_string();
        let message = transfer_message::Entity::find_by_id(mid).one(&self.db_connection).await;
        match message {
            Ok(message) => Ok(message),
            Err(e) => Err(TransferMessageRepoErrors::ErrorFetchingTransferMessage(
                e.into(),
            )),
        }
    }

    async fn create_transfer_message(
        &self,
        new_model: &NewTransferMessageModel,
    ) -> anyhow::Result<transfer_message::Model, TransferMessageRepoErrors> {
        let model: transfer_message::ActiveModel = new_model.clone().into();
        let result = transfer_message::Entity::insert(model).exec_with_returning(&self.db_connection).await;
        match result {
            Ok(message) => Ok(message),
            Err(e) => Err(TransferMessageRepoErrors::ErrorCreatingTransferMessage(
                e.into(),
            )),
        }
    }

    async fn delete_transfer_message(&self, id: &Urn) -> anyhow::Result<(), TransferMessageRepoErrors> {
        let mid = id.to_string();
        let result = transfer_message::Entity::delete_by_id(mid).exec(&self.db_connection).await;

        match result {
            Ok(delete_result) => match delete_result.rows_affected {
                0 => Err(TransferMessageRepoErrors::TransferMessageNotFound),
                _ => Ok(()),
            },
            Err(e) => Err(TransferMessageRepoErrors::ErrorDeletingTransferMessage(
                e.into(),
            )),
        }
    }
}
